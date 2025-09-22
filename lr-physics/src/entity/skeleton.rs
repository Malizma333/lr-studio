use crate::entity::{bone::EntityBoneIndex, joint::EntityJointIndex, point::EntityPointIndex};

pub struct Skeleton {
    points: Vec<EntityPointIndex>,
    bones: Vec<EntityBoneIndex>,
    joints: Vec<EntityJointIndex>,
}
