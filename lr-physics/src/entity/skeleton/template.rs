use std::collections::HashMap;

use crate::entity::{
    registry::{
        EntityBoneId, EntityBoneTemplateId, EntityJointId, EntityJointTemplateId, EntityPointId,
        EntityPointTemplateId,
    },
    skeleton::{
        EntitySkeleton,
        state::{EntitySkeletonState, MountPhase},
    },
};

pub(crate) struct EntitySkeletonTemplate {
    points: Vec<EntityPointTemplateId>,
    bones: Vec<EntityBoneTemplateId>,
    joints: Vec<EntityJointTemplateId>,
    remount_enabled: bool,
    dismounted_timer: Option<u32>,
    remounting_timer: Option<u32>,
    remounted_timer: Option<u32>,
}

impl EntitySkeletonTemplate {
    pub fn new() -> EntitySkeletonTemplate {
        EntitySkeletonTemplate {
            points: Vec::new(),
            bones: Vec::new(),
            joints: Vec::new(),
            remount_enabled: false,
            dismounted_timer: None,
            remounting_timer: None,
            remounted_timer: None,
        }
    }

    pub fn add_point(&mut self, id: EntityPointTemplateId) {
        self.points.push(id);
    }

    pub fn add_bone(&mut self, id: EntityBoneTemplateId) {
        self.bones.push(id);
    }

    pub fn add_joint(&mut self, id: EntityJointTemplateId) {
        self.joints.push(id);
    }

    pub fn enable_remount(&mut self) {
        self.remount_enabled = true;
    }

    pub fn time_until_dismounted(&mut self, limit: u32) {
        self.dismounted_timer = Some(limit);
    }

    pub fn time_until_remounting(&mut self, limit: u32) {
        self.remounting_timer = Some(limit);
    }

    pub fn time_until_remounted(&mut self, limit: u32) {
        self.remounted_timer = Some(limit);
    }

    pub(super) fn points(&self) -> &Vec<EntityPointTemplateId> {
        &self.points
    }

    pub(super) fn bones(&self) -> &Vec<EntityBoneTemplateId> {
        &self.bones
    }

    pub(super) fn joints(&self) -> &Vec<EntityJointTemplateId> {
        &self.joints
    }

    pub fn build(
        &self,
        point_map: &HashMap<EntityPointTemplateId, EntityPointId>,
        bone_map: &HashMap<EntityBoneTemplateId, EntityBoneId>,
        joint_map: &HashMap<EntityJointTemplateId, EntityJointId>,
    ) -> EntitySkeleton {
        EntitySkeleton {
            points: self
                .points
                .iter()
                .map(|point_template_id| point_map[point_template_id])
                .collect(),
            bones: self
                .bones
                .iter()
                .map(|bone_template_id| bone_map[bone_template_id])
                .collect(),
            joints: self
                .joints
                .iter()
                .map(|joint_template_id| joint_map[joint_template_id])
                .collect(),
            remount_enabled: self.remount_enabled,
            dismounted_timer: self.dismounted_timer.unwrap_or(0),
            remounting_timer: self.remounting_timer.unwrap_or(0),
            remounted_timer: self.remounted_timer.unwrap_or(0),
            state: EntitySkeletonState {
                mount_phase: MountPhase::Mounted,
                intact: true,
            },
        }
    }
}
