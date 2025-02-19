use std::fs::File;
use std::io::{self, Bytes, Read, Write};
use std::process::Output;
use std::str::FromStr;

use crate::chunk::{self, Chunk};
use crate::chunk_type::ChunkType;
use crate::png::{self, Png};


fn check_string(s: &str) -> Option<&str> {
    if !s.is_empty() {
        Some(s)
    } else {
        None
    }
}


fn read_file(file_name: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(&file_name)?;
    let metadata = std::fs::metadata(&file_name)?;
    let mut buffer: Vec<u8> = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("buffer overeflow");
    Ok(buffer)
}

fn write_file(name: &String, byte: Vec<u8>) -> std::io::Result<()>{
    let nw_file_name = format!("{}.png", name);
    let mut file = File::create(nw_file_name)?;
    file.write_all(byte.as_slice())
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
    output: String,
}


// struct
#[derive(Debug)]
pub struct Config {
    command: Commands,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, anyhow::Error> {
        let output: String = match check_string(&args.get(5).unwrap()) {
            Some(arg) => arg.to_string(),
            None => "".to_string()
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

    fn operation(&self) -> Result<Png, String> {
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

                Ok(png)
            }
            Commands::Decode => {
                todo!()
                // println!("Executing decode command");
            }
            Commands::Remove => {
                todo!()
                // println!("Executing remove command");
            }
            Commands::Print => {
                todo!()
                // println!("Executing print command");
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
            "new_png".to_string(),
        ])
        .unwrap()
    }

    #[test]
    fn run() {
        let config = build_config();
        // println!("{:?}", config.command)
    }

    #[test]
    fn encode_png() {
        let config = build_config();
        let new_png = config.operation().unwrap();
        assert_eq!(&new_png.chunk_by_type("ruSt").unwrap().chunk_type().to_string(), "ruSt");

    }

}
