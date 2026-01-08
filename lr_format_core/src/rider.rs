use vector2d::Vector2Df;

use crate::RemountVersion;

#[derive(Debug, PartialEq)]
pub struct Rider {
    start_offset: Option<Vector2Df>,
    start_velocity: Option<Vector2Df>,
    remount_version: RemountVersion,
}

impl Rider {
    pub fn new(remount_version: RemountVersion) -> Self {
        Self {
            start_offset: None,
            start_velocity: None,
            remount_version,
        }
    }

    pub fn start_offset(&self) -> Option<Vector2Df> {
        self.start_offset
    }

    pub fn set_start_offset(&mut self, start_offset: Vector2Df) {
        self.start_offset = Some(start_offset);
    }

    pub fn start_velocity(&self) -> Option<Vector2Df> {
        self.start_velocity
    }

    pub fn set_start_velocity(&mut self, start_velocity: Vector2Df) {
        self.start_velocity = Some(start_velocity);
    }

    pub fn remount_version(&self) -> RemountVersion {
        self.remount_version
    }

    pub fn set_remount_version(&mut self, remount_version: RemountVersion) {
        self.remount_version = remount_version;
    }
}
