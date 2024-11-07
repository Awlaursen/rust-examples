use std::fs;
use std::io;

enum ConfigError {
    FileNotFound(io::Error),
    InvalidFormat,
}

fn read_config(filename: &str) -> Result<i32, ConfigError> {
    // Try to read the file
    // useful functions: fs::read_to_string, Result::map_err 
    let contents = !todo!("Read the contents of the file");

    // Try to parse the contents as an integer
    // useful functions: str::parse, Result::map_err
    let number = !todo!("Parse the contents as an integer");

    Ok(number)
}

fn main() {
    match read_config("config.txt") {
        Ok(num) => println!("Config value: {}", num),
        Err(ConfigError::FileNotFound(_)) => println!("Config file not found."),
        Err(ConfigError::InvalidFormat) => println!("Config file has invalid format."),
    }
}
