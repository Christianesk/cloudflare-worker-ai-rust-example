use worker::*;

use crate::interfaces::{
    api_request::APIRequest,
    api_response::APIResponse,
    app_state::AppState,
    model_input::{Message, ModelInput},
};

pub async fn manage_message(mut req: Request, state: AppState) -> Result<Response> {
    let request: APIRequest = req.json::<APIRequest>().await?;

    let input: ModelInput = ModelInput {
        messages: vec![Message {
            role: "user".to_string(),
            content: request.message,
        }],
    };

    let output: APIResponse = state
        .ai
        .run("@cf/meta/llama-3.2-1b-instruct", input)
        .await?;

    Response::from_json(&output)
}
