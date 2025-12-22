#[derive(Debug)]
pub enum RemountVersion {
    None,
    ComV1,
    ComV2,
    LRA,
}

impl Clone for RemountVersion {
    fn clone(&self) -> Self {
        match self {
            RemountVersion::None => RemountVersion::None,
            RemountVersion::ComV1 => RemountVersion::ComV1,
            RemountVersion::ComV2 => RemountVersion::ComV2,
            RemountVersion::LRA => RemountVersion::LRA,
        }
    }
}

impl Copy for RemountVersion {}
