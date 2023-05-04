extern crate serde_json;

use clap::Parser;
use crate::error::AnyError;

pub mod chat;
pub mod cli;
pub mod error;
pub mod file;

fn main() -> Result<(), AnyError> {
    let cli = cli::Cli::parse();
    let message = cli.get_full_message()?;

    let response = chat::post(&chat::Args {
        message: &message,
        model_name: &cli.model_name,
        api_key: &cli.api_key,
        timeout: &cli.timeout,
        debug: &cli.debug,
    })?;

    if cli.in_place && cli.input.is_some() {
        file::write(&cli.input.unwrap(), response.as_bytes())?;
    } else {
        println!("{}", response);
    }

    Ok(())
}
