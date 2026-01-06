//! Format used by [Line Rider: Advanced](https://github.com/jealouscloud/linerider-advanced) and its forks

mod error;
mod reader;

pub use error::TrkReadError;
pub use reader::read;
