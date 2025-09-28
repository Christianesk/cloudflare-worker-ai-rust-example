use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct APIResponse {
    pub response: String,
    usage: Usage,
}
