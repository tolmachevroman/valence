//! What card generation needs from a language model: one constrained request
//! at a time, streamed. `model::LocalModel` is the real one; tests script one.

use std::future::Future;
use std::time::Duration;

use serde_json::Value;

use crate::prompt::Prompt;

/// One request: a prompt, the JSON schema the reply must follow, and sampling.
#[derive(Debug, Clone)]
pub struct Request {
    pub prompt: Prompt,
    pub schema: Value,
    pub max_tokens: usize,
    pub temperature: f64,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Generation {
    pub text: String,
    /// From sending the request to the first streamed text.
    pub first_token: Option<Duration>,
    pub elapsed: Duration,
    pub prompt_tokens: Option<usize>,
    pub completion_tokens: Option<usize>,
}

/// The model failed mid-request. Not a bad card: the card is abandoned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerateError(pub String);

impl std::fmt::Display for GenerateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "model error: {}", self.0)
    }
}

impl std::error::Error for GenerateError {}

pub trait Generate: Send + Sync {
    /// Runs `request`, calling `on_text` with each piece of text as it arrives.
    fn generate(
        &self,
        request: Request,
        on_text: &mut (dyn FnMut(&str) + Send),
    ) -> impl Future<Output = Result<Generation, GenerateError>> + Send;
}
