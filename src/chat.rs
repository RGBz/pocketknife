use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::error::AnyError;

const ENDPOINT: &str = "https://api.openai.com/v1/chat/completions";

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CompletionUsage {
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
    pub total_tokens: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CompletionMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CompletionChoice {
    pub message: CompletionMessage,
    pub finish_reason: String,
    pub index: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CompletionSuccessResponse {
    pub id: Option<String>,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub usage: CompletionUsage,
    pub choices: Vec<CompletionChoice>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CompletionError {
    pub message: String,
    pub r#type: String,
    pub param: String,
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CompletionErrorResponse {
    pub error: CompletionError,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum CompletionResponse {
    Success(CompletionSuccessResponse),
    Error(CompletionErrorResponse),
}

impl TryFrom<&str> for CompletionResponse {
    type Error = serde_json::Error;

    fn try_from(value: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(value)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
    pub timeout: &'a u64,
    pub debug: &'a bool,
}

pub struct BearerToken(pub String);

impl BearerToken {
    pub fn new(api_key: &str) -> Self {
        Self(format!("Bearer {}", api_key))
    }
}

impl From<BearerToken> for String {
    fn from(value: BearerToken) -> Self {
        value.0
    }
}

pub fn post(args: &ChatCompletionPostArgs) -> Result<String, AnyError> {
    let builder = reqwest::blocking::Client::builder();
    let client = builder
        .timeout(Duration::from_secs(*args.timeout))
        .build()?;

    let payload = CompletionPayload::new(args.model_name, args.message);
    let bearer_token: String = BearerToken::new(args.api_key).into();

    if *args.debug {
        println!("payload: {:?}", payload);
    }

    let response = client
        .post(ENDPOINT)
        .header("Content-Type", "application/json")
        .header("Authorization", bearer_token)
        .json(&payload)
        .send()?;

    let response = response.json::<CompletionResponse>()?;

    match response {
        CompletionResponse::Error(CompletionErrorResponse { error }) => {
            let message = format!(
                "Error: {} {} {} {}",
                error.message, error.r#type, error.param, error.code
            );

            Err(message.into())
        }
        CompletionResponse::Success(message) => {
            let choice = message
                .choices
                .last()
                .ok_or("Error: no text found in response from ChatGPT API")?;

            Ok(choice.message.content.to_owned())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completion_payload_new() {
        let payload = CompletionPayload::new("gpt-4", "Hello world");

        assert_eq!(payload.model, CompletionModel::Gpt4);
        assert_eq!(payload.messages.len(), 1);
        assert_eq!(payload.messages[0].content, "Hello world");
    }

    #[test]
    fn test_completion_model_new() {
        assert_eq!(CompletionModel::new("gpt-4"), CompletionModel::Gpt4);
        assert_eq!(
            CompletionModel::new("gpt-3.5-turbo"),
            CompletionModel::Gpt3Turbo
        );
    }
}
