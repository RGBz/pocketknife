extern crate serde_json;

use crate::error::AnyError;
use clap::Parser;

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

    if !cli.in_place && cli.input.is_none() {
        println!("{}", response);
        return Ok(());
    }

    if cli.in_place && cli.input.is_some() {
        file::write(&cli.input.unwrap(), response.as_bytes())?;
        return Ok(());
    }

    Err("Cannot use --in-place without file input".into())
}
