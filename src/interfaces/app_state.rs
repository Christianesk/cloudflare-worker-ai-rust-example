use std::sync::Arc;

use worker::{Ai, Fetcher};

#[derive(Clone)]
pub struct AppState {
    pub ai: Arc<Ai>,
    pub mcp: Fetcher,
    pub model_ai: String,
}
