use std::sync::Arc;

use worker::Ai;

#[derive(Clone)]
pub struct AppState {
    pub ai: Arc<Ai>,
}
