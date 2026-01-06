use std::{
    io,
    num::{ParseFloatError, ParseIntError, TryFromIntError},
    str::Utf8Error,
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TrkReadError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Failed to convert integer: {0}")]
    TryFromInt(#[from] TryFromIntError),
    #[error("Failed to parse integer: {0}")]
    IntConversion(#[from] ParseIntError),
    #[error("Failed to parse float: {0}")]
    FloatConversion(#[from] ParseFloatError),
    #[error("Failed to parse utf8 string: {0}")]
    Utf8Parsing(#[from] Utf8Error),
    #[error("Invalid magic number: {0}")]
    InvalidMagicNumber(String),
    #[error("Unsupported track version: {0}")]
    UnsupportedTrackVersion(String),
    #[error("Invalid song data format: {0}")]
    InvalidSongFormat(String),
    #[error("Unsupported line type: {0}")]
    UnsupportedLineType(String),
    #[error("Invalid key value format: {0}")]
    InvalidKeyValue(String),
    #[error("Empty trigger data")]
    EmptyTriggerData,
    #[error("Unsupported trigger type: {0}")]
    UnsupportedTriggerType(String),
}
