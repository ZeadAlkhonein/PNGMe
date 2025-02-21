use std::fs::File;
use std::io::{self, Bytes, Read, Write};
pub mod args;
pub mod chunk;
pub mod chunk_type;
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
