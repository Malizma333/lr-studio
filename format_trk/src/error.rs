use std::{
    io,
    num::{ParseFloatError, ParseIntError, TryFromIntError},
    string::FromUtf8Error,
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TrkReadError {
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
    #[error("invalid magic number: {0}")]
    InvalidMagicNumber(String),
    #[error("unsupported track version: {0}")]
    UnsupportedTrackVersion(String),
    #[error("invalid song data format: {0}")]
    InvalidSongFormat(String),
    #[error("unsupported line type: {0}")]
    UnsupportedLineType(String),
    #[error("invalid key value format: {0}")]
    InvalidKeyValue(String),
    #[error("empty trigger data")]
    EmptyTriggerData,
    #[error("unsupported trigger type: {0}")]
    UnsupportedTriggerType(String),
}
