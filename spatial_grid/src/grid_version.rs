#[derive(Clone, Copy, Debug)]
pub enum GridVersion {
    V6_0,
    V6_1,
    V6_2,
}

impl PartialEq for GridVersion {
    fn eq(&self, other: &Self) -> bool {
        match self {
            GridVersion::V6_0 => match other {
                GridVersion::V6_0 => true,
                _ => false,
            },
            GridVersion::V6_1 => match other {
                GridVersion::V6_1 => true,
                _ => false,
            },
            GridVersion::V6_2 => match other {
                GridVersion::V6_2 => true,
                _ => false,
            },
        }
    }
}
