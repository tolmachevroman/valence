//! Shared helpers for the M0 spike examples: model loading, streaming with
//! timings, and memory sampling.

use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};
use libproc::pid_rusage::{RUsageInfoV4, pidrusage};
use mistralrs::{
    ChatCompletionChunkResponse, DeviceMapMetadata, GgufModelBuilder, MemoryGoal, Model,
    PagedAttentionMetaBuilder, RequestBuilder, Response, Usage,
};

/// Context window the app will need: snippets plus a few-shot example fit well under this.
pub const MAX_MODEL_LEN: usize = 8192;

/// Physical footprint in bytes: what Activity Monitor reports as memory, including Metal buffers.
pub fn phys_footprint() -> u64 {
    pidrusage::<RUsageInfoV4>(std::process::id() as i32)
        .map(|r| r.ri_phys_footprint)
        .unwrap_or(0)
}

pub fn gb(bytes: u64) -> f64 {
    bytes as f64 / 1e9
}

/// Samples the process footprint every 50 ms and keeps the maximum.
pub struct PeakMemory {
    peak: Arc<AtomicU64>,
    task: tokio::task::JoinHandle<()>,
}

impl PeakMemory {
    pub fn start() -> Self {
        let peak = Arc::new(AtomicU64::new(0));
        let task = {
            let peak = peak.clone();
            tokio::spawn(async move {
                loop {
                    peak.fetch_max(phys_footprint(), Ordering::Relaxed);
                    tokio::time::sleep(Duration::from_millis(50)).await;
                }
            })
        };
        Self { peak, task }
    }

    pub fn peak(&self) -> u64 {
        self.peak.load(Ordering::Relaxed)
    }
}

impl Drop for PeakMemory {
    fn drop(&mut self) {
        self.task.abort();
    }
}

/// Load a local GGUF file on Metal. Returns the model and which KV cache it uses.
///
/// Prefers paged attention with a fixed cache of `MAX_MODEL_LEN` tokens: on a
/// 2,400-token prompt it peaked at 4.9 GB against 6.3 GB for the plain cache.
/// Its startup check still uses mistral.rs's conservative free-memory estimate
/// and can refuse (Qwen3.5 in M0.1), so fall back to the plain cache then.
/// `VALENCE_PLAIN_KV=1` forces the plain cache, for comparisons.
pub async fn load_model(path: &Path) -> Result<(Model, &'static str)> {
    let dir = path.parent().context("model path has no directory")?;
    let file = path
        .file_name()
        .context("model path has no file name")?
        .to_string_lossy();
    let base = || {
        GgufModelBuilder::new(dir.to_string_lossy(), vec![file.to_string()])
            .with_max_model_len(MAX_MODEL_LEN)
            // Auto mapping counts only free and file-backed pages as available, so on a
            // 16 GB Mac with a browser open it refuses a 2.5 GB model. Put every layer on Metal.
            .with_device_mapping(DeviceMapMetadata::dummy())
    };

    if std::env::var_os("VALENCE_PLAIN_KV").is_none() {
        let paged = PagedAttentionMetaBuilder::default()
            .with_memory_goal(MemoryGoal::KvTokens(MAX_MODEL_LEN))
            .build()?;
        match base().with_paged_attn(paged).build().await {
            Ok(model) => return Ok((model, "paged, fixed 8K tokens")),
            Err(e) => eprintln!("paged attention refused ({e}); using the plain KV cache"),
        }
    }
    Ok((base().without_paged_attn().build().await?, "plain"))
}

pub struct Generation {
    pub text: String,
    /// Seconds from sending the request to the first streamed text.
    pub first_token: Option<f64>,
    pub seconds: f64,
    pub usage: Option<Usage>,
}

/// Stream one request, calling `on_text` with each piece of text as it arrives.
pub async fn stream(
    model: &Model,
    request: RequestBuilder,
    mut on_text: impl FnMut(&str),
) -> Result<Generation> {
    let sent = Instant::now();
    let mut generation = Generation {
        text: String::new(),
        first_token: None,
        seconds: 0.0,
        usage: None,
    };
    let mut stream = model.stream_chat_request(request).await?;
    while let Some(response) = stream.next().await {
        match response {
            Response::Chunk(ChatCompletionChunkResponse { choices, usage, .. }) => {
                if let Some(delta) = choices.first().map(|c| &c.delta) {
                    let text = delta
                        .content
                        .as_deref()
                        .or(delta.reasoning_content.as_deref());
                    if let Some(text) = text {
                        generation
                            .first_token
                            .get_or_insert_with(|| sent.elapsed().as_secs_f64());
                        generation.text.push_str(text);
                        on_text(text);
                    }
                }
                if usage.is_some() {
                    generation.usage = usage;
                }
            }
            Response::InternalError(e) | Response::ValidationError(e) => bail!(e),
            Response::ModelError(msg, _) => bail!(msg),
            _ => {}
        }
    }
    generation.seconds = sent.elapsed().as_secs_f64();
    Ok(generation)
}
