use anyhow::Ok;

use crate::chunk;
use std::str::FromStr;

// #[derive(Debug, Clone, PartialEq, Eq)]
// struct field {
//     field : u8
// }

#[derive(Debug, Clone, PartialEq, Eq)]
struct ChunkType {
    bytes: [u8; 4]
}

impl ChunkType {

    pub fn bytes(&self) -> [u8; 4]{
        self.bytes
    }
    
}

impl TryFrom<[i32; 4]> for ChunkType {
    type Error = anyhow::Error;

    fn try_from(value: [i32; 4]) -> Result<Self, Self::Error> {
        
        let byte_vec: [u8; 4] = value.map(|i| i as u8);
        Ok(ChunkType {bytes: byte_vec})

    }
}

impl FromStr for ChunkType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Ok(s.as_bytes())
        let temp = s.as_bytes().to_vec();
        // temp.try_into().unwrap()
        Ok(ChunkType {bytes: temp.try_into().unwrap()})
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected= [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
        // assert_eq!(actual.bytes(), expected )
    }
    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }
    

}

