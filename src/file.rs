use std::fs::File;
use std::io::Read;
use std::io::Write;

use crate::error::AnyError;

pub fn read(filename: &str) -> Result<String, AnyError> {
    let mut file =
        File::open(filename).map_err(|_| format!("Error: could not open file '{}'", filename))?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .map_err(|_| format!("Error: could not read file '{}'", filename))?;

    Ok(contents)
}

pub fn write(filename: &str, contents: &[u8]) -> Result<(), AnyError> {
    let mut file = File::create(filename)
        .map_err(|_| format!("Error: could not create file '{}'", filename))?;

    file.write_all(contents)
        .map_err(|_| format!("Error: could not write to file '{}'", filename))?;

    Ok(())
}
