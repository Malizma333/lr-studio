use vector2d::Vector2Df;

use crate::entity::{
    bone::{builder::EntityBoneBuilder, template::EntityBoneTemplate},
    joint::{builder::EntityJointBuilder, template::EntityJointTemplate},
    point::{builder::EntityPointBuilder, template::EntityPointTemplate},
    registry::{
        EntityBoneTemplateId, EntityJointTemplateId, EntityPointTemplateId, EntityRegistry,
        EntitySkeletonTemplateId,
    },
    skeleton::template::EntitySkeletonTemplate,
};

pub struct EntitySkeletonBuilder<'a> {
    registry: &'a mut EntityRegistry,
    points: Vec<EntityPointTemplateId>,
    bones: Vec<EntityBoneTemplateId>,
    joints: Vec<EntityJointTemplateId>,
    remount_enabled: bool,
    dismounted_timer: Option<u32>,
    remounting_timer: Option<u32>,
    remounted_timer: Option<u32>,
}

impl<'a> EntitySkeletonBuilder<'a> {
    pub(crate) fn new(registry: &'a mut EntityRegistry) -> EntitySkeletonBuilder<'a> {
        EntitySkeletonBuilder {
            registry,
            points: Vec::new(),
            bones: Vec::new(),
            joints: Vec::new(),
            remount_enabled: false,
            dismounted_timer: None,
            remounting_timer: None,
            remounted_timer: None,
        }
    }

    pub(crate) fn add_point(
        mut self,
        point_template: EntityPointTemplate,
    ) -> (Self, EntityPointTemplateId) {
        let id = self.registry.add_point_template(point_template);
        self.points.push(id);
        (self, id)
    }

    pub(crate) fn add_bone(
        mut self,
        bone_template: EntityBoneTemplate,
    ) -> (Self, EntityBoneTemplateId) {
        let id = self.registry.add_bone_template(bone_template);
        self.bones.push(id);
        (self, id)
    }

    pub(crate) fn add_joint(
        mut self,
        joint_template: EntityJointTemplate,
    ) -> (Self, EntityJointTemplateId) {
        let id = self.registry.add_joint_template(joint_template);
        self.joints.push(id);
        (self, id)
    }

    pub fn point(self, initial_position: Vector2Df) -> EntityPointBuilder<'a> {
        EntityPointBuilder::new(self, initial_position)
    }

    pub fn bone(
        self,
        p1: EntityPointTemplateId,
        p2: EntityPointTemplateId,
    ) -> EntityBoneBuilder<'a> {
        EntityBoneBuilder::new(self, p1, p2)
    }

    pub fn joint(
        self,
        b1: EntityBoneTemplateId,
        b2: EntityBoneTemplateId,
    ) -> EntityJointBuilder<'a> {
        EntityJointBuilder::new(self, b1, b2)
    }

    pub fn enable_remount(&mut self) -> &mut Self {
        self.remount_enabled = true;
        self
    }

    pub fn dismounted_timer(&mut self, duration: u32) -> &mut Self {
        self.dismounted_timer = Some(duration);
        self
    }

    pub fn remounting_timer(&mut self, duration: u32) -> &mut Self {
        self.remounting_timer = Some(duration);
        self
    }

    pub fn remounted_timer(&mut self, duration: u32) -> &mut Self {
        self.remounted_timer = Some(duration);
        self
    }

    pub fn build(self) -> EntitySkeletonTemplateId {
        let skeleton_template = EntitySkeletonTemplate {
            points: self.points,
            bones: self.bones,
            joints: self.joints,
            remount_enabled: self.remount_enabled,
            dismounted_timer: self.dismounted_timer.unwrap_or(0),
            remounting_timer: self.remounting_timer.unwrap_or(0),
            remounted_timer: self.remounted_timer.unwrap_or(0),
        };
        self.registry.add_skeleton_template(skeleton_template)
    }
}
