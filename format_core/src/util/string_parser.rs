use std::io::{self, Read};
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseLengthPrefixedStringError {
    #[error("IO error while reading string: {0}")]
    Io(#[from] io::Error),

    #[error("Invalid UTF-8 while parsing string of length {length}: {source}")]
    Utf8 {
        length: usize,
        #[source]
        source: FromUtf8Error,
    },
}

pub enum StringLength {
    U16,
    Fixed(usize),
}

pub enum Endianness {
    Big,
    Little,
}

/// Generalized function for reading binary length-prefixed strings
pub fn parse_string(
    cursor: &mut io::Cursor<Vec<u8>>,
    length_type: StringLength,
    length_endianness: Endianness,
) -> Result<String, ParseLengthPrefixedStringError> {
    let length = match length_type {
        StringLength::U16 => {
            let mut length_bytes: [u8; 2] = [0, 0];
            cursor.read_exact(&mut length_bytes)?;
            let size = match length_endianness {
                Endianness::Big => u16::from_be_bytes(length_bytes),
                Endianness::Little => u16::from_le_bytes(length_bytes),
            };
            usize::from(size)
        }
        StringLength::Fixed(size) => size,
    };

    let mut buffer = vec![0; length];
    cursor.read_exact(&mut buffer)?;
    let string = String::from_utf8(buffer)
        .map_err(|e| ParseLengthPrefixedStringError::Utf8 { length, source: e })?;

    Ok(string)
}
