use crate::track::{
    RemountVersion,
    rider::rider_base::{Rider, RiderBuilder},
};

pub struct RiderGroup {
    riders: Vec<Rider>,
}

impl RiderGroup {
    pub fn riders(&self) -> &Vec<Rider> {
        &self.riders
    }
}

pub struct RiderGroupBuilder {
    riders: Vec<RiderBuilder>,
}

impl RiderGroupBuilder {
    pub fn new() -> Self {
        RiderGroupBuilder { riders: Vec::new() }
    }

    pub fn add_rider(&mut self, remount_version: RemountVersion) -> &mut RiderBuilder {
        self.riders.push(RiderBuilder::new(remount_version));
        self.riders.last_mut().unwrap()
    }

    pub fn get_riders(&mut self) -> &mut Vec<RiderBuilder> {
        &mut self.riders
    }

    pub fn build(&self) -> Option<RiderGroup> {
        let mut riders: Vec<Rider> = vec![];

        for rider_builder in &self.riders {
            let rider = rider_builder.build();
            riders.push(rider);
        }

        if riders.len() == 0 {
            None
        } else {
            Some(RiderGroup { riders })
        }
    }
}
