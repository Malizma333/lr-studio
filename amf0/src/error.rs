// <https://github.com/KallDrexx/rust-media-libs>
// License: See ../LICENSE-APACHE and ../LICENSE-MIT
// Modifications Copyright 2026 Tobias Bessler

use std::{error::Error, fmt, io, string::FromUtf8Error};

// Errors that can occur during the deserialization process
#[derive(Debug)]
pub enum DeserializationError {
    // Every Amf0 value starts with a marker byte describing the type of value that was
    // encoded.  For example a marker of `0x00` is a number, `0x01` is a string, etc..
    //
    // This error is encountered when we see a maker value that we do not recognize.
    UnknownMarker { marker: u8 },

    // Object properties consist of a name and value pair.  It is expected that every property
    // has a valid string name, and if the name is empty this error is raised.
    UnexpectedEmptyObjectPropertyName,

    // This occurs when we are expecting more data but hit the end of the buffer
    // (e.g. we are reading an object property but there was no property value).
    UnexpectedEof,

    Other(Box<dyn Error + Send + Sync>),
}

impl fmt::Display for DeserializationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::UnknownMarker { marker } => write!(f, "Encountered unknown marker: {}", marker),
            Self::UnexpectedEmptyObjectPropertyName => {
                write!(f, "Unexpected empty object property name")
            }
            Self::UnexpectedEof => {
                write!(f, "Hit end of the byte buffer but was expecting more data")
            }
            Self::Other(e) => write!(f, "Other error occurred: {}", e),
        }
    }
}

impl Error for DeserializationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            DeserializationError::Other(e) => Some(&**e),
            _ => None,
        }
    }
}

impl From<io::Error> for DeserializationError {
    fn from(value: io::Error) -> Self {
        DeserializationError::Other(Box::new(value))
    }
}

impl From<FromUtf8Error> for DeserializationError {
    fn from(value: FromUtf8Error) -> Self {
        DeserializationError::Other(Box::new(value))
    }
}

// Errors raised during to the serialization process
#[derive(Debug)]
pub enum SerializationError {
    // Amf0 strings cannot be more than 65,535 characters, so if a string was provided
    // with a larger length than this than this error is raised.
    NormalStringTooLong,

    Other(Box<dyn Error + Send + Sync>),
}

impl fmt::Display for SerializationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::NormalStringTooLong => {
                write!(f, "String length greater than 65,535")
            }
            Self::Other(e) => {
                write!(f, "Other error occurred: {}", e)
            }
        }
    }
}

impl Error for SerializationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            SerializationError::Other(e) => Some(&**e),
            _ => None,
        }
    }
}

impl From<io::Error> for SerializationError {
    fn from(value: io::Error) -> Self {
        SerializationError::Other(Box::new(value))
    }
}
