use geometry::Point;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    iter::zip,
};

use crate::entity_registry::{
    EntityBone, EntityBoneBuilder, EntityJoint, EntityJointBuilder, EntityPoint,
    EntityPointBuilder, MountPhase, RemountVersion, bone,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityPointId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityBoneId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityJointId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// Node in a mount connection graph
pub struct SegmentId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// Edge in a mount connection graph
pub struct MountId(usize);

/// Mapping from a segment id to all the mounts that segment belongs to (node: edges)
type SegmentToMountsMap = BTreeMap<SegmentId, BTreeSet<MountId>>;

/// Mapping from a mount id to the two segments that the mount connects (edge: nodes)
type MountToSegmentsMap = BTreeMap<MountId, (SegmentId, SegmentId)>;

pub struct EntityTemplate {
    points: BTreeMap<EntityPointId, EntityPoint>,
    bones: BTreeMap<EntityBoneId, EntityBone>,
    joints: BTreeMap<EntityJointId, EntityJoint>,
    segment_mounts: SegmentToMountsMap,
    mount_segments: MountToSegmentsMap,
    dismounted_timer: u32,
    remounting_timer: u32,
    mounted_timer: u32,
    remount_version: RemountVersion,
}

impl EntityTemplate {
    pub(crate) fn points(&self) -> &BTreeMap<EntityPointId, EntityPoint> {
        &self.points
    }

    pub(crate) fn bones(&self) -> &BTreeMap<EntityBoneId, EntityBone> {
        &self.bones
    }

    pub(crate) fn joints(&self) -> &BTreeMap<EntityJointId, EntityJoint> {
        &self.joints
    }

    pub(crate) fn remounting_timer(&self) -> u32 {
        self.remounting_timer
    }

    pub(crate) fn mounted_timer(&self) -> u32 {
        self.mounted_timer
    }

    pub(crate) fn remount_version(&self) -> RemountVersion {
        self.remount_version
    }

    pub(crate) fn segment_mounts(&self) -> &SegmentToMountsMap {
        &self.segment_mounts
    }

    pub(crate) fn mount_segments(&self) -> &MountToSegmentsMap {
        &self.mount_segments
    }

    pub(crate) fn get_phase_after_dismount(&self, mount_phase: MountPhase) -> MountPhase {
        match self.remount_version() {
            RemountVersion::None => MountPhase::Dismounted {
                frames_until_remounting: 0,
            },
            _ => {
                if mount_phase.is_mounted() {
                    MountPhase::Dismounting {
                        frames_until_dismounted: self.dismounted_timer,
                    }
                } else if mount_phase.is_remounting() {
                    MountPhase::Dismounted {
                        frames_until_remounting: self.remounting_timer,
                    }
                } else {
                    mount_phase
                }
            }
        }
    }

    pub(crate) fn get_segment_points(&self, segment_id: SegmentId) -> BTreeSet<EntityPointId> {
        let mut points = BTreeSet::new();

        // Loop through bones to find all contact points associated with a particular segment id
        for bone in self.bones.values() {
            if let bone::ConnectionType::Segment(other_segment_id) = bone.connection_type()
                && segment_id == other_segment_id
            {
                points.insert(bone.point_ids().0);
                points.insert(bone.point_ids().1);
            }
        }

        points
    }

    pub(crate) fn is_segment_remounting(
        &self,
        mount_phases: &BTreeMap<MountId, MountPhase>,
        segment_id: SegmentId,
    ) -> bool {
        // Unbreakable bones within a segment are considered remounting if any of the mounts connected to that segment are in remount phase
        // This is somewhat unintuitive but generalizes LRA compatibility
        let mut is_remounting = false;

        for mount_id in self
            .segment_mounts
            .get(&segment_id)
            .expect("Segments should have their connected mounts initialized")
        {
            let mount_phase = mount_phases
                .get(mount_id)
                .expect("Mounts should have a corresponding mount phase");
            is_remounting = is_remounting || mount_phase.is_remounting();
        }

        is_remounting
    }
}

pub struct EntityTemplateBuilder {
    points: BTreeMap<EntityPointId, EntityPointBuilder>,
    bones: BTreeMap<EntityBoneId, EntityBoneBuilder>,
    joints: BTreeMap<EntityJointId, EntityJointBuilder>,
    dismounted_timer: u32,
    remounting_timer: u32,
    mounted_timer: u32,
    remount_version: RemountVersion,
}

impl Default for EntityTemplateBuilder {
    fn default() -> Self {
        EntityTemplateBuilder::new()
    }
}

impl EntityTemplateBuilder {
    pub fn new() -> EntityTemplateBuilder {
        EntityTemplateBuilder {
            points: BTreeMap::new(),
            bones: BTreeMap::new(),
            joints: BTreeMap::new(),
            dismounted_timer: 0,
            remounting_timer: 0,
            mounted_timer: 0,
            remount_version: RemountVersion::None,
        }
    }

    pub fn add_point(&mut self, point: EntityPointBuilder) -> EntityPointId {
        let highest_id = self
            .points
            .last_key_value()
            .map(|entry| entry.0.0 + 1)
            .unwrap_or(0);
        let id = EntityPointId(highest_id);
        self.points.insert(id, point);
        id
    }

    // TODO support removing points, which should also removed the bones and joints involving that point
    // This would also make undoing that action more complex than simply re-adding that point

    pub fn add_bone(&mut self, bone: EntityBoneBuilder) -> EntityBoneId {
        let highest_id = self
            .bones
            .last_key_value()
            .map(|entry| entry.0.0 + 1)
            .unwrap_or(0);
        let id = EntityBoneId(highest_id);
        self.bones.insert(id, bone);
        id
    }

    pub fn add_joint(&mut self, joint: EntityJointBuilder) -> EntityJointId {
        let highest_id = self
            .joints
            .last_key_value()
            .map(|entry| entry.0.0 + 1)
            .unwrap_or(0);
        let id = EntityJointId(highest_id);
        self.joints.insert(id, joint);
        id
    }

    pub fn dismounted_timer(mut self, duration: u32) -> Self {
        self.dismounted_timer = duration;
        self
    }

    pub fn remounting_timer(mut self, duration: u32) -> Self {
        self.remounting_timer = duration;
        self
    }

    pub fn mounted_timer(mut self, duration: u32) -> Self {
        self.mounted_timer = duration;
        self
    }

    pub fn remount_version(mut self, remount_version: RemountVersion) -> Self {
        self.remount_version = remount_version;
        self
    }

    fn calculate_graph(
        &self,
    ) -> (
        SegmentToMountsMap,
        MountToSegmentsMap,
        Vec<bone::ConnectionType>,
    ) {
        // Create a child -> parent map representation of a tree
        // This will act as a disjoint-set structure
        let mut parents: BTreeMap<EntityPointId, Option<EntityPointId>> = BTreeMap::new();
        for point_id in self.points.keys() {
            parents.insert(*point_id, None);
        }

        // Loop through every unbreakable bone to construction the disjoint set
        // The resulting groups are the segments (groups of points connected by unbreakable bones)
        for bone in self.bones.values() {
            if !bone.breakable() {
                let mut root0 = bone.point_ids().0;
                while let Some(parent_node) = parents
                    .get(&root0)
                    .expect("Bone should contain valid point")
                {
                    root0 = *parent_node;
                }
                let mut root1 = bone.point_ids().1;
                while let Some(parent_node) = parents
                    .get(&root1)
                    .expect("Bone should contain valid point")
                {
                    root1 = *parent_node;
                }
                if root0 < root1 {
                    parents.insert(root0, Some(root1));
                }
                if root0 > root1 {
                    parents.insert(root1, Some(root0));
                }
            }
        }

        // Add root nodes to a map of root point id to segment id
        // Also initialize segments map
        let mut roots: HashMap<EntityPointId, SegmentId> = HashMap::new();
        let mut segment_mounts: SegmentToMountsMap = BTreeMap::new();
        for (point_id, parent_point_id) in &parents {
            if parent_point_id.is_none() {
                let next_id = SegmentId(roots.len());
                roots.insert(*point_id, next_id);
                segment_mounts.insert(next_id, BTreeSet::new());
            }
        }

        // Loop through every bone to determine connection types for each
        // As well as compute segment and mount maps
        let mut mount_segments: MountToSegmentsMap = BTreeMap::new();
        let mut connections: Vec<bone::ConnectionType> = Vec::new();
        let mut segment_pairs: HashMap<(SegmentId, SegmentId), MountId> = HashMap::new();

        for bone in self.bones.values() {
            // Retrieve the segments this bone connects based on its points
            let mut root0 = bone.point_ids().0;
            while let Some(parent_node) = parents
                .get(&root0)
                .expect("Bone should contain valid point")
            {
                root0 = *parent_node;
            }
            let mut root1 = bone.point_ids().1;
            while let Some(parent_node) = parents
                .get(&root1)
                .expect("Bone should contain valid point")
            {
                root1 = *parent_node;
            }
            let root_indices = (
                roots
                    .get(&root0)
                    .expect("Roots should have been initialized correctly"),
                roots
                    .get(&root1)
                    .expect("Roots should have been initialized correctly"),
            );
            let segment_ids = (
                *root_indices.0.min(root_indices.1),
                *root_indices.0.max(root_indices.1),
            );

            if bone.breakable() {
                let next_id = if segment_ids.0 == segment_ids.1 {
                    // Breakable bone, same segment
                    // Belongs to a unique mount
                    MountId(mount_segments.len())
                } else {
                    // Breakable bone, different segments
                    // Create a new mount between these if one hasn't been created
                    if let Some(mount_id) = segment_pairs.get(&segment_ids) {
                        *mount_id
                    } else {
                        let next_id = MountId(mount_segments.len());
                        segment_pairs.insert(segment_ids, next_id);
                        next_id
                    }
                };
                mount_segments.insert(next_id, segment_ids);
                segment_mounts
                    .get_mut(&segment_ids.0)
                    .expect("Segment mounts should have been initialized correctly")
                    .insert(next_id);
                segment_mounts
                    .get_mut(&segment_ids.1)
                    .expect("Segment mounts should have been initialized correctly")
                    .insert(next_id);

                connections.push(bone::ConnectionType::Mount(next_id));
            } else {
                // Unbreakable bone, just create segment type connection
                // Must necessarily be same segment
                connections.push(bone::ConnectionType::Segment(segment_ids.0));
            }
        }

        (segment_mounts, mount_segments, connections)
    }

    pub fn build(self) -> EntityTemplate {
        let (segment_mounts, mount_segments, connections) = self.calculate_graph();

        let mut points = BTreeMap::new();
        let mut bones = BTreeMap::new();
        let mut joints = BTreeMap::new();

        for (point_id, point_builder) in self.points {
            points.insert(point_id, point_builder.build());
        }

        for ((bone_id, bone_builder), connection_type) in zip(self.bones, connections) {
            bones.insert(bone_id, bone_builder.build(&points, connection_type));
        }

        for (joint_id, joint_builder) in self.joints {
            joints.insert(joint_id, joint_builder.build(&bones));
        }

        EntityTemplate {
            segment_mounts,
            mount_segments,
            points,
            bones,
            joints,
            dismounted_timer: self.dismounted_timer,
            remounting_timer: self.remounting_timer,
            mounted_timer: self.mounted_timer,
            remount_version: self.remount_version,
        }
    }

    // Known bug: Default riders of different remount versions are not able to
    // cross-remount with each other because they come from different templates,
    // even though they normally would in linerider.com. This is such a niche case
    // that it's probably not worth fixing.
    // TODO Maybe we could solve this with computing graph isomorphism?
    /// Builds the original bosh skeleton
    pub fn default_rider(version: RemountVersion) -> EntityTemplate {
        let repel_length_factor = 0.5;
        let scarf_friction = 0.1;
        let mount_endurance = 0.057;
        let remount_endurance_factor = 2.0;
        let remount_strength_factor = match version {
            RemountVersion::None => 0.0,
            RemountVersion::ComV1 | RemountVersion::ComV2 => 0.1,
            RemountVersion::LRA => 0.5,
        };
        // Adjustment strength when remounting affects all bones in LRA
        let unbreakable_remount_strength_factor = match version {
            RemountVersion::LRA => 0.5,
            _ => 1.0,
        };

        // Remount version also affects physics processing order, which is why it's needed internally
        let mut skeleton = Self::new().remount_version(version);

        skeleton = match version {
            RemountVersion::None => skeleton,
            _ => skeleton
                .dismounted_timer(30)
                .remounting_timer(3)
                .mounted_timer(3),
        };

        let peg = skeleton.add_point(
            EntityPointBuilder::new(Point::new(0.0, 0.0))
                .is_contact(true)
                .contact_friction(0.8),
        );
        let tail =
            skeleton.add_point(EntityPointBuilder::new(Point::new(0.0, 5.0)).is_contact(true));
        let nose =
            skeleton.add_point(EntityPointBuilder::new(Point::new(15.0, 5.0)).is_contact(true));
        let string =
            skeleton.add_point(EntityPointBuilder::new(Point::new(17.5, 0.0)).is_contact(true));
        let butt = skeleton.add_point(
            EntityPointBuilder::new(Point::new(5.0, 0.0))
                .is_contact(true)
                .contact_friction(0.8),
        );
        let shoulder = skeleton.add_point(
            EntityPointBuilder::new(Point::new(5.0, -5.5))
                .is_contact(true)
                .contact_friction(0.8),
        );
        let right_hand = skeleton.add_point(
            EntityPointBuilder::new(Point::new(11.5, -5.0))
                .is_contact(true)
                .contact_friction(0.1),
        );
        let left_hand = skeleton.add_point(
            EntityPointBuilder::new(Point::new(11.5, -5.0))
                .is_contact(true)
                .contact_friction(0.1),
        );
        let left_foot =
            skeleton.add_point(EntityPointBuilder::new(Point::new(10.0, 5.0)).is_contact(true));
        let right_foot =
            skeleton.add_point(EntityPointBuilder::new(Point::new(10.0, 5.0)).is_contact(true));
        let scarf0 = skeleton
            .add_point(EntityPointBuilder::new(Point::new(3.0, -5.5)).air_friction(scarf_friction));
        let scarf1 = skeleton
            .add_point(EntityPointBuilder::new(Point::new(1.0, -5.5)).air_friction(scarf_friction));
        let scarf2 = skeleton.add_point(
            EntityPointBuilder::new(Point::new(-1.0, -5.5)).air_friction(scarf_friction),
        );
        let scarf3 = skeleton.add_point(
            EntityPointBuilder::new(Point::new(-3.0, -5.5)).air_friction(scarf_friction),
        );
        let scarf4 = skeleton.add_point(
            EntityPointBuilder::new(Point::new(-5.0, -5.5)).air_friction(scarf_friction),
        );
        let scarf5 = skeleton.add_point(
            EntityPointBuilder::new(Point::new(-7.0, -5.5)).air_friction(scarf_friction),
        );
        let scarf6 = skeleton.add_point(
            EntityPointBuilder::new(Point::new(-9.0, -5.5)).air_friction(scarf_friction),
        );

        let sled_back = skeleton.add_bone(
            EntityBoneBuilder::new((peg, tail))
                .adjustment_strength_remount_factor(unbreakable_remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((tail, nose))
                .adjustment_strength_remount_factor(unbreakable_remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((nose, string))
                .adjustment_strength_remount_factor(unbreakable_remount_strength_factor),
        );
        let sled_front = skeleton.add_bone(
            EntityBoneBuilder::new((string, peg))
                .adjustment_strength_remount_factor(unbreakable_remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((peg, nose))
                .adjustment_strength_remount_factor(unbreakable_remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((string, tail))
                .adjustment_strength_remount_factor(unbreakable_remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((peg, butt))
                .endurance(mount_endurance)
                .endurance_remount_factor(remount_endurance_factor)
                .adjustment_strength_remount_factor(remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((tail, butt))
                .endurance(mount_endurance)
                .endurance_remount_factor(remount_endurance_factor)
                .adjustment_strength_remount_factor(remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((nose, butt))
                .endurance(mount_endurance)
                .endurance_remount_factor(remount_endurance_factor)
                .adjustment_strength_remount_factor(remount_strength_factor),
        );
        let torso = skeleton.add_bone(
            EntityBoneBuilder::new((shoulder, butt))
                .adjustment_strength_remount_factor(unbreakable_remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((shoulder, left_hand))
                .adjustment_strength_remount_factor(unbreakable_remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((shoulder, right_hand))
                .adjustment_strength_remount_factor(unbreakable_remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((butt, left_foot))
                .adjustment_strength_remount_factor(unbreakable_remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((butt, right_foot))
                .adjustment_strength_remount_factor(unbreakable_remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((shoulder, right_hand))
                .adjustment_strength_remount_factor(unbreakable_remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((shoulder, peg))
                .endurance(mount_endurance)
                .endurance_remount_factor(remount_endurance_factor)
                .adjustment_strength_remount_factor(remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((left_hand, string))
                .endurance(mount_endurance)
                .endurance_remount_factor(remount_endurance_factor)
                .adjustment_strength_remount_factor(remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((right_hand, string))
                .endurance(mount_endurance)
                .endurance_remount_factor(remount_endurance_factor)
                .adjustment_strength_remount_factor(remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((left_foot, nose))
                .endurance(mount_endurance)
                .endurance_remount_factor(remount_endurance_factor)
                .adjustment_strength_remount_factor(remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((right_foot, nose))
                .endurance(mount_endurance)
                .endurance_remount_factor(remount_endurance_factor)
                .adjustment_strength_remount_factor(remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((shoulder, left_foot))
                .repel(true)
                .initial_length_factor(repel_length_factor)
                .adjustment_strength_remount_factor(unbreakable_remount_strength_factor),
        );
        skeleton.add_bone(
            EntityBoneBuilder::new((shoulder, right_foot))
                .repel(true)
                .initial_length_factor(repel_length_factor)
                .adjustment_strength_remount_factor(unbreakable_remount_strength_factor),
        );
        skeleton.add_bone(EntityBoneBuilder::new((shoulder, scarf0)).bias(1.0));
        skeleton.add_bone(EntityBoneBuilder::new((scarf0, scarf1)).bias(1.0));
        skeleton.add_bone(EntityBoneBuilder::new((scarf1, scarf2)).bias(1.0));
        skeleton.add_bone(EntityBoneBuilder::new((scarf2, scarf3)).bias(1.0));
        skeleton.add_bone(EntityBoneBuilder::new((scarf3, scarf4)).bias(1.0));
        skeleton.add_bone(EntityBoneBuilder::new((scarf4, scarf5)).bias(1.0));
        skeleton.add_bone(EntityBoneBuilder::new((scarf5, scarf6)).bias(1.0));

        skeleton.add_joint(EntityJointBuilder::new(sled_back, sled_front));
        skeleton.add_joint(EntityJointBuilder::new(torso, sled_front));

        skeleton.build()
    }
}

impl From<EntityTemplate> for EntityTemplateBuilder {
    fn from(skeleton: EntityTemplate) -> Self {
        Self {
            points: skeleton
                .points
                .into_iter()
                .map(|x| (x.0, x.1.into()))
                .collect(),
            bones: skeleton
                .bones
                .into_iter()
                .map(|x| (x.0, x.1.into()))
                .collect(),
            joints: skeleton
                .joints
                .into_iter()
                .map(|x| (x.0, x.1.into()))
                .collect(),
            dismounted_timer: skeleton.dismounted_timer,
            remounting_timer: skeleton.remounting_timer,
            mounted_timer: skeleton.mounted_timer,
            remount_version: skeleton.remount_version,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use geometry::Point;
    use pretty_assertions::assert_eq;

    use crate::entity_registry::{
        EntityBoneBuilder, EntityPointBuilder, EntityTemplateBuilder, bone,
        entity_template::{MountId, SegmentId},
    };

    fn check_test_case(
        num_points: u32,
        bones: Vec<(usize, usize, bool)>,
        segment_mounts_vec: Vec<(usize, Vec<usize>)>,
        mount_segments_vec: Vec<(usize, (usize, usize))>,
        bone_connections_vec: Vec<(bool, usize)>,
    ) {
        let mut skeleton = EntityTemplateBuilder::new();
        let mut points = Vec::new();

        for _ in 0..num_points {
            let point = EntityPointBuilder::new(Point::zero());
            points.push(skeleton.add_point(point));
        }

        for (p0, p1, is_breakable) in bones {
            let mut bone = EntityBoneBuilder::new((
                *points.get(p0).expect("Test case should have valid bone"),
                *points.get(p1).expect("Test case should have valid bone"),
            ));
            if is_breakable {
                bone = bone.endurance(0.5);
            }
            skeleton.add_bone(bone);
        }

        let result = skeleton.calculate_graph();

        let mut segment_mounts = BTreeMap::new();
        for (segment_id, mounts_vec) in segment_mounts_vec {
            let mut mounts = BTreeSet::new();
            for mount_id in mounts_vec {
                mounts.insert(MountId(mount_id));
            }
            segment_mounts.insert(SegmentId(segment_id), mounts);
        }

        let mut mount_segments = BTreeMap::new();
        for (mount_id, (segment0_id, segment1_id)) in mount_segments_vec {
            mount_segments.insert(
                MountId(mount_id),
                (SegmentId(segment0_id), SegmentId(segment1_id)),
            );
        }

        let mut bone_connections = Vec::new();
        for (is_mount_bone, target_id) in bone_connections_vec {
            if is_mount_bone {
                bone_connections.push(bone::ConnectionType::Mount(MountId(target_id)));
            } else {
                bone_connections.push(bone::ConnectionType::Segment(SegmentId(target_id)));
            }
        }

        let expected = (segment_mounts, mount_segments, bone_connections);

        assert_eq!(
            result.0, expected.0,
            "Segments should point to the correct mounts"
        );
        assert_eq!(
            result.1, expected.1,
            "Mounts should point to the correct segments"
        );
        assert_eq!(result.2, expected.2, "Bones should have the correct types");
    }

    #[test]
    fn single_unbreakable_bone() {
        check_test_case(
            2,
            vec![(0, 1, false)],
            vec![(0, vec![])],
            vec![],
            vec![(false, 0)],
        );
    }

    #[test]
    fn single_breakable_bone() {
        check_test_case(
            2,
            vec![(0, 1, true)],
            vec![(0, vec![0]), (1, vec![0])],
            vec![(0, (0, 1))],
            vec![(true, 0)],
        );
    }

    #[test]
    fn triangle_mount_same_segment() {
        check_test_case(
            3,
            vec![(0, 1, false), (1, 2, true), (0, 2, false)],
            vec![(0, vec![0])],
            vec![(0, (0, 0))],
            vec![(false, 0), (true, 0), (false, 0)],
        );
    }

    #[test]
    fn triangle_mount_different_segments() {
        check_test_case(
            3,
            vec![(0, 1, true), (1, 2, true), (0, 2, false)],
            vec![(0, vec![0]), (1, vec![0])],
            vec![(0, (0, 1))],
            vec![(true, 0), (true, 0), (false, 1)],
        );
    }

    #[test]
    fn hexagon_single_segment() {
        check_test_case(
            5,
            vec![
                (0, 1, false),
                (1, 2, false),
                (2, 3, false),
                (3, 4, false),
                (4, 0, false),
                (0, 2, false),
                (1, 4, false),
            ],
            vec![(0, vec![])],
            vec![],
            vec![
                (false, 0),
                (false, 0),
                (false, 0),
                (false, 0),
                (false, 0),
                (false, 0),
                (false, 0),
            ],
        );
    }

    #[test]
    fn hexagon_single_segment_multiple_mounts() {
        check_test_case(
            5,
            vec![
                (0, 1, true),
                (1, 2, false),
                (2, 3, false),
                (3, 4, false),
                (4, 0, false),
                (0, 2, true),
                (1, 4, true),
            ],
            vec![(0, vec![0, 1, 2])],
            vec![(0, (0, 0)), (1, (0, 0)), (2, (0, 0))],
            vec![
                (true, 0),
                (false, 0),
                (false, 0),
                (false, 0),
                (false, 0),
                (true, 1),
                (true, 2),
            ],
        );
    }

    #[test]
    fn hexagon_multiple_segments_and_mounts() {
        check_test_case(
            5,
            vec![
                (0, 1, true),
                (1, 2, false),
                (2, 3, false),
                (3, 4, true),
                (4, 0, false),
                (0, 2, true),
                (1, 4, true),
            ],
            vec![(0, vec![0]), (1, vec![0])],
            vec![(0, (0, 1))],
            vec![
                (true, 0),
                (false, 0),
                (false, 0),
                (true, 0),
                (false, 1),
                (true, 0),
                (true, 0),
            ],
        );
    }

    #[test]
    fn hexagon_only_mounts() {
        check_test_case(
            5,
            vec![
                (0, 1, true),
                (1, 2, true),
                (2, 3, true),
                (3, 4, true),
                (4, 0, true),
                (0, 2, true),
                (1, 4, true),
            ],
            vec![
                (0, vec![0, 4, 5]),
                (1, vec![0, 1, 6]),
                (2, vec![1, 2, 5]),
                (3, vec![2, 3]),
                (4, vec![3, 4, 6]),
            ],
            vec![
                (0, (0, 1)),
                (1, (1, 2)),
                (2, (2, 3)),
                (3, (3, 4)),
                (4, (0, 4)),
                (5, (0, 2)),
                (6, (1, 4)),
            ],
            vec![
                (true, 0),
                (true, 1),
                (true, 2),
                (true, 3),
                (true, 4),
                (true, 5),
                (true, 6),
            ],
        );
    }

    #[test]
    fn double_unbreakable_bone() {
        check_test_case(
            2,
            vec![(0, 1, false), (0, 1, false)],
            vec![(0, vec![])],
            vec![],
            vec![(false, 0), (false, 0)],
        );
    }

    #[test]
    fn unbreakable_bone_over_breakable_bone() {
        check_test_case(
            2,
            vec![(0, 1, false), (0, 1, true)],
            vec![(0, vec![0])],
            vec![(0, (0, 0))],
            vec![(false, 0), (true, 0)],
        );
    }

    #[test]
    fn double_breakable_bone() {
        check_test_case(
            2,
            vec![(0, 1, true), (0, 1, true)],
            vec![(0, vec![0]), (1, vec![0])],
            vec![(0, (0, 1))],
            vec![(true, 0), (true, 0)],
        );
    }

    #[test]
    fn connected_triangles() {
        check_test_case(
            9,
            vec![
                (0, 1, false),
                (0, 2, false),
                (1, 2, false),
                (3, 4, false),
                (3, 5, false),
                (4, 5, false),
                (6, 7, false),
                (6, 8, false),
                (7, 8, false),
                (1, 3, true),
                (1, 5, true),
                (2, 3, true),
                (2, 5, true),
                (5, 6, true),
                (5, 8, true),
                (4, 8, true),
                (4, 6, true),
            ],
            vec![(0, vec![0]), (1, vec![0, 1]), (2, vec![1])],
            vec![(0, (0, 1)), (1, (1, 2))],
            vec![
                (false, 0),
                (false, 0),
                (false, 0),
                (false, 1),
                (false, 1),
                (false, 1),
                (false, 2),
                (false, 2),
                (false, 2),
                (true, 0),
                (true, 0),
                (true, 0),
                (true, 0),
                (true, 1),
                (true, 1),
                (true, 1),
                (true, 1),
            ],
        );
    }

    #[test]
    fn connected_triangles_both_mount_types() {
        check_test_case(
            9,
            vec![
                (0, 1, false),
                (0, 2, false),
                (1, 2, false),
                (3, 4, false),
                (3, 5, false),
                (4, 5, false),
                (6, 7, false),
                (6, 8, false),
                (7, 8, false),
                (1, 3, true),
                (1, 5, true),
                (2, 3, true),
                (2, 5, true),
                (5, 6, true),
                (5, 8, false),
                (4, 8, true),
                (4, 6, false),
            ],
            vec![(0, vec![0]), (1, vec![0, 1, 2])],
            vec![(0, (0, 1)), (1, (1, 1)), (2, (1, 1))],
            vec![
                (false, 0),
                (false, 0),
                (false, 0),
                (false, 1),
                (false, 1),
                (false, 1),
                (false, 1),
                (false, 1),
                (false, 1),
                (true, 0),
                (true, 0),
                (true, 0),
                (true, 0),
                (true, 1),
                (false, 1),
                (true, 2),
                (false, 1),
            ],
        );
    }
}
