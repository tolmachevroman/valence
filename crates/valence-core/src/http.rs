//! The one HTTP client every data source shares: a 15 s timeout, an honest
//! User-Agent, PubChem's 5 requests/s limit, and one retry when the server is
//! busy.

use std::fmt;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;

use governor::{DefaultDirectRateLimiter, Quota, RateLimiter};
use reqwest::{Client, StatusCode, header};

pub const USER_AGENT: &str = concat!(
    "Valence/",
    env!("CARGO_PKG_VERSION"),
    " (+",
    env!("CARGO_PKG_REPOSITORY"),
    ")"
);

/// PubChem's published limit: no more than 5 requests per second.
const PUBCHEM_PER_SECOND: u32 = 5;

/// Who a request goes to, which decides its rate limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Host {
    PubChem,
    Wikipedia,
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub timeout: Duration,
    /// Wait before the retry when the server doesn't send `Retry-After`.
    pub backoff: Duration,
    /// Longest `Retry-After` we honor; a longer one fails the request instead.
    pub max_backoff: Duration,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(15),
            backoff: Duration::from_secs(1),
            max_backoff: Duration::from_secs(10),
        }
    }
}

#[derive(Debug)]
pub enum HttpError {
    /// The server answered with an error status, after any retry.
    Status(StatusCode),
    /// No usable answer: offline, timed out, or the connection broke.
    Network(reqwest::Error),
}

impl HttpError {
    pub fn is_not_found(&self) -> bool {
        matches!(self, Self::Status(StatusCode::NOT_FOUND))
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Status(s) => write!(f, "server answered {s}"),
            Self::Network(e) if e.is_timeout() => write!(f, "request timed out"),
            Self::Network(e) => write!(f, "network error: {e}"),
        }
    }
}

impl std::error::Error for HttpError {}

/// Cheap to clone; clones share the connection pool and the rate limit.
#[derive(Clone)]
pub struct Http {
    client: Client,
    pubchem: Arc<DefaultDirectRateLimiter>,
    settings: Settings,
}

impl Http {
    pub fn new() -> Self {
        Self::with_settings(Settings::default())
    }

    pub fn with_settings(settings: Settings) -> Self {
        let client = Client::builder()
            .user_agent(USER_AGENT)
            .timeout(settings.timeout)
            .build()
            .expect("TLS backend is available");
        let quota = Quota::per_second(NonZeroU32::new(PUBCHEM_PER_SECOND).unwrap());
        Self {
            client,
            pubchem: Arc::new(RateLimiter::direct(quota)),
            settings,
        }
    }

    /// GET `url` and return the body. Retries once on 429, 5xx or a network
    /// error, waiting for `Retry-After` when the server sends one.
    pub async fn get(&self, host: Host, url: &str) -> Result<Vec<u8>, HttpError> {
        match self.attempt(host, url).await {
            Err(Failure::Retry(wait, _)) => {
                tokio::time::sleep(wait).await;
                self.attempt(host, url).await.map_err(Failure::into_error)
            }
            other => other.map_err(Failure::into_error),
        }
    }

    pub async fn get_text(&self, host: Host, url: &str) -> Result<String, HttpError> {
        let body = self.get(host, url).await?;
        Ok(String::from_utf8_lossy(&body).into_owned())
    }

    async fn attempt(&self, host: Host, url: &str) -> Result<Vec<u8>, Failure> {
        if host == Host::PubChem {
            self.pubchem.until_ready().await;
        }
        let response = match self.client.get(url).send().await {
            Ok(r) => r,
            Err(e) => return Err(Failure::Retry(self.settings.backoff, HttpError::Network(e))),
        };
        let status = response.status();
        if status.is_success() {
            return match response.bytes().await {
                Ok(b) => Ok(b.to_vec()),
                Err(e) => Err(Failure::Retry(self.settings.backoff, HttpError::Network(e))),
            };
        }
        if status != StatusCode::TOO_MANY_REQUESTS && !status.is_server_error() {
            return Err(Failure::Final(HttpError::Status(status)));
        }
        let retry_after = response
            .headers()
            .get(header::RETRY_AFTER)
            .and_then(|v| v.to_str().ok()?.trim().parse::<u64>().ok())
            .map(Duration::from_secs);
        match retry_after {
            Some(wait) if wait > self.settings.max_backoff => {
                Err(Failure::Final(HttpError::Status(status)))
            }
            wait => Err(Failure::Retry(
                wait.unwrap_or(self.settings.backoff),
                HttpError::Status(status),
            )),
        }
    }
}

impl Default for Http {
    fn default() -> Self {
        Self::new()
    }
}

enum Failure {
    /// Worth one more try after waiting this long.
    Retry(Duration, HttpError),
    Final(HttpError),
}

impl Failure {
    fn into_error(self) -> HttpError {
        match self {
            Self::Retry(_, e) | Self::Final(e) => e,
        }
    }
}

