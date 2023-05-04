use clap::Parser;

use crate::error::AnyError;
use crate::file;

#[derive(Parser, Debug)]
pub struct Cli {
    /// The prompt to send to chat GPT
    pub prompt: String,
    /// An optional file input to send to chat GPT
    pub input: Option<String>,
    /// OpenAI API key
    #[arg(short, long, env = "OPENAI_API_KEY")]
    pub api_key: String,
    /// OpenAI API model name to use
    #[arg(short, long, env = "OPENAI_MODEL", default_value = "gpt-4")]
    pub model_name: String,
    /// Length of time in seconds to wait for a response from the API
    #[arg(short, long, env = "OPENAI_TIMEOUT", default_value = "60")]
    pub timeout: u64,
    /// Option to redirect output in place to the input file
    #[arg(short, long, default_value = "false")]
    pub in_place: bool,
    /// Whether to debug CLI operation
    #[arg(short, long, default_value = "false")]
    pub debug: bool,
}

impl Cli {
    pub fn get_full_message(&self) -> Result<String, AnyError> {
        let prompt = format!("Please output without extra explanation {}", self.prompt);

        if let Some(filename) = &self.input {
            let contents = file::read(filename)?;
            return Ok(format!("{}:\n\n{}", prompt, &contents));
        }

        Ok(prompt)
    }
}
