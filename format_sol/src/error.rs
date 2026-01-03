use std::{
    io,
    num::{ParseFloatError, ParseIntError, TryFromIntError},
};

use format_core::util::string_parser::ParseLengthPrefixedStringError;
use thiserror::Error;

use amf0::{Amf0DeserializationError, Amf0SerializationError};

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
    #[error("{0}")]
    Amf0Deserialization(#[from] Amf0DeserializationError),
    #[error("invalid magic number: {0}")]
    InvalidMagicNumber(String),
    #[error("missing track list")]
    MissingTrackList,
    #[error("invalid track list: {0}")]
    InvalidTrackList(String),
    #[error("invalid track index: {0}")]
    InvalidTrackIndex(String),
    #[error("invalid track: {0}")]
    InvalidTrack(String),
    #[error("invalid label: {0}")]
    InvalidLabel(String),
    #[error("invalid grid version: {0}")]
    InvalidGridVersion(String),
    #[error("unsupported grid version: {0}")]
    UnsupportedGridVersion(String),
    #[error("invalid start line: {0}")]
    InvalidStartLine(String),
    #[error("invalid lines list: {0}")]
    InvalidLinesList(String),
    #[error("invalid line: {0}")]
    InvalidLine(String),
    #[error("invalid line type: {0}")]
    InvalidLineType(String),
    #[error("unsupported line type: {0}")]
    UnsupportedLineType(String),
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
