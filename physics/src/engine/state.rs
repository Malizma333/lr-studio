use std::collections::BTreeMap;

use crate::entity::{
    bone::state::EntityBoneState,
    joint::state::EntityJointState,
    point::state::EntityPointState,
    registry::{EntityBoneId, EntityJointId, EntityPointId, EntitySkeletonId},
    skeleton::state::EntitySkeletonState,
};

pub struct EngineState {
    point_states: BTreeMap<EntityPointId, EntityPointState>,
    bone_states: BTreeMap<EntityBoneId, EntityBoneState>,
    joint_states: BTreeMap<EntityJointId, EntityJointState>,
    skeleton_states: BTreeMap<EntitySkeletonId, EntitySkeletonState>,
}

impl Clone for EngineState {
    fn clone(&self) -> Self {
        Self {
            point_states: self.point_states.clone(),
            bone_states: self.bone_states.clone(),
            joint_states: self.joint_states.clone(),
            skeleton_states: self.skeleton_states.clone(),
        }
    }
}

impl EngineState {
    pub fn new() -> Self {
        Self {
            point_states: BTreeMap::new(),
            bone_states: BTreeMap::new(),
            joint_states: BTreeMap::new(),
            skeleton_states: BTreeMap::new(),
        }
    }

    pub fn skeletons(&self) -> Vec<&EntitySkeletonState> {
        self.skeleton_states
            .values()
            .collect::<Vec<&EntitySkeletonState>>()
    }
}
