use std::fs::File;
use std::io::{self, Read};
use std::process::Output;
use std::str::FromStr;

// use anyhow::{Error, Ok};

// use anyhow::Ok;

use crate::chunk_type::ChunkType;
use crate::png::Png;

fn string_to_bool(s: &str) -> Option<bool> {
    // Trim whitespace and convert to lowercase for case-insensitive matching
    match s.trim().to_lowercase().as_str() {
        // Matches strings that represent 'true'
        "true" | "yes" | "on" | "1" => Some(true),
        // Matches strings that represent 'false'
        "false" | "no" | "off" | "0" => Some(false),
        // If the string doesn't match any known boolean representation
        _ => None,
    }
}

fn read_file(file_name: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(&file_name)?;
    let metadata = std::fs::metadata(&file_name)?;
    let mut buffer: Vec<u8> = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("buffer overeflow");
    Ok(buffer)
}
#[derive(Debug)]
enum Commands {
    Encode(EncodeCommand),
    Decode,
    Remove,
    Print,
}

#[derive(Debug)]
struct EncodeCommand {
    data: Vec<u8>,
    chunk_type: ChunkType,
    message: String,
    output: bool,
}

// struct
#[derive(Debug)]
pub struct Config {
    command: Commands,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, anyhow::Error> {
        let output = match args.get(5) {
            Some(arg) if arg.is_empty() => {
                eprintln!("Failed to parse case sensitivity, using default value false.");
                false
            }
            Some(arg) => match string_to_bool(arg) {
                Some(value) => value,
                None => {
                    eprintln!("Failed to parse case sensitivity, using default value.");
                    false
                }
            },
            None => {
                eprintln!("Failed to parse case sensitivity, using default value.");
                false
            }
        };
        let cmd: Commands = match args[1].as_str() {
            "encode" => Commands::Encode(EncodeCommand {
                data: read_file(&args[2].to_string())?,
                chunk_type: ChunkType::from_str(args[3].as_str()).unwrap(),
                message: args[4].clone(),
                output: output,
            }),
            "decode" => Commands::Decode,
            "remove" => Commands::Remove,
            "print" => Commands::Print,
            _ => panic!("Invalid command. Use encode, decode, remove, or print."),
        };
        Ok(Config { command: cmd })
    }

    fn operation(&self) {
        println!("{:?}", &self.command);
        match &self.command {
            Commands::Encode(encode_cmd) => {
                println!("Executing encode command: {:?}", encode_cmd);
            },
            Commands::Decode => {
                println!("Executing decode command");
            },
            Commands::Remove => {
                println!("Executing remove command");
            },
            Commands::Print => {
                println!("Executing print command");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_config() -> Config {
        Config::build(&[
            "0".to_string(),
            "encode".to_string(),
            "png_file.png".to_string(),
            "ruSt".to_string(),
            "This is a secret message!".to_string(),
            "0".to_string(),
        ])
        .unwrap()
    }

    #[test]
    fn run() {
        let config = build_config();
        println!("{:?}", config.command)
    }

    #[test]
    fn encode_png() {
        let config = build_config();
        config.operation()
    }

    // #[test]
    // fn test_encode_command() {
    //     let config = build_config(Commands::encode, "tests/images/sample.png");
    //     match config.command {
    //         Commands::encode => assert!(true),
    //         _ => panic!("Expected encode command"),
    //     }
    // }

    // #[test]
    // fn test_decode_command() {
    //     let config = build_config(Commands::decode, "tests/images/sample.png");
    //     match config.command {
    //         Commands::decode => assert!(true),
    //         _ => panic!("Expected decode command"),
    //     }
    // }

    // #[test]
    // fn test_remove_command() {
    //     let config = build_config(Commands::remove, "tests/images/sample.png");
    //     match config.command {
    //         Commands::remove => assert!(true),
    //         _ => panic!("Expected remove command"),
    //     }
    // }

    // #[test]
    // fn test_print_command() {
    //     let config = build_config(Commands::print, "tests/images/sample.png");
    //     match config.command {
    //         Commands::print => assert!(true),
    //         _ => panic!("Expected print command"),
    //     }
    // }
}
