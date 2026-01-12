#[derive(Debug, Clone, Copy)]
pub enum RemountVersion {
    None,
    ComV1,
    ComV2,
    LRA,
}

impl RemountVersion {
    pub fn is_none(&self) -> bool {
        match self {
            RemountVersion::None => true,
            _ => false,
        }
    }

    pub fn is_comv1(&self) -> bool {
        match self {
            RemountVersion::ComV1 => true,
            _ => false,
        }
    }

    pub fn is_comv2(&self) -> bool {
        match self {
            RemountVersion::ComV2 => true,
            _ => false,
        }
    }

    pub fn is_lra(&self) -> bool {
        match self {
            RemountVersion::LRA => true,
            _ => false,
        }
    }
}
