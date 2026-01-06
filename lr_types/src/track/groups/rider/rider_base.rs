use vector2d::Vector2Df;

use crate::track::RemountVersion;

#[derive(Debug, PartialEq)]
pub struct Rider {
    index: u32,
    start_position: Option<Vector2Df>,
    start_velocity: Option<Vector2Df>,
    start_angle: Option<f64>,
    can_remount: Option<bool>,
    remount_version: RemountVersion,
}

impl Rider {
    pub fn start_position(&self) -> Option<Vector2Df> {
        self.start_position
    }

    pub fn start_velocity(&self) -> Option<Vector2Df> {
        self.start_velocity
    }

    pub fn start_angle(&self) -> Option<f64> {
        self.start_angle
    }

    pub fn can_remount(&self) -> Option<bool> {
        self.can_remount
    }

    pub fn remount_version(&self) -> RemountVersion {
        self.remount_version
    }

    pub fn index(&self) -> u32 {
        self.index
    }
}

pub struct RiderBuilder {
    index: u32,
    start_position: Option<Vector2Df>,
    start_velocity: Option<Vector2Df>,
    start_angle: Option<f64>,
    can_remount: Option<bool>,
    remount_version: RemountVersion,
}

impl RiderBuilder {
    pub fn new(remount_version: RemountVersion, index: u32) -> Self {
        RiderBuilder {
            index,
            start_position: None,
            start_velocity: None,
            start_angle: None,
            can_remount: None,
            remount_version,
        }
    }

    pub fn start_position(&mut self, start_position: Vector2Df) -> &mut Self {
        self.start_position = Some(start_position);
        self
    }

    pub fn start_velocity(&mut self, start_velocity: Vector2Df) -> &mut Self {
        self.start_velocity = Some(start_velocity);
        self
    }

    pub fn start_angle(&mut self, start_angle: f64) -> &mut Self {
        self.start_angle = Some(start_angle);
        self
    }

    pub fn remount_version(&mut self, remount_version: RemountVersion) -> &mut Self {
        self.remount_version = remount_version;
        self
    }

    pub(crate) fn build(&self) -> Rider {
        Rider {
            index: self.index,
            start_position: self.start_position,
            start_velocity: self.start_velocity,
            start_angle: self.start_angle,
            remount_version: self.remount_version,
            can_remount: self.can_remount,
        }
    }
}
