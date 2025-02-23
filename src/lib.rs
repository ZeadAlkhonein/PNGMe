use std::fs::File;
use std::io::{self, Bytes, Read, Write};

use args::Config;
use chunk::Chunk;
use commands::{Commands, OperationResult};
use png::Png;
pub mod args;
pub mod chunk;
pub mod chunk_type;
pub mod commands;
pub mod png;

pub fn read_file(file_name: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(&file_name)?;
    let metadata = std::fs::metadata(&file_name)?;
    let mut buffer: Vec<u8> = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("buffer overeflow");
    Ok(buffer)
}

pub fn write_file(name: &String, byte: Vec<u8>) -> std::io::Result<()> {
    let nw_file_name = format!("{}.png", name);
    let mut file = File::create(nw_file_name)?;
    file.write_all(byte.as_slice())
}

pub fn check_string(s: &str) -> Option<&str> {
    if !s.is_empty() {
        Some(s)
    } else {
        None
    }
}

pub fn operation(config: &Config) -> Result<OperationResult, String> {
        // println!("{:?}", &self.command);
        match &config.command {
            Commands::Encode(encode_cmd) => {
                let mut png = match Png::try_from(encode_cmd.data().as_slice()) {
                    Ok(png) => png,
                    Err(_) => return Err("Error".to_string()), // fix later on
                };
                let new_data = Chunk::new(
                    encode_cmd.chunk_type().clone(),
                    encode_cmd.message().as_bytes().to_vec(),
                );
                png.append_chunk(new_data);

                if !encode_cmd.output().is_empty() {
                    let _ = write_file(encode_cmd.output(), png.as_bytes());
                }

                Ok(OperationResult::EncodedPng(png))
            }
            Commands::Decode(decode_cmd) => {
                let png = match Png::try_from(decode_cmd.data().as_slice()) {
                    Ok(png) => png,
                    Err(e) => return Err(e.to_string()),
                };

                let message = match png.chunk_by_type(&decode_cmd.chunk_type().to_string()) {
                    Some(chunk) => match chunk.data_as_string() {
                        Ok(msg) => msg,
                        Err(e) => return Err(e.to_string()),
                    },
                    None => "Chunk not found".to_string(),
                };

                Ok(OperationResult::DecodedMessage(message))
            }
            Commands::Remove(remove_cmd) => {
                let mut png = match Png::try_from(remove_cmd.data().as_slice()) {
                    Ok(png) => png,
                    Err(e) => return Err(e.to_string()),
                };

                let chunk = match png.remove_chunk(&remove_cmd.chunk_type().to_string()) {
                    Ok(chunk) => chunk,
                    Err(e) => return Err(e.to_string()),
                };

                Ok(OperationResult::RemovedChunk(chunk))
            }
            Commands::Print(print_cmd) => {
                let png = match Png::try_from(print_cmd.data().as_slice()) {
                    Ok(png) => png,
                    Err(e) => return Err(e.to_string()),
                };

                let chunk = match png.chunk_by_type(&print_cmd.chunk_type().to_string()) {
                    Some(chunk) => chunk,
                    None => return Err("Chunk not found".into()),
                };
                println!("{}", chunk.chunk_type().to_string());

                let mut stdout = io::stdout();
                let result: Result<(), io::Error> = writeln!(
                    stdout,
                    "Executing print command\n and the chunk is === {}",
                    chunk.chunk_type().to_string()
                ); // Returns Result

                Ok(OperationResult::PrintedInfo(result))
            }
        }
    }
