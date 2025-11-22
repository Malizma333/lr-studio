//! This crate provides safe, compatible reading and writing of Line Rider track file formats.
//!
//! Supported formats: SOL (read/write), JSON (read/write), TRK (read)

pub mod formats;
pub mod track;
pub(crate) mod util;
