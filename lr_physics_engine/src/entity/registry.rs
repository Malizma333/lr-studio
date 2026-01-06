use std::collections::{BTreeMap, HashMap};

use crate::entity::{
    bone::{entity::EntityBone, template::EntityBoneTemplate},
    joint::{entity::EntityJoint, template::EntityJointTemplate},
    point::{entity::EntityPoint, template::EntityPointTemplate},
    skeleton::{
        builder::EntitySkeletonBuilder, entity::EntitySkeleton, template::EntitySkeletonTemplate,
    },
};

macro_rules! registry_id {
    ($name:ident) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(usize);
    };
}
registry_id!(EntityPointId);
registry_id!(EntityPointTemplateId);
registry_id!(EntityBoneId);
registry_id!(EntityBoneTemplateId);
registry_id!(EntityJointId);
registry_id!(EntityJointTemplateId);
registry_id!(EntitySkeletonId);
registry_id!(EntitySkeletonTemplateId);

pub struct EntityRegistry {
    points: BTreeMap<EntityPointId, EntityPoint>,
    bones: BTreeMap<EntityBoneId, EntityBone>,
    joints: BTreeMap<EntityJointId, EntityJoint>,
    skeletons: BTreeMap<EntitySkeletonId, EntitySkeleton>,
    point_templates: HashMap<EntityPointTemplateId, EntityPointTemplate>,
    bone_templates: HashMap<EntityBoneTemplateId, EntityBoneTemplate>,
    joint_templates: HashMap<EntityJointTemplateId, EntityJointTemplate>,
    skeleton_templates: HashMap<EntitySkeletonTemplateId, EntitySkeletonTemplate>,
}

// Note: unchecked index access is safe here, because users shouldn't be able to
// construct their own Id structs to pass in (we provide them)
impl EntityRegistry {
    pub(crate) fn new() -> Self {
        Self {
            points: BTreeMap::new(),
            bones: BTreeMap::new(),
            joints: BTreeMap::new(),
            skeletons: BTreeMap::new(),
            point_templates: HashMap::new(),
            bone_templates: HashMap::new(),
            joint_templates: HashMap::new(),
            skeleton_templates: HashMap::new(),
        }
    }

    pub(crate) fn get_point(&self, id: EntityPointId) -> &EntityPoint {
        &self.points[&id]
    }

    pub(crate) fn get_bone(&self, id: EntityBoneId) -> &EntityBone {
        &self.bones[&id]
    }

    pub(crate) fn get_joint(&self, id: EntityJointId) -> &EntityJoint {
        &self.joints[&id]
    }

    pub(crate) fn get_skeleton(&self, id: EntitySkeletonId) -> &EntitySkeleton {
        &self.skeletons[&id]
    }

    pub(super) fn get_point_template(&self, id: EntityPointTemplateId) -> &EntityPointTemplate {
        &self.point_templates[&id]
    }

    pub(crate) fn skeletons(&self) -> &BTreeMap<EntitySkeletonId, EntitySkeleton> {
        &self.skeletons
    }

    pub(super) fn add_point_template(
        &mut self,
        point_template: EntityPointTemplate,
    ) -> EntityPointTemplateId {
        let id = EntityPointTemplateId(self.point_templates.len());
        self.point_templates.insert(id, point_template);
        id
    }

    pub(super) fn add_bone_template(
        &mut self,
        bone_template: EntityBoneTemplate,
    ) -> EntityBoneTemplateId {
        let id = EntityBoneTemplateId(self.bone_templates.len());
        self.bone_templates.insert(id, bone_template);
        id
    }

    pub(super) fn add_joint_template(
        &mut self,
        joint_template: EntityJointTemplate,
    ) -> EntityJointTemplateId {
        let id = EntityJointTemplateId(self.joint_templates.len());
        self.joint_templates.insert(id, joint_template);
        id
    }

    pub(super) fn add_skeleton_template(
        &mut self,
        skeleton_template: EntitySkeletonTemplate,
    ) -> EntitySkeletonTemplateId {
        let id = EntitySkeletonTemplateId(self.skeleton_templates.len());
        self.skeleton_templates.insert(id, skeleton_template);
        id
    }

    fn create_point(&mut self, point_id: &EntityPointTemplateId) -> EntityPointId {
        let target_point_template = &self.point_templates[point_id];
        let point = target_point_template.build();
        let id = EntityPointId(self.points.len());
        self.points.insert(id, point);
        id
    }

    fn create_bone(
        &mut self,
        bone_id: EntityBoneTemplateId,
        point_mapping: &HashMap<EntityPointTemplateId, EntityPointId>,
    ) -> EntityBoneId {
        let target_bone_template = &self.bone_templates[&bone_id];
        let bone = target_bone_template.build(point_mapping, &self);
        let id = EntityBoneId(self.bones.len());
        self.bones.insert(id, bone);
        id
    }

    fn create_joint(
        &mut self,
        joint_id: EntityJointTemplateId,
        bone_mapping: &HashMap<EntityBoneTemplateId, EntityBoneId>,
    ) -> EntityJointId {
        let target_joint_template = &self.joint_templates[&joint_id];
        let joint = target_joint_template.build(bone_mapping);
        let id = EntityJointId(self.joints.len());
        self.joints.insert(id, joint);
        id
    }

    pub fn create_skeleton(
        &mut self,
        skeleton_template_id: EntitySkeletonTemplateId,
    ) -> EntitySkeletonId {
        let points = self.skeleton_templates[&skeleton_template_id]
            .points()
            .clone();
        let mut point_mapping = HashMap::<EntityPointTemplateId, EntityPointId>::new();
        for point_id in points {
            point_mapping.insert(point_id, self.create_point(&point_id));
        }

        let bones = self.skeleton_templates[&skeleton_template_id]
            .bones()
            .clone();
        let mut bone_mapping = HashMap::<EntityBoneTemplateId, EntityBoneId>::new();
        for bone_id in bones {
            bone_mapping.insert(bone_id, self.create_bone(bone_id, &point_mapping));
        }

        let joints = self.skeleton_templates[&skeleton_template_id]
            .joints()
            .clone();
        let mut joint_mapping = HashMap::<EntityJointTemplateId, EntityJointId>::new();
        for joint_id in joints {
            joint_mapping.insert(joint_id, self.create_joint(joint_id, &bone_mapping));
        }

        let target_skeleton_template = &self.skeleton_templates[&skeleton_template_id];
        let skeleton = target_skeleton_template.build(
            &point_mapping,
            &bone_mapping,
            &joint_mapping,
            skeleton_template_id,
        );
        let skeleton_id = EntitySkeletonId(self.skeletons.len());
        self.skeletons.insert(skeleton_id, skeleton);
        skeleton_id
    }

    pub fn skeleton_template_builder(&mut self) -> EntitySkeletonBuilder<'_> {
        EntitySkeletonBuilder::new(self)
    }

    pub fn delete_skeleton(&mut self, id: EntitySkeletonId) {
        let skeleton = self.skeletons.remove(&id).unwrap();

        for joint in skeleton.joints() {
            self.joints.remove(&joint);
        }

        for bone in skeleton.bones() {
            self.bones.remove(&bone);
        }

        for point in skeleton.points() {
            self.points.remove(&point);
        }
    }
}
