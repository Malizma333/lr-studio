/// Implementation of a Linear Congruential Generator for basic pseudo-random number generation
/// (This is not cryptographically secure and is just used for testing purposes)
mod random;
pub use random::{Random, RandomError};
