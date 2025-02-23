use std::fmt;
use std::str::FromStr;

use anyhow::Error;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl fmt::Display for ChunkType {
    // need to add error handling in the future
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8(self.bytes.to_vec()).unwrap());
        Ok(())
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    pub fn new(val: [u8; 4]) -> ChunkType {
        ChunkType { bytes: val }
    }

    pub fn is_valid(&self) -> bool {
        !self.is_public() && self.is_reserved_bit_valid()
    }

    pub fn is_critical(&self) -> bool {
        if self.bytes[0].is_ascii_uppercase() {
            return true;
        }
        false
    }
    fn is_public(&self) -> bool {
        if self.bytes[1].is_ascii_uppercase() {
            return true;
        }
        return false;
    }

    fn is_reserved_bit_valid(&self) -> bool {
        if self.bytes[2].is_ascii_uppercase() {
            return true;
        }
        return false;
    }

    fn is_safe_to_copy(&self) -> bool {
        if self.bytes[3].is_ascii_lowercase() {
            return true;
        }
        return false;
    }
}

impl TryFrom<[i32; 4]> for ChunkType {
    type Error = anyhow::Error;

    fn try_from(value: [i32; 4]) -> Result<Self, Self::Error> {
        let byte_vec: [u8; 4] = value.map(|i| i as u8);
        Ok(ChunkType { bytes: byte_vec })
    }
}

impl FromStr for ChunkType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            let error_msg = "Length is not equal to 4 ".to_string();
            return Err(Error::msg(error_msg));
        }

        if !s.chars().all(|x| x.is_ascii_alphabetic()) {
            let error_msg = "contains number".to_string();
            return Err(Error::msg(error_msg));
        }

        let temp = s.as_bytes().to_vec();
        Ok(ChunkType {
            bytes: temp.try_into().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
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

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
        // println!("{}", chunk)
    }
    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        print!("{}", chunk);
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }
}
