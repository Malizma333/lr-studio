use vector2d::Vector2Df;

use crate::RemountVersion;

#[derive(Debug, PartialEq)]
pub struct Rider {
    start_offset: Option<Vector2Df>,
    start_velocity: Option<Vector2Df>,
    remount_version: RemountVersion,
}

impl Rider {
    pub fn start_offset(&self) -> Option<Vector2Df> {
        self.start_offset
    }

    pub fn start_velocity(&self) -> Option<Vector2Df> {
        self.start_velocity
    }

    pub fn remount_version(&self) -> RemountVersion {
        self.remount_version
    }
}

#[derive(Debug, PartialEq)]
pub struct RiderBuilder {
    start_offset: Option<Vector2Df>,
    start_velocity: Option<Vector2Df>,
    remount_version: RemountVersion,
}

impl RiderBuilder {
    pub fn new(remount_version: RemountVersion) -> Self {
        Self {
            start_offset: None,
            start_velocity: None,
            remount_version,
        }
    }

    pub fn start_offset(&mut self, start_offset: Vector2Df) -> &mut Self {
        self.start_offset = Some(start_offset);
        self
    }

    pub fn start_velocity(&mut self, start_velocity: Vector2Df) -> &mut Self {
        self.start_velocity = Some(start_velocity);
        self
    }

    pub fn remount_version(&mut self, remount_version: RemountVersion) -> &mut Self {
        self.remount_version = remount_version;
        self
    }

    pub fn build(self) -> Rider {
        Rider {
            start_offset: self.start_offset,
            start_velocity: self.start_velocity,
            remount_version: self.remount_version,
        }
    }
}

impl From<Rider> for RiderBuilder {
    fn from(rider: Rider) -> Self {
        RiderBuilder {
            start_offset: rider.start_offset,
            start_velocity: rider.start_velocity,
            remount_version: rider.remount_version,
        }
    }
}
