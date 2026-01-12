use geometry::Point;
use std::collections::BTreeMap;

use crate::entity_registry::{
    EntityBone, EntityBoneBuilder, EntityJoint, EntityJointBuilder, EntityPoint,
    EntityPointBuilder, EntityState, RemountVersion,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityPointId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityBoneId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityJointId(usize);

pub struct EntityTemplate {
    points: BTreeMap<EntityPointId, EntityPoint>,
    bones: BTreeMap<EntityBoneId, EntityBone>,
    joints: BTreeMap<EntityJointId, EntityJoint>,
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

    pub(crate) fn dismounted_timer(&self) -> u32 {
        self.dismounted_timer
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

    pub(super) fn can_swap_sleds(
        &self,
        state: &mut EntityState,
        other_state: &mut EntityState,
    ) -> bool {
        if other_state.sled_intact() && other_state.mount_phase().is_dismounted() {
            // Swap sleds to check entity can safely remount
            self.swap_skeleton_sleds(state, other_state);

            if self.skeleton_can_enter_phase(state, true) {
                return true;
            }

            // Swap sleds back if we failed
            self.swap_skeleton_sleds(state, other_state);
        }

        false
    }

    fn swap_skeleton_sleds(&self, state: &mut EntityState, other_state: &mut EntityState) {
        match self.remount_version() {
            RemountVersion::ComV2 | RemountVersion::LRA => {
                let sled_intact = state.skeleton_state().sled_intact();
                let other_sled_intact = other_state.skeleton_state().sled_intact();
                other_state
                    .skeleton_state_mut()
                    .set_sled_intact(sled_intact);
                state
                    .skeleton_state_mut()
                    .set_sled_intact(other_sled_intact);
            }
            _ => {}
        }

        // Assumes sled points are in same order, because they originate from same template
        for point_id in self.sled_points() {
            let point_state = state.point_state(point_id).clone();
            let other_point_state = other_state.point_state(point_id).clone();

            other_state.point_state_mut(point_id).update(
                Some(point_state.position()),
                Some(point_state.velocity()),
                Some(point_state.computed_previous_position()),
            );
            state.point_state_mut(point_id).update(
                Some(other_point_state.position()),
                Some(other_point_state.velocity()),
                Some(other_point_state.computed_previous_position()),
            );
        }
    }

    pub(super) fn skeleton_can_enter_phase(
        &self,
        state: &EntityState,
        target_phase_is_remounting: bool,
    ) -> bool {
        for bone in self.bones().values() {
            let point_states = (
                state.point_state(&bone.point_ids().0),
                state.point_state(&bone.point_ids().1),
            );

            if bone.is_breakable() && !bone.get_intact(point_states, target_phase_is_remounting) {
                return false;
            }
        }

        match self.remount_version() {
            RemountVersion::ComV1 | RemountVersion::ComV2 => {
                for joint in self.joints().values() {
                    if !joint.is_mount() && self.get_joint_should_break(state, joint) {
                        return false;
                    }
                }

                for joint in self.joints().values() {
                    if joint.is_mount() && self.get_joint_should_break(state, joint) {
                        return false;
                    }
                }
            }
            _ => {}
        }

        true
    }

    pub(super) fn get_joint_should_break(&self, state: &EntityState, joint: &EntityJoint) -> bool {
        let bones = (
            &self.bones()[&joint.bones().0],
            &self.bones()[&joint.bones().1],
        );
        let bone0_p0 = state.point_state(&bones.0.point_ids().0);
        let bone0_p1 = state.point_state(&bones.0.point_ids().1);
        let bone1_p0 = state.point_state(&bones.1.point_ids().0);
        let bone1_p1 = state.point_state(&bones.1.point_ids().1);
        let bone_vectors = (
            bone0_p0.position().vector_from(bone0_p1.position()),
            bone1_p0.position().vector_from(bone1_p1.position()),
        );
        joint.should_break(bone_vectors)
    }

    // TODO use computed graph for this
    fn sled_points(&self) -> Vec<&EntityPointId> {
        let x: Vec<&EntityPointId> = self.points.keys().collect();
        vec![x[0], x[1], x[2], x[3]]
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

    pub(crate) fn add_point(&mut self, point: EntityPointBuilder) -> EntityPointId {
        let id = EntityPointId(self.points.len());
        self.points.insert(id, point);
        id
    }

    pub(crate) fn add_bone(&mut self, bone: EntityBoneBuilder) -> EntityBoneId {
        let id = EntityBoneId(self.bones.len());
        self.bones.insert(id, bone);
        id
    }

    pub(crate) fn add_joint(&mut self, joint: EntityJointBuilder) -> EntityJointId {
        let id = EntityJointId(self.joints.len());
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

    pub fn build(self) -> EntityTemplate {
        let points = self
            .points
            .into_iter()
            .map(|x| (x.0, x.1.build()))
            .collect();
        let bones = self
            .bones
            .into_iter()
            .map(|x| (x.0, x.1.build(&points)))
            .collect();
        let joints = self
            .joints
            .into_iter()
            .map(|x| (x.0, x.1.build()))
            .collect();

        EntityTemplate {
            points,
            bones,
            joints,
            dismounted_timer: self.dismounted_timer,
            remounting_timer: self.remounting_timer,
            mounted_timer: self.mounted_timer,
            remount_version: self.remount_version,
        }
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

impl EntityTemplateBuilder {
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
            RemountVersion::ComV1 => 0.1,
            RemountVersion::ComV2 => 0.1,
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

        // TODO remove redundant joint by breaking sled causing dismount
        skeleton.add_joint(EntityJointBuilder::new(sled_back, sled_front).is_mount(true));
        skeleton.add_joint(EntityJointBuilder::new(torso, sled_front).is_mount(true));
        skeleton.add_joint(EntityJointBuilder::new(sled_back, sled_front));

        skeleton.build()
    }
}
