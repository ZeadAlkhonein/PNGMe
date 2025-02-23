use anyhow::Error;

use crate::chunk::{self, Chunk};
use crate::chunk_type::{self, ChunkType};
use crate::commands::{Commands, DataCommand, EncodeCommand, OperationResult};
use crate::png::{self, Png};
use crate::{check_string, operation, read_file, write_file};
use std::io::{self, Write};
use std::str::FromStr;
// use crate::

// struct
#[derive(Debug)]
pub struct Config {
    pub command: Commands,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, anyhow::Error> {
        if args.len() < 4 {
            let message = format!(
                "Not enough arguments provided: expected at least 4, got {}",
                args.len()
            );
            return Err(Error::msg(message));
        }

        let data = read_file(&args[2].to_string())?;
        let chunk_type = ChunkType::from_str(args[3].as_str())?;

        let cmd: Commands = match args[1].as_str() {
            "encode" => {
                let message = args[4].clone();
                let output: String = if args.len() > 4 {
                    match check_string(&args.get(5).unwrap()) {
                        Some(arg) => arg.to_string(),
                        None => String::new(),
                    }
                } else {
                    String::new()
                };
                Commands::Encode(EncodeCommand::new(data, chunk_type, message, output))
            }
            "decode" => Commands::Decode(DataCommand::new(data, chunk_type)),
            "remove" => Commands::Remove(DataCommand::new(data, chunk_type)),
            "print" => Commands::Print(DataCommand::new(data, chunk_type)),
            _ => panic!("Invalid command. Use encode, decode, remove, or print."),
        };
        Ok(Config { command: cmd })
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use super::*;

    fn read_and_write_png() -> Png {
        let mut png = encode_png_from_file().unwrap();
        let chunk_type = ChunkType::from_str("ruSt").unwrap();
        let chunk = Chunk::new(chunk_type, "Test Message".as_bytes().to_vec());
        png.append_chunk(chunk);
        let temp_png_file = "temp_png";
        let _ = write_file(&temp_png_file.to_string(), png.as_bytes()).unwrap();
        png
    }

    fn encode_png_from_file() -> Result<Png, String> {
        let data = read_file("png_file.png").unwrap();

        let mut png = match Png::try_from(data.as_slice()) {
            std::result::Result::Ok(png) => png,
            Err(e) => {
                return Err(format!("Failed to decode PNG: {}", e)); // Assuming your function returns `Result<Png, String>`
            }
        };
        std::result::Result::Ok(png)
    }

    fn build_config_encode() -> Config {
        Config::build(&[
            "0".to_string(),
            "encode".to_string(),
            "png_file.png".to_string(),
            "ruSt".to_string(),
            "This is a secret message!".to_string(),
            "new_png".to_string(),
        ])
        .unwrap()
    }

    fn verify_build_config_decode() -> Config {
        Config::build(&[
            "0".to_string(),
            "decode".to_string(),
            "new_png.png".to_string(),
            "ruSt".to_string(),
        ])
        .unwrap()
    }

    fn unverify_build_config_decode() -> Config {
        Config::build(&[
            "0".to_string(),
            "decode".to_string(),
            "png_file.png".to_string(),
            "ruSt".to_string(),
        ])
        .unwrap()
    }

    fn build_config_remove() -> Config {
        Config::build(&[
            "0".to_string(),
            "remove".to_string(),
            "temp_png.png".to_string(),
            "ruSt".to_string(), // Assuming "ruSt" is the chunk type to be removed
        ])
        .unwrap()
    }

    fn build_config_print() -> Config {
        Config::build(&[
            "0".to_string(),
            "print".to_string(),
            "temp_png.png".to_string(),
            "ruSt".to_string(), // Assuming "ruSt" is the chunk type to be removed
        ])
        .unwrap()
    }

    #[test]
    fn encode_png() {
        let config = build_config_encode();
        let operation_result = operation(&config).unwrap();

        match operation_result {
            OperationResult::EncodedPng(png) => {
                assert_eq!(
                    png.chunk_by_type("ruSt").unwrap().chunk_type().to_string(),
                    "ruSt"
                );
            }
            _ => panic!("Expected EncodedPng variant"),
        }
    }
    #[test]
    fn verify_secret_message_in_decoded_png() {
        let config = verify_build_config_decode();
        let operation_result = operation(&config).unwrap();

        match operation_result {
            OperationResult::DecodedMessage(message) => {
                assert_eq!(message, "This is a secret message!");
            }
            _ => println!("Expected decoded variant"),
        }
    }
    #[test]
    fn unverify_secret_message_in_decoded_png() {
        let config = unverify_build_config_decode();
        // println!("==================");

        let operation_result = operation(&config).unwrap();
        // println!("==================");

        match operation_result {
            OperationResult::DecodedMessage(message) => {
                assert_eq!(message, "Chunk not found");
            }
            _ => println!("Expected decoded variant"),
        }
    }

    #[test]
    fn remove_chunk_from_png() {
        let _ = read_and_write_png();

        let config = build_config_remove();
        let operation_result = operation(&config).unwrap();

        match operation_result {
            OperationResult::RemovedChunk(removed_chunk) => {
                // Check if the chunk was successfully removed
                assert_eq!(removed_chunk.chunk_type().to_string(), "ruSt");
            }
            _ => panic!("Expected RemovedChunk variant"),
        }
    }

    #[test]
    fn print_chunk_from_png() {
        let _ = read_and_write_png();

        let config = build_config_print();
        let operation_result = operation(&config).unwrap();
        match operation_result {
            OperationResult::PrintedInfo(result) => {
                assert!(result.is_ok())
            }
            _ => panic!("Expected PrintedInfo variant"),
        }
    }
}
