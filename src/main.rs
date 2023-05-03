extern crate serde_json;

use clap::Parser;
use std::fs::File;
use std::io::Write;

pub mod chat;
pub mod cli;
pub mod error;

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

    if let Some(filename) = cli.output {
        let mut file = File::create(filename)?;
        file.write_all(response.as_bytes())?;
    } else {
        println!("{}", response);
    }

    Ok(())
}
