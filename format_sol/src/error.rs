use std::{
    io,
    num::{ParseFloatError, ParseIntError, TryFromIntError},
};

use format_core::util::string_parser::ParseLengthPrefixedStringError;
use thiserror::Error;

use crate::{Amf0DeserializationError, Amf0SerializationError};

#[derive(Error, Debug)]
pub enum SolReadError {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    TryFromInt(#[from] TryFromIntError),
    #[error("{0}")]
    IntConversion(#[from] ParseIntError),
    #[error("{0}")]
    FloatConversion(#[from] ParseFloatError),
    #[error("{0}")]
    StringParsing(#[from] ParseLengthPrefixedStringError),
    #[error("Invalid value for `{name}`: {value}")]
    InvalidData { name: String, value: String },
    #[error("{0}")]
    Amf0Deserialization(#[from] Amf0DeserializationError),
}

#[derive(Error, Debug)]
pub enum SolWriteError {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    IntConversion(#[from] TryFromIntError),
    #[error("{0}")]
    Amf0Serialization(#[from] Amf0SerializationError),
}
