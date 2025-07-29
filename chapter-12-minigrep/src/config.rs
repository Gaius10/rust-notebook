use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, String> {
        if args.len() < 3 {
            return Err(format!(
                "Not enough arguments.\n\
                Usage: {} <search_string> <file_path>",
                args[0]
            ));
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

