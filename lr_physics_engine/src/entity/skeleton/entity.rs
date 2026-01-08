use crate::entity::{
    registry::{EntityBoneId, EntityJointId, EntityPointId, EntitySkeletonTemplateId},
    skeleton::RemountVersion,
};

pub(crate) struct EntitySkeleton {
    pub(super) points: Vec<EntityPointId>,
    pub(super) bones: Vec<EntityBoneId>,
    pub(super) joints: Vec<EntityJointId>,
    pub(super) dismounted_timer: u32,
    pub(super) remounting_timer: u32,
    pub(super) mounted_timer: u32,
    pub(super) remount_version: RemountVersion,
    pub(super) template_id: EntitySkeletonTemplateId,
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

    pub(crate) fn dismounted_timer(&self) -> u32 {
        self.dismounted_timer
    }

    pub(crate) fn remounting_timer(&self) -> u32 {
        self.remounting_timer
    }

    pub(crate) fn mounted_timer(&self) -> u32 {
        self.mounted_timer
    }

    pub(crate) fn remount_version(&self) -> RemountVersion {
        self.remount_version
    }

    pub(crate) fn template_id(&self) -> EntitySkeletonTemplateId {
        self.template_id
    }

    pub(crate) fn sled_points(&self) -> Vec<EntityPointId> {
        vec![
            self.points[0],
            self.points[1],
            self.points[2],
            self.points[3],
        ]
    }
}
