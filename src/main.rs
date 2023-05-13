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

    if cli.in_place {
        return match cli.input {
            None => Err("Cannot use --in-place without file input".into()),
            Some(input) => {
                file::write(&input, response.as_bytes())?;
                Ok(())
            }
        };
    }

    println!("{}", response);
    Ok(())
}
