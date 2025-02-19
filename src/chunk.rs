use std::{error, path::Display};
// use std::result::Result::Ok;
use anyhow::{Error, Ok};
use crc::{self, Crc, CRC_32_ISO_HDLC};
use std::fmt;

// mod chunk_type;
// use crate::pngme::chunk_type;
use crate::chunk_type::{self, ChunkType};

fn convert_into_checksum(crc: u32, chunk_type: ChunkType, data: Vec<u8>) -> u32 {
    let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
    let mut digest = crc.digest();
    digest.update(chunk_type.bytes().as_slice());
    digest.update(data.as_slice());
    let calculated_crc = digest.finalize();
    calculated_crc
}
#[derive(Debug)]
pub struct Chunk {
    data_length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
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

    pub fn length(&self) -> u32 {
        self.data_length
    }
    pub fn chunk_type(&self) -> ChunkType {
        self.chunk_type.clone()
    }
    pub fn data(&self) -> &[u8] {
        &self.chunk_data
    }
    pub fn crc(&self) -> u32 {
        self.crc
    }
    pub fn data_as_string(&self) -> Result<String, Error> {
        match String::from_utf8(self.chunk_data.clone()) {
            std::result::Result::Ok(data) => Ok(data),
            Err(e) => Err(e.into()),
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.data_length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.chunk_data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }

    // pub fn header_to_chunk() -> Chunk {
    //     // The signature is 8 bytes, but it’s not a standard chunk, so we define a custom type
    //     let signature = crate::png::Png::STANDARD_HEADER;
    //     let header_chunk_type = ChunkType::new([83, 73, 71, 78]);

    //     Chunk {
    //         data_length: 8, // Length of the signature data
    //         chunk_type: header_chunk_type, // "SIGN" (arbitrary custom type)
    //         chunk_data: signature.to_vec(), // [137, 80, 78, 71, 13, 10, 26, 10]
    //         crc: 0,         // CRC isn’t defined for the signature; use 0 or compute one
    //     }
    // }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "chunk length is {} and chunk_type {} and chunk data {} and chuck crc is {} ",
            self.length(),
            self.chunk_type(),
            String::from_utf8(self.data().to_vec()).unwrap(),
            self.crc()
        )
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // println!("i am here i have issue: ======> {:?}", &value);
        let length = u32::from_be_bytes(value[0..4].try_into()?);
        // println!("i am here i have issue: ===>{:?}", &value);

        let chunk_type = &value[4..8];
        // println!("i am here i have issue: ===>{:?}", &value);

        let message = &value[8..(8 + length as usize)];
        // println!("i am here i have issue: ===>{:?}", &value);

        let crc = u32::from_be_bytes(value[(8 + length as usize)..].try_into()?);

        // println!("i am here i have issue: ===>{:?}", &value);

        let chunk = Chunk {
            data_length: length,
            chunk_type: ChunkType::new(chunk_type.try_into()?),
            chunk_data: message.to_vec(),
            crc: crc,
        };

        let crc = convert_into_checksum(chunk.crc(), chunk.chunk_type(), chunk.data().to_vec());

        if crc != chunk.crc() {
            return Err(anyhow::anyhow!("CRC check failed"));
        }

        Ok(chunk)
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

        // println!("{:?}", chunk_data);

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }
    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
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
        println!("{}", chunk.chunk_type());
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
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

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }
    #[test]
    pub fn test_chunk_trait_impls() {
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

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
