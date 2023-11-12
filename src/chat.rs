use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::error::AnyError;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Usage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Choice {
    pub message: Message,
    pub finish_reason: String,
    pub index: usize,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct SuccessResponse {
    pub id: Option<String>,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub usage: Usage,
    pub choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Error {
    pub message: String,
    pub r#type: String,
    pub param: String,
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ErrorResponse {
    pub error: Error,
}

impl From<ErrorResponse> for String {
    fn from(value: ErrorResponse) -> Self {
        format!(
            "{} {} {} {}",
            value.error.message, value.error.r#type, value.error.param, value.error.code
        )
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum Response {
    Success(SuccessResponse),
    Error(ErrorResponse),
}

impl TryFrom<&str> for Response {
    type Error = serde_json::Error;

    fn try_from(value: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(value)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Model {
    #[serde(rename = "gpt-3.5-turbo")]
    Gpt3Turbo,
    #[serde(rename = "gpt-4")]
    Gpt4,
    #[serde(rename = "gpt-4-1106-preview")]
    Gpt41106Preview,
}

impl Model {
    pub fn new(model: &str) -> Result<Self, AnyError> {
        match model {
            "gpt-3.5-turbo" => Ok(Self::Gpt3Turbo),
            "gpt-4" => Ok(Self::Gpt4),
            "gpt-4-1106-preview" => Ok(Self::Gpt41106Preview),
            _ => Err("Not a valid model name".into()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Message {
    pub fn new(content: &str) -> Self {
        Self {
            role: "user".to_owned(),
            content: content.to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Payload {
    pub model: Model,
    pub messages: Vec<Message>,
}

impl Payload {
    pub fn new(model: &str, content: &str) -> Result<Self, AnyError> {
        Ok(Self {
            model: Model::new(model)?,
            messages: vec![Message::new(content)],
        })
    }
}

impl TryFrom<&Payload> for String {
    type Error = serde_json::Error;

    fn try_from(value: &Payload) -> Result<Self, Self::Error> {
        serde_json::to_string(value)
    }
}

pub struct Args<'a> {
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

const ENDPOINT: &str = "https://api.openai.com/v1/chat/completions";

pub fn post(args: &Args) -> Result<String, AnyError> {
    let timeout = Duration::from_secs(*args.timeout);
    let builder = reqwest::blocking::Client::builder();
    let client = builder
        .timeout(timeout)
        .build()
        .map_err(|_| "Could not create HTTP client")?;

    let payload = Payload::new(args.model_name, args.message)?;
    let token: String = BearerToken::new(args.api_key).into();

    if *args.debug {
        println!("payload: {:?}", payload);
    }

    let response = client
        .post(ENDPOINT)
        .header("Content-Type", "application/json")
        .header("Authorization", token)
        .json(&payload)
        .send()
        .map_err(|error| format!("Could not send request to ChatGPT API\n  {}", error))?;

    let response = response
        .json::<Response>()
        .map_err(|_| "Could not parse response from ChatGPT API")?;

    match response {
        Response::Error(message) => {
            let formatted: String = message.into();
            Err(formatted.into())
        }
        Response::Success(message) => {
            let last_choice = message.choices.last();
            let choice = last_choice.ok_or("No text found in response from ChatGPT API")?;

            Ok(choice.message.content.trim().to_owned())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payload_new() {
        let payload = Payload::new("gpt-4", "Hello world").unwrap();

        assert_eq!(payload.model, Model::Gpt4);
        assert_eq!(payload.messages.len(), 1);
        assert_eq!(payload.messages[0].content, "Hello world");
    }

    #[test]
    fn test_model_new_valid_input() {
        assert_eq!(Model::new("gpt-4").unwrap(), Model::Gpt4);
        assert_eq!(Model::new("gpt-3.5-turbo").unwrap(), Model::Gpt3Turbo);
    }

    #[test]
    fn test_model_new_invalid_input() {
        assert!(Model::new("not-a-model").is_err());
    }
}
