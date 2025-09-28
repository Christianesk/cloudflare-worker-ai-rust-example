use serde::Serialize;

#[derive(Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct ModelInput {
    pub messages: Vec<Message>,
}
