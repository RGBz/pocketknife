use clap::Parser;
use std::fs::File;
use std::io::Read;

use crate::error::AnyError;

#[derive(Parser, Debug)]
pub struct Cli {
    /// The prompt to send to chat GPT
    pub prompt: String,
    /// OpenAI API key
    #[arg(short, long, env = "OPENAI_API_KEY")]
    pub api_key: String,
    /// OpenAI API model name to use
    #[arg(short, long, env = "OPENAI_MODEL", default_value = "gpt-4")]
    pub model_name: String,
    /// An optional file input to send to chat GPT
    #[arg(short, long)]
    pub input: Option<String>,
    /// Optional file name to output response to
    #[arg(short, long)]
    pub output: Option<String>,
    /// Length of time in seconds to wait for a response from the API
    #[arg(short, long, default_value = "60")]
    pub timeout: u64,
    /// Whether to debug CLI operation
    #[arg(short, long, default_value = "false")]
    pub debug: bool,
}

impl Cli {
    pub fn get_full_message(&self) -> Result<String, AnyError> {
        if let Some(filename) = &self.input {
            let mut file = File::open(filename)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            return Ok(format!("{} ```{}```", self.prompt, &contents));
        }

        Ok(self.prompt.clone())
    }
}
