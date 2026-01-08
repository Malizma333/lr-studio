use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum GridVersion {
    V6_0,
    V6_1,
    V6_2,
}

impl fmt::Display for GridVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridVersion::V6_0 => f.write_str("GridVersion::V6_0"),
            GridVersion::V6_1 => f.write_str("GridVersion::V6_1"),
            GridVersion::V6_2 => f.write_str("GridVersion::V6_2"),
        }
    }
}
