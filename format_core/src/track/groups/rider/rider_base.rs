use crate::track::{RemountVersion, Vec2};

pub struct Rider {
    start_position: Option<Vec2>,
    start_velocity: Option<Vec2>,
    start_angle: Option<f64>,
    can_remount: Option<bool>,
    remount_version: RemountVersion,
}

impl Rider {
    pub fn start_position(&self) -> Option<Vec2> {
        self.start_position
    }

    pub fn start_velocity(&self) -> Option<Vec2> {
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
}

pub struct RiderBuilder {
    start_position: Option<Vec2>,
    start_velocity: Option<Vec2>,
    start_angle: Option<f64>,
    can_remount: Option<bool>,
    remount_version: RemountVersion,
}

impl RiderBuilder {
    pub fn new(remount_version: RemountVersion) -> Self {
        RiderBuilder {
            start_position: None,
            start_velocity: None,
            start_angle: None,
            can_remount: None,
            remount_version,
        }
    }

    pub fn start_position(&mut self, start_position: Option<Vec2>) -> &mut Self {
        self.start_position = start_position;
        self
    }

    pub fn start_velocity(&mut self, start_velocity: Option<Vec2>) -> &mut Self {
        self.start_velocity = start_velocity;
        self
    }

    pub fn start_angle(&mut self, start_angle: Option<f64>) -> &mut Self {
        self.start_angle = start_angle;
        self
    }

    pub fn can_remount(&mut self, can_remount: Option<bool>) -> &mut Self {
        self.can_remount = can_remount;
        self
    }

    pub fn remount_version(&mut self, remount_version: RemountVersion) -> &mut Self {
        self.remount_version = remount_version;
        self
    }

    pub fn build(&self) -> Rider {
        Rider {
            start_position: self.start_position,
            start_velocity: self.start_velocity,
            start_angle: self.start_angle,
            remount_version: self.remount_version,
            can_remount: self.can_remount,
        }
    }
}
