use std::time::{SystemTime, UNIX_EPOCH};

const A: u64 = 6364136223846793005;
const ALPHANUMERIC_CHARS: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z',
];

#[derive(Debug)]
pub enum RandomError {
    InvalidBounds,
    EmptySlice,
}

/// Implementation of a Linear Congruential Generator for basic pseudo-random number generation
pub struct Random {
    state: u64,
}

impl Random {
    pub fn new() -> Self {
        let seed: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        Self { state: seed }
    }

    pub fn from_seed(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(A).wrapping_add(1);
        (self.state >> 32) as u32
    }

    /// Generates a pseudo-random number in the range [0,1)
    pub fn rand(&mut self) -> f64 {
        f64::from(self.next()) / (f64::from(u32::MAX) + 1.0)
    }

    /// Generates a random f64 in the range [lower, upper)
    pub fn rand_range_f64(&mut self, lower: f64, upper: f64) -> Result<f64, RandomError> {
        if lower > upper {
            Err(RandomError::InvalidBounds)
        } else {
            Ok((upper - lower) * self.rand() + lower)
        }
    }

    /// Generates a random u32 in the range [lower, upper)
    pub fn rand_range_u32(&mut self, lower: u32, upper: u32) -> Result<u32, RandomError> {
        if lower > upper {
            Err(RandomError::InvalidBounds)
        } else {
            Ok(self.next() % (upper - lower) + lower)
        }
    }

    /// Generates a random i32 in the range [lower, upper)
    pub fn rand_range_i32(&mut self, lower: i32, upper: i32) -> Result<i32, RandomError> {
        if lower > upper {
            Err(RandomError::InvalidBounds)
        } else {
            Ok((self.next() % ((upper - lower) as u32)) as i32 + lower)
        }
    }

    /// Retrieves a random item from a given slice
    pub fn rand_choice<'a, T>(&mut self, items: &'a [T]) -> Result<&'a T, RandomError> {
        if items.is_empty() {
            Err(RandomError::EmptySlice)
        } else {
            let index = self.rand_range_u32(0, items.len() as u32).unwrap() as usize;
            Ok(&items[index.min(items.len() - 1)])
        }
    }

    /// Generates a random string of alphanumeric characters
    pub fn rand_str(&mut self, length: usize) -> String {
        let mut result = String::new();
        for _ in 0..length {
            let char = self.rand_choice(ALPHANUMERIC_CHARS.as_slice());
            result.push(*char.unwrap());
        }
        result
    }
}
