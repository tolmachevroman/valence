//! The local model: a GGUF file run on Metal by mistral.rs (feature `model`).

use std::path::Path;
use std::time::Instant;

use mistralrs::{
    ChatCompletionChunkResponse, Constraint, GgufModelBuilder, MacosAvailableMemory, MemoryGoal,
    Model, PagedAttentionMetaBuilder, RequestBuilder, Response, TextMessageRole,
    set_macos_available_memory,
};

use crate::generate::{Generate, GenerateError, Generation, Request};

/// The default model (decision record 0001).
pub const MODEL_FILE: &str = "Qwen3-4B-Instruct-2507-Q4_K_M.gguf";

/// Context window and fixed KV cache size. The longest card prompt is about
/// 2,500 tokens plus up to 300 generated; 4K instead of 8K saves about 600 MB.
pub const MAX_MODEL_LEN: usize = 4096;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadError {
    /// No model file at the path.
    Missing,
    /// mistral.rs found too little free memory for the weights and the cache.
    NotEnoughMemory(String),
    Failed(String),
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Missing => write!(f, "model file not found"),
            Self::NotEnoughMemory(e) => write!(f, "not enough free memory: {e}"),
            Self::Failed(e) => write!(f, "model failed to load: {e}"),
        }
    }
}

impl std::error::Error for LoadError {}

/// How mistral.rs words a memory refusal, at load or while planning the cache.
fn is_memory_refusal(message: &str) -> bool {
    let m = message.to_lowercase();
    m.contains("insufficient memory") || m.contains("exceeds available")
}

pub struct LocalModel(Model);

impl LocalModel {
    /// Loads a GGUF file on Metal with a fixed paged KV cache of
    /// `MAX_MODEL_LEN` tokens.
    ///
    /// mistral.rs's default macOS memory estimate counts only fully reclaimable
    /// pages, about 2 GB on a 16 GB Mac with a browser open; our fork can use
    /// the kernel's memory pressure level instead, the system's own measure of
    /// usable memory (M0.6c). There is no fallback to the plain cache: it
    /// peaked at 8.5 GB in M0.6b and stalled a busy Mac, so refusing is better.
    pub async fn load(path: &Path) -> Result<Self, LoadError> {
        let (Some(dir), Some(file)) = (path.parent(), path.file_name()) else {
            return Err(LoadError::Missing);
        };
        if !path.is_file() {
            return Err(LoadError::Missing);
        }
        set_macos_available_memory(MacosAvailableMemory::SystemPressure);
        let failed = |e: anyhow::Error| {
            let message = format!("{e:#}");
            if is_memory_refusal(&message) {
                LoadError::NotEnoughMemory(message)
            } else {
                LoadError::Failed(message)
            }
        };
        let paged = PagedAttentionMetaBuilder::default()
            .with_memory_goal(MemoryGoal::KvTokens(MAX_MODEL_LEN))
            .build()
            .map_err(failed)?;
        let model = GgufModelBuilder::new(
            dir.to_string_lossy(),
            vec![file.to_string_lossy().into_owned()],
        )
        .with_max_model_len(MAX_MODEL_LEN)
        .with_paged_attn(paged)
        .build()
        .await
        .map_err(failed)?;
        Ok(Self(model))
    }
}

impl Generate for LocalModel {
    async fn generate(
        &self,
        request: Request,
        on_text: &mut (dyn FnMut(&str) + Send),
    ) -> Result<Generation, GenerateError> {
        let sent = Instant::now();
        let builder = RequestBuilder::new()
            .add_message(TextMessageRole::System, request.prompt.system)
            .add_message(TextMessageRole::User, request.prompt.user)
            .enable_thinking(false)
            .set_constraint(Constraint::JsonSchema(request.schema))
            .set_sampler_max_len(request.max_tokens)
            .set_sampler_temperature(request.temperature);
        let mut stream = self
            .0
            .stream_chat_request(builder)
            .await
            .map_err(|e| GenerateError(format!("{e:#}")))?;
        let mut g = Generation::default();
        while let Some(response) = stream.next().await {
            match response {
                Response::Chunk(ChatCompletionChunkResponse { choices, usage, .. }) => {
                    if let Some(text) = choices.first().and_then(|c| c.delta.content.as_deref()) {
                        g.first_token.get_or_insert_with(|| sent.elapsed());
                        g.text.push_str(text);
                        on_text(text);
                    }
                    if let Some(u) = usage {
                        g.prompt_tokens = Some(u.prompt_tokens);
                        g.completion_tokens = Some(u.completion_tokens);
                    }
                }
                Response::InternalError(e) | Response::ValidationError(e) => {
                    return Err(GenerateError(e.to_string()));
                }
                Response::ModelError(message, _) => return Err(GenerateError(message)),
                _ => {}
            }
        }
        g.elapsed = sent.elapsed();
        Ok(g)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_refusals_are_recognized() {
        assert!(is_memory_refusal(
            "Insufficient memory for a usable KV cache after weights and runtime reservations."
        ));
        assert!(is_memory_refusal(
            "Model weights (2.5 GB) exceeds available VRAM"
        ));
        assert!(!is_memory_refusal("unsupported GGUF architecture"));
    }

    #[tokio::test]
    async fn a_missing_file_is_reported_as_missing() {
        let r = LocalModel::load(Path::new("/nonexistent/model.gguf")).await;
        assert_eq!(r.err(), Some(LoadError::Missing));
    }

    /// With the real model:
    /// `VALENCE_MODEL=path/to/model.gguf cargo test -p valence-core --features model -- --ignored live_model`
    #[tokio::test]
    #[ignore = "needs the model file"]
    async fn live_model_streams_constrained_json() {
        let path = std::env::var_os("VALENCE_MODEL").expect("VALENCE_MODEL");
        let model = LocalModel::load(Path::new(&path)).await.unwrap();
        let request = Request {
            prompt: crate::prompt::Prompt {
                system: "Answer in JSON.".into(),
                user: "Which element has the symbol O?".into(),
            },
            schema: serde_json::json!({
                "type": "object",
                "properties": {"element": {"type": "string"}},
                "required": ["element"],
                "additionalProperties": false
            }),
            max_tokens: 30,
            temperature: 0.0,
        };
        let mut pieces = 0;
        let g = model.generate(request, &mut |_| pieces += 1).await.unwrap();
        let v: serde_json::Value = serde_json::from_str(&g.text).unwrap();
        assert_eq!(v["element"], "Oxygen", "{}", g.text);
        assert!(pieces > 1, "streamed");
        assert!(g.first_token.is_some() && g.completion_tokens.is_some());
    }
}
