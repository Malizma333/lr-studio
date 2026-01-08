use crate::entity::{
    registry::{
        EntityBoneId, EntityBoneTemplateId, EntityJointId, EntityJointTemplateId, EntityPointId,
        EntityPointTemplateId, EntitySkeletonTemplateId,
    },
    skeleton::{RemountVersion, entity::EntitySkeleton},
};
use std::collections::HashMap;

pub(crate) struct EntitySkeletonTemplate {
    pub(super) points: Vec<EntityPointTemplateId>,
    pub(super) bones: Vec<EntityBoneTemplateId>,
    pub(super) joints: Vec<EntityJointTemplateId>,
    pub(super) dismounted_timer: u32,
    pub(super) remounting_timer: u32,
    pub(super) mounted_timer: u32,
    pub(super) remount_version: RemountVersion,
}

impl EntitySkeletonTemplate {
    pub(crate) fn points(&self) -> &Vec<EntityPointTemplateId> {
        &self.points
    }

    pub(crate) fn bones(&self) -> &Vec<EntityBoneTemplateId> {
        &self.bones
    }

    pub(crate) fn joints(&self) -> &Vec<EntityJointTemplateId> {
        &self.joints
    }

    pub(crate) fn build(
        &self,
        point_mapping: &HashMap<EntityPointTemplateId, EntityPointId>,
        bone_mapping: &HashMap<EntityBoneTemplateId, EntityBoneId>,
        joint_mapping: &HashMap<EntityJointTemplateId, EntityJointId>,
        template_id: EntitySkeletonTemplateId,
    ) -> EntitySkeleton {
        EntitySkeleton {
            points: self
                .points
                .iter()
                .map(|point_template_id| point_mapping[point_template_id])
                .collect(),
            bones: self
                .bones
                .iter()
                .map(|bone_template_id| bone_mapping[bone_template_id])
                .collect(),
            joints: self
                .joints
                .iter()
                .map(|joint_template_id| joint_mapping[joint_template_id])
                .collect(),
            dismounted_timer: self.dismounted_timer,
            remounting_timer: self.remounting_timer,
            mounted_timer: self.mounted_timer,
            remount_version: self.remount_version,
            template_id,
        }
    }
}
