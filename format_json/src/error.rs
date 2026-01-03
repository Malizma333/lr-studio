use std::{
    fmt::Display,
    io,
    num::{ParseFloatError, ParseIntError, TryFromIntError},
    string::FromUtf8Error,
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum JsonReadError {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    TryFromInt(#[from] TryFromIntError),
    #[error("{0}")]
    IntConversion(#[from] ParseIntError),
    #[error("{0}")]
    FloatConversion(#[from] ParseFloatError),
    #[error("{0}")]
    FromUTF8(#[from] FromUtf8Error),
    #[error("{0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("unsupported grid version: {0}")]
    UnsupportedGridVersion(String),
    #[error("unsupported line type: {0}")]
    UnsupportedLineType(String),
    #[error("unsupported trigger type: {0}")]
    UnsupportedTriggerType(String),
    #[error("invalid trigger format")]
    InvalidTriggerFormat(#[from] InvalidTriggerFormatError),
}

#[derive(Clone, Debug, Error)]
pub struct InvalidTriggerFormatError(pub String);

impl Display for InvalidTriggerFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidTriggerFormatError: {}", self.0)
    }
}
