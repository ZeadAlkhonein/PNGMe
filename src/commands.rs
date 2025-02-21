use crate::chunk_type::{self, ChunkType};

#[derive(Debug)]
pub enum Commands {
    Encode(EncodeCommand),
    Decode(DataCommand),
    Remove(DataCommand),
    Print(DataCommand),
}

#[derive(Debug)]
pub struct DataCommand {
    data: Vec<u8>,
    chunk_type: ChunkType,
}

impl DataCommand {
    pub fn new(data: Vec<u8>, chunk_type: ChunkType) -> DataCommand {
        DataCommand {
            data: data,
            chunk_type: chunk_type,
        }
    }
    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
}

#[derive(Debug)]
pub struct EncodeCommand {
    data: Vec<u8>,
    chunk_type: ChunkType,
    message: String,
    output: String,
}

impl EncodeCommand {
    pub fn new(data: Vec<u8>, chunk_type: ChunkType, message: String, output: String) -> EncodeCommand {
        EncodeCommand {
            data,
            chunk_type,
            message,
            output,
        }
    }
    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn output(&self) -> &String {
        &self.output
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
}