/// A local HTTP server for tests: no network, no mocking library.
#[cfg(test)]
pub(crate) mod test_server {
    use std::sync::{Arc, Mutex};

    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    /// Request heads in the order they arrived.
    pub(crate) type Log = Arc<Mutex<Vec<String>>>;

    /// Serve every request with `respond(index, path)`, which returns a full
    /// HTTP response. Returns the base URL (`http://127.0.0.1:port`) and the log.
    pub(crate) async fn serve(
        respond: impl Fn(usize, &str) -> String + Send + 'static,
    ) -> (String, Log) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        let seen = Log::default();
        let log = seen.clone();
        tokio::spawn(async move {
            loop {
                let (mut socket, _) = listener.accept().await.unwrap();
                let mut buf = vec![0; 8192];
                let n = socket.read(&mut buf).await.unwrap();
                let head = String::from_utf8_lossy(&buf[..n]).into_owned();
                let path = head.split(' ').nth(1).unwrap_or("").to_string();
                let i = {
                    let mut log = log.lock().unwrap();
                    log.push(head);
                    log.len() - 1
                };
                let reply = respond(i, &path);
                socket.write_all(reply.as_bytes()).await.unwrap();
            }
        });
        (base, seen)
    }

    pub(crate) fn response(status: &str, body: &str) -> String {
        format!(
            "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use tokio::net::TcpListener;

    use super::test_server::{Log, serve as serve_fn};
    use super::*;

    /// Answer each request with the next scripted response; the last repeats.
    async fn serve(script: Vec<&'static str>) -> (String, Log) {
        let (base, log) = serve_fn(move |i, _| script[i.min(script.len() - 1)].to_string()).await;
        (format!("{base}/x"), log)
    }

    const OK: &str = "HTTP/1.1 200 OK\r\nContent-Length: 5\r\nConnection: close\r\n\r\nhello";
    const BUSY: &str =
        "HTTP/1.1 503 Service Unavailable\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
    const BUSY_LONG: &str = "HTTP/1.1 503 Service Unavailable\r\nRetry-After: 60\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
    const MISSING: &str =
        "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";

    fn http() -> Http {
        Http::with_settings(Settings {
            backoff: Duration::from_millis(10),
            ..Settings::default()
        })
    }

    #[tokio::test]
    async fn sends_user_agent_and_returns_body() {
        let (url, seen) = serve(vec![OK]).await;
        assert_eq!(
            http().get_text(Host::Wikipedia, &url).await.unwrap(),
            "hello"
        );
        let head = seen.lock().unwrap()[0].to_lowercase();
        assert!(head.contains(&format!("user-agent: {}", USER_AGENT.to_lowercase())));
        assert!(USER_AGENT.starts_with("Valence/0."));
        assert!(USER_AGENT.ends_with("(+https://github.com/tolmachevroman/valence)"));
    }

    #[tokio::test]
    async fn retries_once_when_busy() {
        let (url, seen) = serve(vec![BUSY, OK]).await;
        assert_eq!(http().get_text(Host::PubChem, &url).await.unwrap(), "hello");
        assert_eq!(seen.lock().unwrap().len(), 2);

        let (url, seen) = serve(vec![BUSY]).await;
        let err = http().get(Host::PubChem, &url).await.unwrap_err();
        assert!(matches!(
            err,
            HttpError::Status(StatusCode::SERVICE_UNAVAILABLE)
        ));
        assert_eq!(seen.lock().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn no_retry_on_not_found_or_a_long_retry_after() {
        let (url, seen) = serve(vec![MISSING]).await;
        assert!(
            http()
                .get(Host::PubChem, &url)
                .await
                .unwrap_err()
                .is_not_found()
        );
        assert_eq!(seen.lock().unwrap().len(), 1);

        let (url, seen) = serve(vec![BUSY_LONG, OK]).await;
        assert!(http().get(Host::PubChem, &url).await.is_err());
        assert_eq!(seen.lock().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn network_errors_are_retried_then_reported() {
        // Nothing listens on this port once the listener is dropped.
        let addr = TcpListener::bind("127.0.0.1:0")
            .await
            .unwrap()
            .local_addr()
            .unwrap();
        let err = http()
            .get(Host::Wikipedia, &format!("http://{addr}/"))
            .await
            .unwrap_err();
        assert!(matches!(err, HttpError::Network(_)));
    }

    #[tokio::test]
    async fn pubchem_is_held_to_five_per_second() {
        let (url, _) = serve(vec![OK]).await;
        let http = http();
        let start = Instant::now();
        for _ in 0..10 {
            http.get(Host::PubChem, &url).await.unwrap();
        }
        // A burst of 5, then one every 200 ms.
        assert!(
            start.elapsed() >= Duration::from_millis(900),
            "{:?}",
            start.elapsed()
        );

        let start = Instant::now();
        for _ in 0..10 {
            http.get(Host::Wikipedia, &url).await.unwrap();
        }
        assert!(start.elapsed() < Duration::from_millis(500));
    }
}
