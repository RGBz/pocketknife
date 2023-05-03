use crate::error::AnyError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CompletionUsage {
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
    pub total_tokens: i64,
}

#[derive(Serialize, Deserialize)]
pub struct CompletionMessageResponse {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct CompletionChoice {
    pub message: CompletionMessageResponse,
    pub finish_reason: String,
    pub index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct CompletionResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub usage: CompletionUsage,
    pub choices: Vec<CompletionChoice>,
}

impl TryFrom<&str> for CompletionResponse {
    type Error = serde_json::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value)
    }
}

#[derive(Serialize, Deserialize)]
pub enum CompletionModel {
    #[serde(rename = "gpt-3.5-turbo")]
    Gpt3Turbo,
    #[serde(rename = "gpt-4")]
    Gpt4,
}

impl CompletionModel {
    pub fn new(model: &str) -> Self {
        match model {
            "gpt-3.5-turbo" => Self::Gpt3Turbo,
            "gpt-4" => Self::Gpt4,
            _ => panic!("Invalid model"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CompletionMessagePayload {
    pub role: String,
    pub content: String,
}

impl CompletionMessagePayload {
    pub fn new(content: &str) -> Self {
        Self {
            role: "user".to_owned(),
            content: content.to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CompletionPayload {
    pub model: CompletionModel,
    pub messages: Vec<CompletionMessagePayload>,
}

impl CompletionPayload {
    pub fn new(model: &str, content: &str) -> Self {
        Self {
            model: CompletionModel::new(model),
            messages: vec![CompletionMessagePayload::new(content)],
        }
    }
}

impl TryFrom<&CompletionPayload> for String {
    type Error = serde_json::Error;

    fn try_from(value: &CompletionPayload) -> Result<Self, Self::Error> {
        serde_json::to_string(value)
    }
}

pub struct ChatCompletionPostArgs<'a> {
    pub message: &'a str,
    pub api_key: &'a str,
    pub model_name: &'a str,
}

pub fn post(args: ChatCompletionPostArgs) -> Result<String, AnyError> {
    let client = reqwest::blocking::Client::new();
    let payload = CompletionPayload::new(&args.model_name, args.message);
    let bearer_token = format!("Bearer {}", args.api_key);

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", bearer_token)
        .json(&payload)
        .send()?;

    let response = response.json::<CompletionResponse>()?;
    let choice = response
        .choices
        .last()
        .ok_or("Error: no text found in response from ChatGPT API")?;

    Ok(choice.message.content.to_owned())
}
