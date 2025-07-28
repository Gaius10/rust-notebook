
pub struct Config {
    pub query: String,
    pub file_path: String,
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

        Ok(Config { query, file_path })
    }
}

