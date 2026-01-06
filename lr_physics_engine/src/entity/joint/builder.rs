use crate::entity::{
    joint::template::EntityJointTemplate,
    registry::{EntityBoneTemplateId, EntityJointTemplateId},
    skeleton::builder::EntitySkeletonBuilder,
};

pub struct EntityJointBuilder<'a> {
    skeleton: EntitySkeletonBuilder<'a>,
    bones_involved: (EntityBoneTemplateId, EntityBoneTemplateId),
    mount: bool,
}

impl<'a> EntityJointBuilder<'a> {
    pub fn new(
        skeleton: EntitySkeletonBuilder<'a>,
        b1: EntityBoneTemplateId,
        b2: EntityBoneTemplateId,
    ) -> EntityJointBuilder<'a> {
        Self {
            skeleton,
            bones_involved: (b1, b2),
            mount: false,
        }
    }

    pub fn mount(mut self) -> Self {
        self.mount = true;
        self
    }

    pub fn build(self) -> (EntitySkeletonBuilder<'a>, EntityJointTemplateId) {
        let joint_template = EntityJointTemplate {
            bones_involved: self.bones_involved,
            mount: self.mount,
        };
        self.skeleton.add_joint(joint_template)
    }
}
