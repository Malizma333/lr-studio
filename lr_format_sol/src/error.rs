use std::{io, num::TryFromIntError, str::Utf8Error};

use thiserror::Error;

use amf0::Amf0DeserializationError;

#[derive(Error, Debug)]
pub enum SolReadError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Failed to convert integer: {0}")]
    TryFromInt(#[from] TryFromIntError),
    #[error("Failed to parse utf8 string: {0}")]
    Utf8Parsing(#[from] Utf8Error),
    #[error("Failed to parse utf8 string: {0}")]
    Amf0Deserialization(#[from] Amf0DeserializationError),
    #[error("Invalid magic number: {0}")]
    InvalidMagicNumber(String),
    #[error("Missing track list")]
    MissingTrackList,
    #[error("Invalid track list: {0}")]
    InvalidTrackList(String),
    #[error("Invalid track index: {0}")]
    InvalidTrackIndex(String),
    #[error("Invalid track: {0}")]
    InvalidTrack(String),
    #[error("Invalid label: {0}")]
    InvalidLabel(String),
    #[error("Invalid grid version: {0}")]
    InvalidGridVersion(String),
    #[error("Unsupported grid version: {0}")]
    UnsupportedGridVersion(String),
    #[error("Invalid start line: {0}")]
    InvalidStartLine(String),
    #[error("Invalid lines list: {0}")]
    InvalidLinesList(String),
    #[error("Invalid line: {0}")]
    InvalidLine(String),
    #[error("Unsupported line type: {0}")]
    UnsupportedLineType(String),
}
