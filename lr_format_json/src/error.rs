use std::{fmt::Display, num::TryFromIntError, str::Utf8Error};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum JsonReadError {
    #[error("Failed to convert integer: {0}")]
    TryFromInt(#[from] TryFromIntError),
    #[error("Failed to parse utf8 string: {0}")]
    Utf8Parsing(#[from] Utf8Error),
    #[error("Failed to deserialize json: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Unsupported grid version: {0}")]
    UnsupportedGridVersion(String),
    #[error("Unsupported line type: {0}")]
    UnsupportedLineType(String),
    #[error("Unsupported trigger type: {0}")]
    UnsupportedTriggerType(String),
    #[error("Invalid trigger format")]
    InvalidTriggerFormat(#[from] InvalidTriggerFormatError),
}

#[derive(Clone, Debug, Error)]
pub struct InvalidTriggerFormatError(pub String);

impl Display for InvalidTriggerFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidTriggerFormatError: {}", self.0)
    }
}
