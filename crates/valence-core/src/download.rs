//! The model download (M4.2): one file from Hugging Face at a pinned
//! revision, resumed after an interruption and checked by sha256 before it
//! takes the model's place.

use std::fmt;
use std::path::{Path, PathBuf};
use std::time::Duration;

use reqwest::{Client, StatusCode, header};
use sha2::{Digest, Sha256};
use tokio::fs::{self, File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::http::USER_AGENT;

/// One downloadable model file.
#[derive(Debug, Clone, Copy)]
pub struct Pinned<'a> {
    pub url: &'a str,
    /// Lowercase hex.
    pub sha256: &'a str,
    pub size: u64,
}

/// The default model at the revision of decision record 0001.
pub const DEFAULT_MODEL: Pinned<'static> = Pinned {
    url: "https://huggingface.co/unsloth/Qwen3-4B-Instruct-2507-GGUF/resolve/a06e946bb6b655725eafa393f4a9745d460374c9/Qwen3-4B-Instruct-2507-Q4_K_M.gguf",
    sha256: "3605803b982cb64aead44f6c1b2ae36e3acdb41d8e46c8a94c6533bc4c67e597",
    size: 2_497_281_120,
};

/// Progress is reported about this many times over a whole download.
const REPORTS: u64 = 1000;

#[derive(Debug)]
pub enum DownloadError {
    /// Offline, timed out, or the connection broke; the part kept so far is
    /// resumed next time.
    Network(reqwest::Error),
    Status(StatusCode),
    Disk(std::io::Error),
    /// The finished file isn't the pinned one; it was deleted.
    Mismatch,
}

impl fmt::Display for DownloadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Network(e) if e.is_timeout() => write!(f, "the download timed out"),
            Self::Network(_) => write!(f, "the connection was lost"),
            Self::Status(s) => write!(f, "Hugging Face answered {s}"),
            Self::Disk(e) => write!(f, "could not write the file: {e}"),
            Self::Mismatch => write!(f, "the downloaded file did not match its checksum"),
        }
    }
}

impl std::error::Error for DownloadError {}

impl From<std::io::Error> for DownloadError {
    fn from(e: std::io::Error) -> Self {
        Self::Disk(e)
    }
}

impl From<reqwest::Error> for DownloadError {
    fn from(e: reqwest::Error) -> Self {
        Self::Network(e)
    }
}

/// The partial file next to `dest`, kept between attempts.
pub fn part_path(dest: &Path) -> PathBuf {
    let mut name = dest.file_name().unwrap_or_default().to_os_string();
    name.push(".part");
    dest.with_file_name(name)
}

/// A client for large files: no overall timeout, only for connecting and for
/// each read.
pub fn client() -> Client {
    Client::builder()
        .user_agent(USER_AGENT)
        .connect_timeout(Duration::from_secs(15))
        .read_timeout(Duration::from_secs(60))
        .build()
        .expect("TLS backend is available")
}

/// Download `file` to `dest`, resuming a partial file from an earlier
/// attempt. `on_progress(done, total)` is called as bytes arrive, first with
/// what an earlier attempt kept. `dest` appears only once the size and
/// sha256 match.
pub async fn download(
    client: &Client,
    file: Pinned<'_>,
    dest: &Path,
    on_progress: &mut (dyn FnMut(u64, u64) + Send),
) -> Result<(), DownloadError> {
    if let Some(dir) = dest.parent() {
        fs::create_dir_all(dir).await?;
    }
    let part = part_path(dest);
    let mut hasher = Sha256::new();
    let mut done = match fs::metadata(&part).await {
        Ok(m) if m.len() <= file.size => hash_file(&part, &mut hasher).await?,
        _ => 0,
    };
    on_progress(done, file.size);

    let mut request = client.get(file.url);
    if done > 0 {
        request = request.header(header::RANGE, format!("bytes={done}-"));
    }
    let mut response = request.send().await?;
    let mut out = match response.status() {
        StatusCode::PARTIAL_CONTENT if done > 0 => {
            OpenOptions::new().append(true).open(&part).await?
        }
        // Everything there is, or the server ignored the range: start over.
        s if s.is_success() => {
            done = 0;
            hasher = Sha256::new();
            File::create(&part).await?
        }
        // The part was the whole file already.
        StatusCode::RANGE_NOT_SATISFIABLE if done == file.size => {
            OpenOptions::new().append(true).open(&part).await?
        }
        s => return Err(DownloadError::Status(s)),
    };

    let step = (file.size / REPORTS).max(1);
    let mut reported = done;
    let received = loop {
        let chunk = match response.chunk().await {
            Ok(Some(chunk)) => chunk,
            Ok(None) => break Ok(()),
            Err(e) => break Err(e),
        };
        if done + chunk.len() as u64 > file.size {
            break Ok(());
        }
        out.write_all(&chunk).await?;
        hasher.update(&chunk);
        done += chunk.len() as u64;
        if done - reported >= step {
            reported = done;
            on_progress(done, file.size);
        }
    };
    // Flushed even when the connection broke, so the part on disk is what
    // was received and resumes cleanly.
    out.flush().await?;
    drop(out);
    received?;

    let digest: String = hasher
        .finalize()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    if done != file.size || digest != file.sha256 {
        fs::remove_file(&part).await?;
        return Err(DownloadError::Mismatch);
    }
    on_progress(done, file.size);
    fs::rename(&part, dest).await?;
    Ok(())
}

