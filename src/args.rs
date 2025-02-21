use crate::{read_file, write_file};
use std::str::FromStr;
use std::io::{self, Error, Write};
use crate::chunk::{self, Chunk};
use crate::chunk_type::ChunkType;
use crate::png::{self, Png};

#[derive(Debug)]
pub enum OperationResult {
    EncodedPng(Png),
    DecodedMessage(String),
    RemovedChunk(Chunk),
    PrintedInfo(Result<(), Error>),
}

fn check_string(s: &str) -> Option<&str> {
    if !s.is_empty() {
        Some(s)
    } else {
        None
    }
}

#[derive(Debug)]
enum Commands {
    Encode(EncodeCommand),
    Decode(DataCommand),
    Remove(DataCommand),
    Print(DataCommand),
}

#[derive(Debug)]
struct DataCommand {
    data: Vec<u8>,
    chunk_type: ChunkType,
}

#[derive(Debug)]
struct EncodeCommand {
    data: Vec<u8>,
    chunk_type: ChunkType,
    message: String,
    output: String,
}

// struct
#[derive(Debug)]
pub struct Config {
    command: Commands,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, anyhow::Error> {
        let output: String = if args.len() > 4 {
            match check_string(&args.get(5).unwrap()) {
                Some(arg) => arg.to_string(),
                None => String::new(),
            }
        } else {
            String::new()
        };

        let cmd: Commands = match args[1].as_str() {
            "encode" => Commands::Encode(EncodeCommand {
                data: read_file(&args[2].to_string())?,
                chunk_type: ChunkType::from_str(args[3].as_str()).unwrap(),
                message: args[4].clone(),
                output: output,
            }),

            "decode" => Commands::Decode(DataCommand {
                data: read_file(&args[2].to_string())?,
                chunk_type: ChunkType::from_str(args[3].as_str()).unwrap(),
            }),
            "remove" => Commands::Remove(DataCommand {
                data: read_file(&args[2].to_string())?,
                chunk_type: ChunkType::from_str(args[3].as_str()).unwrap(),
            }),
            "print" => Commands::Print(DataCommand {
                data: read_file(&args[2].to_string())?,
                chunk_type: ChunkType::from_str(args[3].as_str()).unwrap(),
            }),
            _ => panic!("Invalid command. Use encode, decode, remove, or print."),
        };
        Ok(Config { command: cmd })
    }

    fn operation(&self) -> Result<OperationResult, String> {
        // println!("{:?}", &self.command);
        match &self.command {
            Commands::Encode(encode_cmd) => {
                let mut png = match Png::try_from(encode_cmd.data.as_slice()) {
                    Ok(png) => png,
                    Err(_) => return Err("Error".to_string()), // fix later on
                };
                let new_data = Chunk::new(
                    encode_cmd.chunk_type,
                    encode_cmd.message.as_bytes().to_vec(),
                );
                png.append_chunk(new_data);

                if !encode_cmd.output.is_empty() {
                    let _ = write_file(&encode_cmd.output, png.as_bytes());
                }

                Ok(OperationResult::EncodedPng(png))
            }
            Commands::Decode(decode_cmd) => {
                // todo!();
                let png = match Png::try_from(decode_cmd.data.as_slice()) {
                    Ok(png) => png,
                    Err(e) => return Err(e.to_string()),
                };
                let chunk = match png.chunk_by_type(&decode_cmd.chunk_type.to_string()) {
                    Some(chunk) => chunk,
                    None => return Err("Chunk not found".into()),
                };
                Ok(OperationResult::DecodedMessage(
                    (match chunk.data_as_string() {
                        Ok(massage) => massage,
                        Err(e) => Err(format!("{}", e.to_string())).unwrap(),
                    }),
                ))
            }
            Commands::Remove(remove_cmd) => {
                let mut png = match Png::try_from(remove_cmd.data.as_slice()) {
                    Ok(png) => png,
                    Err(e) => return Err(e.to_string()),
                };

                let chunk = match png.remove_chunk(&remove_cmd.chunk_type.to_string()) {
                    Ok(chunk) => chunk,
                    Err(e) => return Err(e.to_string()),
                };

                Ok(OperationResult::RemovedChunk(chunk))
            }
            Commands::Print(print_cmd) => {
                let png = match Png::try_from(print_cmd.data.as_slice()) {
                    Ok(png) => png,
                    Err(e) => return Err(e.to_string()),
                };

                let chunk = match png.chunk_by_type(&print_cmd.chunk_type.to_string()) {
                    Some(chunk) => chunk,
                    None => return Err("Chunk not found".into()),
                };
                println!("{}", chunk.chunk_type().to_string());

                let mut stdout = io::stdout();
                let result: Result<(), io::Error> = writeln!(stdout, "Executing print command\n and the chunk is === {}", chunk.chunk_type().to_string()); // Returns Result

                Ok(OperationResult::PrintedInfo(result))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // use anyhow::Ok;

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
        let operation_result = config.operation().unwrap();

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
        let operation_result = config.operation().unwrap();

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
        let operation_result = config.operation().unwrap();

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
        let operation_result = config.operation().unwrap();

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
        let operation_result = config.operation().unwrap();
        // println!("==================");
        // println!("{:?}", operation_result);
        match operation_result {
            OperationResult::PrintedInfo(result) => {
                assert!(result.is_ok())

            }
            _ => panic!("Expected PrintedInfo variant"),
        }
    }


}
