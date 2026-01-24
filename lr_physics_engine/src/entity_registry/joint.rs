use std::collections::BTreeMap;

use vector2d::Vector2Df;

use crate::entity_registry::{
    EntityBone, EntityBoneId, EntityState, EntityTemplate, bone,
    entity_template::{MountId, SegmentId},
};

#[derive(Clone, Copy, Debug)]
pub(crate) enum ConnectionType {
    Segments(SegmentId, SegmentId),
    Mounts(MountId, MountId),
    Hybrid(SegmentId, MountId),
}

/// Computed properties when built
#[derive(Debug)]
struct Computed {
    connection_type: ConnectionType,
}

/// Constructed joint that holds props after building
#[derive(Debug)]
pub struct EntityJoint {
    bone_ids: (EntityBoneId, EntityBoneId),
    computed: Computed,
}

impl EntityJoint {
    pub(crate) fn connection_type(&self) -> ConnectionType {
        self.computed.connection_type
    }

    pub(crate) fn should_break(&self, state: &EntityState, template: &EntityTemplate) -> bool {
        let bones = (
            template
                .bones()
                .get(&self.bone_ids.0)
                .expect("Template should have bones needed for this joint"),
            template
                .bones()
                .get(&self.bone_ids.1)
                .expect("Template should have bones needed for this joint"),
        );
        let bone0_p0 = state.point_state(&bones.0.point_ids().0);
        let bone0_p1 = state.point_state(&bones.0.point_ids().1);
        let bone1_p0 = state.point_state(&bones.1.point_ids().0);
        let bone1_p1 = state.point_state(&bones.1.point_ids().1);
        let bone_vectors = (
            bone0_p0.position().vector_from(bone0_p1.position()),
            bone1_p0.position().vector_from(bone1_p1.position()),
        );
        Vector2Df::cross(bone_vectors.0, bone_vectors.1) < 0.0
    }
}

/// Joint builder for custom skeletons
pub struct EntityJointBuilder {
    bone_ids: (EntityBoneId, EntityBoneId),
}

impl EntityJointBuilder {
    pub fn new(b1: EntityBoneId, b2: EntityBoneId) -> EntityJointBuilder {
        Self { bone_ids: (b1, b2) }
    }

    pub(crate) fn build(self, bone_map: &BTreeMap<EntityBoneId, EntityBone>) -> EntityJoint {
        let bones = (
            bone_map
                .get(&self.bone_ids.0)
                .expect("Building this joint should properly resolve builder bones"),
            bone_map
                .get(&self.bone_ids.1)
                .expect("Building this joint should properly resolve builder bones"),
        );

        let connection_type = match bones.0.connection_type() {
            bone::ConnectionType::Mount(mount_id) => match bones.1.connection_type() {
                bone::ConnectionType::Mount(other_mount_id) => {
                    ConnectionType::Mounts(mount_id, other_mount_id)
                }
                bone::ConnectionType::Segment(segment_id) => {
                    ConnectionType::Hybrid(segment_id, mount_id)
                }
            },
            bone::ConnectionType::Segment(segment_id) => match bones.1.connection_type() {
                bone::ConnectionType::Mount(mount_id) => {
                    ConnectionType::Hybrid(segment_id, mount_id)
                }
                bone::ConnectionType::Segment(other_segment_id) => {
                    ConnectionType::Segments(segment_id, other_segment_id)
                }
            },
        };

        EntityJoint {
            bone_ids: self.bone_ids,
            computed: Computed { connection_type },
        }
    }
}

impl From<EntityJoint> for EntityJointBuilder {
    fn from(joint: EntityJoint) -> Self {
        Self {
            bone_ids: joint.bone_ids,
        }
    }
}