/// Feed a file into `hasher`; returns its length.
async fn hash_file(path: &Path, hasher: &mut Sha256) -> std::io::Result<u64> {
    let mut file = File::open(path).await?;
    let mut buf = vec![0; 1 << 20];
    let mut len = 0;
    loop {
        let n = file.read(&mut buf).await?;
        if n == 0 {
            return Ok(len);
        }
        hasher.update(&buf[..n]);
        len += n as u64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::test_server::{response, serve};

    const BODY: &[u8] = b"pretend these are 2.5 GB of model weights";

    fn sha(bytes: &[u8]) -> String {
        Sha256::digest(bytes)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect()
    }

    fn dir() -> PathBuf {
        let d = std::env::temp_dir().join(format!(
            "valence-download-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::remove_dir_all(&d);
        d
    }

    async fn run(url: &str, sha256: &str, dest: &Path) -> (Result<(), DownloadError>, Vec<u64>) {
        let mut seen = Vec::new();
        let file = Pinned {
            url,
            sha256,
            size: BODY.len() as u64,
        };
        let r = download(&client(), file, dest, &mut |done, _| seen.push(done)).await;
        (r, seen)
    }

    #[tokio::test]
    async fn downloads_and_checks_the_file() {
        let (base, _) = serve(|_, _| response("200 OK", BODY)).await;
        let dest = dir().join("model.gguf");
        let (r, seen) = run(&format!("{base}/m"), &sha(BODY), &dest).await;
        r.unwrap();
        assert_eq!(std::fs::read(&dest).unwrap(), BODY);
        assert!(!part_path(&dest).exists());
        assert_eq!(seen.first(), Some(&0));
        assert_eq!(seen.last(), Some(&(BODY.len() as u64)));
    }

    #[tokio::test]
    async fn resumes_after_a_broken_connection() {
        let half = BODY.len() / 2;
        let (base, log) = serve(move |i, _| {
            if i == 0 {
                // Promises the whole file, sends half, closes.
                let mut r = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    BODY.len()
                )
                .into_bytes();
                r.extend_from_slice(&BODY[..half]);
                r
            } else {
                response("206 Partial Content", &BODY[half..])
            }
        })
        .await;
        let dest = dir().join("model.gguf");
        let url = format!("{base}/m");
        let (r, _) = run(&url, &sha(BODY), &dest).await;
        assert!(matches!(r, Err(DownloadError::Network(_))), "{r:?}");
        assert_eq!(std::fs::read(part_path(&dest)).unwrap(), &BODY[..half]);

        let (r, seen) = run(&url, &sha(BODY), &dest).await;
        r.unwrap();
        assert_eq!(std::fs::read(&dest).unwrap(), BODY);
        assert_eq!(seen.first(), Some(&(half as u64)));
        let second = log.lock().unwrap()[1].to_lowercase();
        assert!(
            second.contains(&format!("range: bytes={half}-")),
            "{second}"
        );
    }

    #[tokio::test]
    async fn starts_over_when_the_server_ignores_the_range() {
        let (base, _) = serve(|_, _| response("200 OK", BODY)).await;
        let dest = dir().join("model.gguf");
        std::fs::create_dir_all(dest.parent().unwrap()).unwrap();
        std::fs::write(part_path(&dest), b"pretend").unwrap();
        let (r, _) = run(&format!("{base}/m"), &sha(BODY), &dest).await;
        r.unwrap();
        assert_eq!(std::fs::read(&dest).unwrap(), BODY);
    }

    #[tokio::test]
    async fn a_file_that_does_not_match_is_deleted() {
        let (base, _) = serve(|_, _| response("200 OK", BODY)).await;
        let dest = dir().join("model.gguf");
        let (r, _) = run(&format!("{base}/m"), &sha(b"other"), &dest).await;
        assert!(matches!(r, Err(DownloadError::Mismatch)), "{r:?}");
        assert!(!dest.exists() && !part_path(&dest).exists());
    }

    /// Against Hugging Face: seeds a part from the model at `VALENCE_MODEL`
    /// minus its last 50 MB, resumes, and checks the whole file. Run with
    /// `cargo test -p valence-core -- --ignored resumes_the_real_model`.
    #[tokio::test]
    #[ignore = "downloads 50 MB from Hugging Face"]
    async fn resumes_the_real_model() {
        let model = PathBuf::from(std::env::var_os("VALENCE_MODEL").expect("VALENCE_MODEL"));
        let dest = dir().join("model.gguf");
        std::fs::create_dir_all(dest.parent().unwrap()).unwrap();
        let kept = DEFAULT_MODEL.size - 50_000_000;
        let mut from = std::fs::File::open(model).unwrap();
        let mut part = std::fs::File::create(part_path(&dest)).unwrap();
        std::io::copy(&mut std::io::Read::take(&mut from, kept), &mut part).unwrap();
        drop(part);

        let mut first = None;
        download(&client(), DEFAULT_MODEL, &dest, &mut |done, _| {
            first.get_or_insert(done);
        })
        .await
        .unwrap();
        assert_eq!(first, Some(kept));
        assert_eq!(std::fs::metadata(&dest).unwrap().len(), DEFAULT_MODEL.size);
        std::fs::remove_dir_all(dest.parent().unwrap()).unwrap();
    }

    #[tokio::test]
    async fn reports_an_error_status() {
        let (base, _) = serve(|_, _| response("404 Not Found", "")).await;
        let dest = dir().join("model.gguf");
        let (r, _) = run(&format!("{base}/m"), &sha(BODY), &dest).await;
        assert!(
            matches!(r, Err(DownloadError::Status(StatusCode::NOT_FOUND))),
            "{r:?}"
        );
    }
}
