use crate::entity::{bone::EntityBone, joint::EntityJoint, point::EntityPoint};

// TODO data structure w/ operations:
// - adding and removing individual skeletons
// - adding and removing mounts
// - caching point states for each frame (basic clone)
// - caching mount states for each frame (basic clone)
// - removing from and adding to cache size whenever skeletons/mounts change
// - clearing front of cache whenever lines or skeletons (remounting) change
// - hot swapping cached point states whenever frame info requested

struct EntityRegistry {
    points: Vec<EntityPoint>,
    bones: Vec<EntityBone>,
    joints: Vec<EntityJoint>,
}
