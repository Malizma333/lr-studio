use crate::entity::{
    bone::snapshot::EntityBoneSnapshot, joint::snapshot::EntityJointSnapshot,
    point::snapshot::EntityPointSnapshot, skeleton::MountPhase,
};

pub(crate) struct EntitySkeletonSnapshot {
    pub(super) points: Vec<EntityPointSnapshot>,
    pub(super) bones: Vec<EntityBoneSnapshot>,
    pub(super) joints: Vec<EntityJointSnapshot>,
    pub(super) remount_enabled: bool,
    pub(super) dismounted_timer: u32,
    pub(super) remounting_timer: u32,
    pub(super) remounted_timer: u32,
    pub(super) mount_phase: MountPhase,
    pub(super) sled_intact: bool,
}

impl EntitySkeletonSnapshot {
    fn is_remounting(&self) -> bool {
        matches!(
            self.mount_phase,
            MountPhase::Remounting {
                frames_until_remounted: _
            }
        )
    }

    fn is_mounted(&self) -> bool {
        matches!(self.mount_phase, MountPhase::Mounted)
    }
}
