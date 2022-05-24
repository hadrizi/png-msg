#![allow(unused_variables, dead_code)]
use std::{
    fmt::Display,
    str::{from_utf8, FromStr},
};

use anyhow::{bail, Error, Ok, Result};

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl ChunkType {
    /// Returns chunk type bytes as u8 array
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    /// Checks if chunk type is critical, i.e. if bit 5 of first byte is **unset**
    pub fn is_critical(&self) -> bool {
        self.test_property_bit_unset(0)
    }

    /// Checks if chunk type is public, i.e. if bit 5 of second byte is **unset**
    pub fn is_public(&self) -> bool {
        self.test_property_bit_unset(1)
    }

    /// Checks if chunk type reserved bit is valid, i.e. if bit 5 of third byte is **set**
    pub fn is_safe_to_copy(&self) -> bool {
        !self.test_property_bit_unset(3)
    }

    /// Checks if chunk type reserved bit is valid, i.e. if bit 5 of third byte is **unset**
    /// Checks if chunk type is valid \
    /// Valid chunk type
    ///     consist of uppercase and lowercase ASCII letters (A-Z and a-z, or 65-90 and 97-122 decimal) and
    ///     has reserved bit **unset**
    fn is_valid(&self) -> bool {
        let in_range = self.bytes.iter().all(|b| b.is_ascii_alphabetic());
        let reserved_bit_valid = self.is_reserved_bit_valid();

        in_range && reserved_bit_valid
    }

    fn is_reserved_bit_valid(&self) -> bool {
        self.test_property_bit_unset(2)
    }

    /// Tests if bit 5 of `idx` byte is **unset**
    fn test_property_bit_unset(&self, idx: usize) -> bool {
        self.bytes[idx].is_ascii_uppercase()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self> {
        let chunk_type = Self { bytes: value };
        match chunk_type.is_valid() {
            true => Ok(chunk_type),
            false => bail!("chunk type is invalid"),
        }
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let chunk_type = Self {
            bytes: s.as_bytes().try_into()?,
        };
        match chunk_type.is_valid() {
            true => Ok(chunk_type),
            false => bail!("chunk type is invalid"),
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // no need to check for error since structure implementation implies this
        write!(f, "{}", from_utf8(&self.bytes).unwrap())
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
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
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
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust");
        assert!(chunk.is_err());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let chunk_string = format!("{}", chunk_type_1);
        let are_chunks_equal = chunk_type_1 == chunk_type_2;
        assert_eq!(chunk_string, "RuSt");
        assert!(are_chunks_equal)
    }
}
