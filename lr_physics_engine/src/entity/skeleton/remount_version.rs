use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum RemountVersion {
    None,
    ComV1,
    ComV2,
    LRA,
}

impl Display for RemountVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RemountVersion::None => f.write_str("RemountVersion::None"),
            RemountVersion::ComV1 => f.write_str("RemountVersion::ComV1"),
            RemountVersion::ComV2 => f.write_str("RemountVersion::ComV2"),
            RemountVersion::LRA => f.write_str("RemountVersion::LRA"),
        }
    }
}
