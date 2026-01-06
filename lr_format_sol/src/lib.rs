//! Format used by original flash editions of Line Rider, which includes multiple tracks within the same file

mod error;
mod reader;

pub use error::SolReadError;
pub use reader::{get_track_count, read};
