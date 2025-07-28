use std::fs;
use std::error::Error;
use crate::config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n\n{contents}");

    Ok(())
}

