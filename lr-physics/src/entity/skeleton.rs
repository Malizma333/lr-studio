use crate::entity::{
    registry::{EntityBoneId, EntityJointId, EntityPointId},
    skeleton::state::{EntitySkeletonState, MountPhase},
};

pub(crate) mod snapshot;
pub(crate) mod state;
pub(crate) mod template;

const REMOUNT_STRENGTH_FACTOR: f64 = 0.1;
const LRA_REMOUNT_STRENGTH_FACTOR: f64 = 0.5;

pub struct EntitySkeleton {
    points: Vec<EntityPointId>,
    bones: Vec<EntityBoneId>,
    joints: Vec<EntityJointId>,
    remount_enabled: bool,
    dismounted_timer: u32,
    remounting_timer: u32,
    remounted_timer: u32,
    state: EntitySkeletonState,
}

impl EntitySkeleton {
    pub fn is_remounting(&self) -> bool {
        matches!(
            self.state.mount_phase,
            MountPhase::Remounting {
                frames_until_remounted: _
            }
        )
    }

    pub fn is_mounted(&self) -> bool {
        matches!(self.state.mount_phase, MountPhase::Mounted) || self.is_remounting()
    }

    pub fn dismount(&mut self) {
        // Currently does the same thing as destroy
        self.state.mount_phase = MountPhase::Dismounted {
            frames_until_can_remount: None,
        }
    }

    pub fn destroy(&mut self) {
        self.state.mount_phase = MountPhase::Dismounted {
            frames_until_can_remount: None,
        }
    }

    pub fn points(&self) -> &Vec<EntityPointId> {
        &self.points
    }

    pub fn bones(&self) -> &Vec<EntityBoneId> {
        &self.bones
    }

    pub fn joints(&self) -> &Vec<EntityJointId> {
        &self.joints
    }

    pub fn mount_bones(&self) -> &Vec<EntityBoneId> {
        todo!()
    }

    pub fn mount_joints(&self) -> &Vec<EntityJointId> {
        todo!()
    }
}
