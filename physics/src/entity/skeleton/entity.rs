use crate::entity::{
    registry::{EntityBoneId, EntityJointId, EntityPointId},
    skeleton::{MountPhase, snapshot::EntitySkeletonSnapshot},
};

pub(crate) struct EntitySkeleton {
    pub(super) points: Vec<EntityPointId>,
    pub(super) bones: Vec<EntityBoneId>,
    pub(super) joints: Vec<EntityJointId>,
    pub(super) remount_enabled: bool,
    pub(super) dismounted_timer: u32,
    pub(super) remounting_timer: u32,
    pub(super) remounted_timer: u32,
}

impl EntitySkeleton {
    pub(crate) fn points(&self) -> &[EntityPointId] {
        &self.points
    }

    pub(crate) fn bones(&self) -> &[EntityBoneId] {
        &self.bones
    }

    pub(crate) fn joints(&self) -> &[EntityJointId] {
        &self.joints
    }

    pub(crate) fn snapshot(&self) -> EntitySkeletonSnapshot {
        // TODO
        EntitySkeletonSnapshot {
            points: Vec::new(),
            bones: Vec::new(),
            joints: Vec::new(),
            remount_enabled: self.remount_enabled,
            dismounted_timer: self.dismounted_timer,
            remounting_timer: self.remounting_timer,
            remounted_timer: self.remounted_timer,
            mount_phase: MountPhase::Mounted,
            sled_intact: true,
        }
    }
}
