//! Shared helpers for the M0 spike examples: model loading, streaming with
//! timings, and memory sampling.

use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};
use libproc::pid_rusage::{RUsageInfoV4, pidrusage};
use mistralrs::{
    ChatCompletionChunkResponse, GgufModelBuilder, MacosAvailableMemory, MemoryGoal, Model,
    PagedAttentionMetaBuilder, RequestBuilder, Response, Usage, set_macos_available_memory,
};

/// Context window and fixed KV cache size. The longest card prompt is about
/// 2,500 tokens plus up to 300 generated; 4K instead of 8K saves about 600 MB.
pub const MAX_MODEL_LEN: usize = 4096;

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
/// mistral.rs's default macOS memory estimate counts only fully reclaimable
/// pages, about 2 GB on a 16 GB Mac with a browser open, and every memory check
/// reads it: automatic device mapping, paged attention planning, per-request
/// admission. Our patched mistral.rs (M0.6c) can use the kernel's memory
/// pressure level instead, the system's own measure of usable memory.
///
/// Uses paged attention with a fixed cache of `MAX_MODEL_LEN` tokens. There is
/// no fallback to the plain cache: it peaked at 8.5 GB in M0.6b and stalled a
/// busy Mac, so a refusal is better. `VALENCE_MEMORY=reclaimable|inactive|pressure`
/// picks the estimate (default `pressure`); `VALENCE_PLAIN_KV=1` forces the plain cache.
pub async fn load_model(path: &Path) -> Result<(Model, &'static str)> {
    let policy = match std::env::var("VALENCE_MEMORY").as_deref() {
        Ok("reclaimable") => MacosAvailableMemory::Reclaimable,
        Ok("inactive") => MacosAvailableMemory::IncludeInactive,
        _ => MacosAvailableMemory::SystemPressure,
    };
    set_macos_available_memory(policy);
    let dir = path.parent().context("model path has no directory")?;
    let file = path
        .file_name()
        .context("model path has no file name")?
        .to_string_lossy();
    let base = || {
        GgufModelBuilder::new(dir.to_string_lossy(), vec![file.to_string()])
            .with_max_model_len(MAX_MODEL_LEN)
    };

    if std::env::var_os("VALENCE_PLAIN_KV").is_some() {
        return Ok((base().without_paged_attn().build().await?, "plain"));
    }
    let paged = PagedAttentionMetaBuilder::default()
        .with_memory_goal(MemoryGoal::KvTokens(MAX_MODEL_LEN))
        .build()?;
    let model = base()
        .with_paged_attn(paged)
        .build()
        .await
        .context("not enough free memory for the model and its 4K-token cache")?;
    Ok((model, "paged, fixed 4K tokens"))
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
