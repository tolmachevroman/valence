//! M0.1: load a local GGUF model on Metal, stream one card-style reply, and
//! report load time, time to first token, decode speed and peak memory.
//!
//! Run with:
//! `cargo run --release -p valence-core --example model_smoke -- ~/Models/valence/<file>.gguf`

mod common;

use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;

use anyhow::{Context, Result};
use mistralrs::{RequestBuilder, TextMessageRole};

use common::{PeakMemory, gb, load_model, phys_footprint, stream};

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

#[tokio::main]
async fn main() -> Result<()> {
    let path: PathBuf = std::env::args()
        .nth(1)
        .context("usage: model_smoke <path/to/model.gguf>")?
        .into();

    let memory = PeakMemory::start();
    let start = Instant::now();
    let (model, kv_cache) = load_model(&path).await?;
    eprintln!("KV cache: {kv_cache}");
    let load = start.elapsed();
    let after_load = phys_footprint();

    let request = RequestBuilder::new()
        .add_message(TextMessageRole::System, SYSTEM)
        .add_message(TextMessageRole::User, USER)
        .enable_thinking(false)
        .set_sampler_max_len(MAX_NEW_TOKENS);
    let reply = stream(&model, request, |text| {
        print!("{text}");
        let _ = std::io::stdout().flush();
    })
    .await?;

    println!("\n\n--- {}", path.display());
    println!("load            {:>7.2} s", load.as_secs_f64());
    if let Some(t) = reply.first_token {
        println!("first token     {t:>7.2} s");
    }
    println!("full reply      {:>7.2} s", reply.seconds);
    if let Some(u) = reply.usage {
        println!(
            "tokens          {:>7} prompt, {} completion",
            u.prompt_tokens, u.completion_tokens
        );
        println!("prefill speed   {:>7.1} tok/s", u.avg_prompt_tok_per_sec);
        println!("decode speed    {:>7.1} tok/s", u.avg_compl_tok_per_sec);
    }
    println!("memory loaded   {:>7.2} GB", gb(after_load));
    println!("memory peak     {:>7.2} GB", gb(memory.peak()));
    Ok(())
}
