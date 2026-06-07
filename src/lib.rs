mod constants;
mod interfaces;
mod services;

use crate::{interfaces::app_state::AppState, services::model_service::manage_message};
use std::sync::Arc;
use worker::*;

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    let ai: Ai = env.ai("AI")?;
    let mcp: Fetcher = env.service("MCP_USERS")?;
    let model_ai: String = env.var("MODEL_AI")?.to_string();
    let state: AppState = AppState {
        ai: Arc::new(ai),
        mcp,
        model_ai,
    };

    Router::with_data(state)
        .post_async("/", |req, ctx| async move {
            manage_message(req, ctx.data.clone()).await
        })
        .run(req, env)
        .await
}
