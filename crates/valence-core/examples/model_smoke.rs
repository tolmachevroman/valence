//! M0.1: load a local GGUF model on Metal, stream one card-style reply, and
//! report load time, time to first token, decode speed and peak memory.
//!
//! Run with:
//! `cargo run --release -p valence-core --example model_smoke -- ~/Models/valence/<file>.gguf`

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};
use libproc::pid_rusage::{RUsageInfoV4, pidrusage};
use mistralrs::{
    ChatCompletionChunkResponse, DeviceMapMetadata, GgufModelBuilder, RequestBuilder, Response,
    TextMessageRole,
};
use tokio::io::{AsyncWriteExt, stdout};

/// Context window the app will need: snippets plus a few-shot example fit well under this.
const MAX_MODEL_LEN: usize = 8192;
const MAX_NEW_TOKENS: usize = 256;

const SYSTEM: &str = "You write short, playful, exact fact cards about molecules. \
Use only facts from the numbered snippets. Reply with JSON only: \
{\"title\": string, \"body\": string of 2 to 4 sentences, \"sources\": [snippet ids]}.";

const USER: &str = "Card type: Weird fact\n\
Molecule: capsaicin\n\
[s1] Capsaicin is the active component of chili peppers, plants belonging to the genus Capsicum. \
It is an irritant for mammals, including humans, and produces a sensation of burning in any tissue with which it comes into contact. (Wikipedia, Capsaicin)\n\
[s2] Birds are generally not sensitive to capsaicin, because their TRPV1 receptor does not respond to it. \
Chili seeds consumed by birds pass through the digestive tract and can germinate later. (Wikipedia, Capsaicin)\n\
[s3] Capsaicin was first extracted in impure form in 1816 by Christian Friedrich Bucholz. (Wikipedia, History)";

/// Physical footprint in bytes: what Activity Monitor reports as memory, including Metal buffers.
fn phys_footprint() -> u64 {
    pidrusage::<RUsageInfoV4>(std::process::id() as i32)
        .map(|r| r.ri_phys_footprint)
        .unwrap_or(0)
}

fn gb(bytes: u64) -> f64 {
    bytes as f64 / 1e9
}

#[tokio::main]
async fn main() -> Result<()> {
    let path: PathBuf = std::env::args()
        .nth(1)
        .context("usage: model_smoke <path/to/model.gguf>")?
        .into();
    let dir = path.parent().context("model path has no directory")?;
    let file = path
        .file_name()
        .context("model path has no file name")?
        .to_string_lossy();

    let peak = Arc::new(AtomicU64::new(0));
    let sampler = {
        let peak = peak.clone();
        tokio::spawn(async move {
            loop {
                peak.fetch_max(phys_footprint(), Ordering::Relaxed);
                tokio::time::sleep(Duration::from_millis(50)).await;
            }
        })
    };

    let start = Instant::now();
    let model = GgufModelBuilder::new(dir.to_string_lossy(), vec![file.to_string()])
        .with_max_model_len(MAX_MODEL_LEN)
        // Auto mapping counts only free and file-backed pages as available, so on a
        // 16 GB Mac with a browser open it refuses a 2.5 GB model. Put every layer on Metal.
        .with_device_mapping(DeviceMapMetadata::dummy())
        // Paged attention (on by default for Metal) sizes its cache from the same estimate,
        // so memory use swings with whatever else is open. One short request at a time
        // doesn't need it; a plain KV cache keeps memory predictable.
        .without_paged_attn()
        .build()
        .await?;
    let load = start.elapsed();
    let after_load = phys_footprint();

    let request = RequestBuilder::new()
        .add_message(TextMessageRole::System, SYSTEM)
        .add_message(TextMessageRole::User, USER)
        .enable_thinking(false)
        .set_sampler_max_len(MAX_NEW_TOKENS);

    let sent = Instant::now();
    let mut first_token = None;
    let mut usage = None;
    let mut out = stdout();
    let mut stream = model.stream_chat_request(request).await?;
    while let Some(response) = stream.next().await {
        match response {
            Response::Chunk(ChatCompletionChunkResponse {
                choices, usage: u, ..
            }) => {
                if let Some(delta) = choices.first().map(|c| &c.delta) {
                    let text = delta
                        .content
                        .as_deref()
                        .or(delta.reasoning_content.as_deref());
                    if let Some(text) = text {
                        first_token.get_or_insert_with(|| sent.elapsed());
                        out.write_all(text.as_bytes()).await?;
                        out.flush().await?;
                    }
                }
                if u.is_some() {
                    usage = u;
                }
            }
            Response::InternalError(e) | Response::ValidationError(e) => bail!(e),
            Response::ModelError(msg, _) => bail!(msg),
            _ => {}
        }
    }
    let total = sent.elapsed();
    sampler.abort();

    println!("\n\n--- {file}");
    println!("load            {:>7.2} s", load.as_secs_f64());
    if let Some(t) = first_token {
        println!("first token     {:>7.2} s", t.as_secs_f64());
    }
    println!("full reply      {:>7.2} s", total.as_secs_f64());
    if let Some(u) = usage {
        println!(
            "tokens          {:>7} prompt, {} completion",
            u.prompt_tokens, u.completion_tokens
        );
        println!("prefill speed   {:>7.1} tok/s", u.avg_prompt_tok_per_sec);
        println!("decode speed    {:>7.1} tok/s", u.avg_compl_tok_per_sec);
    }
    println!("memory loaded   {:>7.2} GB", gb(after_load));
    println!(
        "memory peak     {:>7.2} GB",
        gb(peak.load(Ordering::Relaxed))
    );
    Ok(())
}
