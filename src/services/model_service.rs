use serde_json::{json, Value};
use worker::js_sys::{JsString, Promise};
use worker::send::SendFuture;
use worker::wasm_bindgen::{JsCast, JsValue};
use worker::wasm_bindgen_futures::JsFuture;
use worker::*;

use crate::constants::constants::GET_USERS_BY_COUNTRY;
use crate::interfaces::{api_request::APIRequest, app_state::AppState};

pub async fn manage_message(mut req: Request, state: AppState) -> Result<Response> {
    let request: APIRequest = req.json::<APIRequest>().await?;

    let response: Value = call_ai(
        &state,
        json!({
            "messages": [
                {
                    "role": "system",
                    "content": "Use tools when you need external data."
                },
                {
                    "role": "user",
                    "content": request.message
                }
            ],
            "tools": [
                {
                    "type": "function",
                    "function": {
                        "name": GET_USERS_BY_COUNTRY,
                        "description": "Get users filtered by country.",
                        "parameters": {
                            "type": "object",
                            "properties": {
                                "country": {
                                    "type": "string",
                                    "description": "Country name, e.g. 'Colombia'"
                                }
                            },
                            "required": ["country"]
                        }
                    }
                }
            ]
        }),
    )
    .await?;

    if let Some(tool_calls) = response["choices"][0]["message"]["tool_calls"].as_array() {
        if let Some(call) = tool_calls.first() {
            let name = call["function"]["name"].as_str().unwrap_or("");
            let args_str = call["function"]["arguments"].as_str().unwrap_or("{}");
            let args: Value = serde_json::from_str(args_str).unwrap_or(json!({}));

            if name == GET_USERS_BY_COUNTRY {
                let country = args["country"].as_str().unwrap_or("");

                let mcp_body = json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "method": "tools/call",
                    "params": {
                        "name": GET_USERS_BY_COUNTRY,
                        "arguments": {
                            "country": country
                        }
                    }
                });

                let mut init: RequestInit = RequestInit::new();
                init.method = Method::Post;
                init.body = Some(JsValue::from_str(&mcp_body.to_string()));

                let mut sub_req: Request = Request::new_with_init("https://internal/mcp", &init)?;
                sub_req
                    .headers_mut()?
                    .set("Content-Type", "application/json")?;
                let mut mcp_res: Response = state.mcp.fetch_request(sub_req).await?;
                let mcp_text: String = mcp_res.text().await?;

                let mcp_data: Value = serde_json::from_str(&mcp_text)?;
                let users_text: &str = mcp_data["result"]["content"][0]["text"]
                    .as_str()
                    .unwrap_or("[]");
                let users: Value = serde_json::from_str(users_text).unwrap_or(json!([]));

                let users_str: String = serde_json::to_string(&users).unwrap_or_default();
                let final_response:Value = call_ai(
                    &state,
                    json!({
                        "messages": [
                            {
                                "role": "system",
                                "content": format!("The data obtained from the service is: {}. Here are the users we found for the requested country.", users_str)
                            },
                            {
                                "role": "user",
                                "content": request.message
                            }
                        ]
                    }),
                )
                .await?;

                let response_text: &str = final_response["choices"][0]["message"]["content"]
                    .as_str()
                    .unwrap_or("No Response");

                return Response::ok(response_text);
            }
        }
    }

    Response::ok(response["response"].to_string())
}

async fn call_ai(state: &AppState, input: Value) -> Result<Value> {
    let json_str: String = serde_json::to_string(&input).unwrap();

    let js_input: JsValue = js_sys::JSON::parse(&json_str)?;
    let ai_sys: &worker_sys::Ai = state.ai.as_ref().unchecked_ref();
    let promise: Promise = ai_sys.run(&state.model_ai, js_input);
    let output: JsValue = SendFuture::new(JsFuture::from(promise))
        .await
        .map_err(Error::from)?;

    let output_js: JsString = js_sys::JSON::stringify(&output)?;
    let output_str: String = output_js
        .as_string()
        .ok_or_else(|| Error::RustError("Failed to stringify AI output".into()))?;

    Ok(serde_json::from_str(&output_str)?)
}
