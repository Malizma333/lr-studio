use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

use crate::entity::{
    bone::{EntityBone, template::EntityBoneTemplate},
    joint::{EntityJoint, template::EntityJointTemplate},
    point::{EntityPoint, template::EntityPointTemplate},
    skeleton::{EntitySkeleton, template::EntitySkeletonTemplate},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityPointId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityBoneId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityJointId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntitySkeletonId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntityPointTemplateId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntityBoneTemplateId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntityJointTemplateId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntitySkeletonTemplateId(usize);

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

    pub(super) fn get_point_template(&self, id: EntityPointTemplateId) -> &EntityPointTemplate {
        &self.point_templates[&id]
    }

    pub(super) fn get_bone_template(&self, id: EntityBoneTemplateId) -> &EntityBoneTemplate {
        &self.bone_templates[&id]
    }

    pub(super) fn get_joint_template(&self, id: EntityJointTemplateId) -> &EntityJointTemplate {
        &self.joint_templates[&id]
    }

    pub(super) fn add_point(&mut self, point: EntityPoint) -> EntityPointId {
        let id = EntityPointId(self.points.len());
        self.points.insert(id, point);
        id
    }

    pub(super) fn add_bone(&mut self, bone: EntityBone) -> EntityBoneId {
        let id = EntityBoneId(self.bones.len());
        self.bones.insert(id, bone);
        id
    }

    pub(super) fn add_joint(&mut self, joint: EntityJoint) -> EntityJointId {
        let id = EntityJointId(self.joints.len());
        self.joints.insert(id, joint);
        id
    }

    pub(super) fn add_skeleton(&mut self, skeleton: EntitySkeleton) -> EntitySkeletonId {
        let id = EntitySkeletonId(self.skeletons.len());
        self.skeletons.insert(id, skeleton);
        id
    }

    pub fn get_point(&self, id: EntityPointId) -> &EntityPoint {
        &self.points[&id]
    }

    pub fn get_point_mut(&mut self, id: EntityPointId) -> &mut EntityPoint {
        self.points.get_mut(&id).unwrap()
    }

    pub fn get_bone(&self, id: EntityBoneId) -> &EntityBone {
        &self.bones[&id]
    }

    pub fn get_joint(&self, id: EntityJointId) -> &EntityJoint {
        &self.joints[&id]
    }

    pub fn add_skeleton_template(
        &mut self,
        template: EntitySkeletonTemplate,
    ) -> EntitySkeletonTemplateId {
        let id = EntitySkeletonTemplateId(self.skeleton_templates.len());
        self.skeleton_templates.insert(id, template);
        id
    }

    pub fn get_skeleton_template(&self, id: EntitySkeletonTemplateId) -> &EntitySkeletonTemplate {
        &self.skeleton_templates[&id]
    }

    pub fn create_skeleton(&mut self, id: EntitySkeletonTemplateId) {
        let target_skeleton_template = &self.skeleton_templates[&id];

        let mut point_template_mapping: HashMap<EntityPointTemplateId, EntityPointId> =
            HashMap::new();
        let mut bone_template_mapping: HashMap<EntityBoneTemplateId, EntityBoneId> = HashMap::new();
        let mut joint_template_mapping: HashMap<EntityJointTemplateId, EntityJointId> =
            HashMap::new();

        for point_template_id in target_skeleton_template.points() {
            let new_point = self.get_point_template(*point_template_id).build();
            let new_point_id = self.add_point(new_point);
            point_template_mapping.insert(point_template_id.clone(), new_point_id);
        }

        for bone_template_id in target_skeleton_template.bones() {
            let new_bone = self
                .get_bone_template(*bone_template_id)
                .build(self, &point_template_mapping);
            let new_bone_id = self.add_bone(new_bone);
            bone_template_mapping.insert(bone_template_id.clone(), new_bone_id);
        }

        for joint_template_id in target_skeleton_template.joints() {
            let new_joint = self
                .get_joint_template(*joint_template_id)
                .build(&bone_template_mapping);
            let new_joint_id = self.add_joint(new_joint);
            joint_template_mapping.insert(joint_template_id.clone(), new_joint_id);
        }

        let new_skeleton = target_skeleton_template.build(
            &point_template_mapping,
            &bone_template_mapping,
            &joint_template_mapping,
        );

        let id = EntitySkeletonId(self.skeletons.len());
        self.skeletons.insert(id, new_skeleton);
    }

    pub fn get_skeleton(&self, id: EntitySkeletonId) -> &EntitySkeleton {
        &self.skeletons[&id]
    }

    pub fn list_skeletons(&self) -> Vec<EntitySkeletonId> {
        self.skeletons.keys().cloned().collect()
    }

    pub fn remove_skeleton(&mut self, id: EntitySkeletonId) {
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
