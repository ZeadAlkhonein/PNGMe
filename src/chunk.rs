use anyhow::Ok;
use crc::{self, Crc, CRC_32_ISO_HDLC};

// mod chunk_type;
// use crate::pngme::chunk_type;
use crate::chunk_type::{self, ChunkType};

// fn cconvert_into_checksum() -> u32 {}

struct Chunk {
    data_length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let mut digest = crc.digest();
        digest.update(chunk_type.bytes().as_slice());
        digest.update(data.as_slice());
        let calculated_crc = digest.finalize();

        Chunk {
            data_length: data.len() as u32,
            chunk_type: chunk_type,
            chunk_data: data,
            crc: calculated_crc,
        }
    }

    fn length(&self) -> u32 {
        self.data_length
    }
    fn chunk_type(&self) -> ChunkType {
        self.chunk_type.clone()
    }
    fn data(&self) -> &[u8] {
        &self.chunk_data
    }
    fn crc(&self) -> u32 {
        self.crc
    }
    fn data_as_string() -> u32 {
        todo!()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // let chunk_parts = value.iter().map(|i| *i as u8);
        // Ok(Chu)

        let length = u32::from_be_bytes(value[0..4].try_into()?);
        // let data_length = value[0..4].try_into()?;
        let chunk_type = &value[4..8];
        let message = &value[8..length as usize];
        // let crc = &value[(8 + length as usize)..];
        let crc = u32::from_be_bytes(value[(8 + length as usize)..].try_into()?);
        

        Ok(Chunk {
            data_length: length,
            chunk_type: ChunkType::new(chunk_type.try_into()?),
            chunk_data: message.to_vec(),
            crc: crc,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        println!("{:?}", chunk_data);

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }
    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        println!("{}",chunk.chunk_type() );
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }
}
