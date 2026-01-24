use std::collections::{BTreeMap, BTreeSet};

use geometry::Point;
use vector2d::Vector2Df;

use crate::{
    entity_registry::{
        EntityPointId, EntityPointState, EntityTemplate, MountPhase, RemountVersion, bone,
        entity_template::{MountId, SegmentId},
        joint,
    },
    line_registry::LineRegistry,
};

const MAX_ITERATION: u8 = 6;

#[derive(Debug, Clone)]
pub struct EntityState {
    // Cloning a map is 5x slower than cloning a vec, but at this scale it's a difference of nanoseconds
    broken_segments: BTreeSet<SegmentId>,
    mount_phases: BTreeMap<MountId, MountPhase>,
    point_states: BTreeMap<EntityPointId, EntityPointState>,
}

impl EntityState {
    pub(super) fn new(
        template: &EntityTemplate,
        initial_offset: Vector2Df,
        initial_velocity: Vector2Df,
    ) -> Self {
        let mut point_states = BTreeMap::new();

        for (point_id, point_template) in template.points() {
            let position = point_template
                .initial_position()
                .translated_by(initial_offset);
            let velocity = initial_velocity;
            let point_state =
                EntityPointState::new(position, velocity, position.translated_by(-velocity));
            point_states.insert(*point_id, point_state);
        }

        let broken_segments = BTreeSet::new();
        let mut mount_phases = BTreeMap::new();

        for mount_id in template.mount_segments().keys() {
            mount_phases.insert(*mount_id, MountPhase::Mounted);
        }

        Self {
            broken_segments,
            mount_phases,
            point_states,
        }
    }

    pub fn point_positions(&self) -> Vec<Point> {
        self.point_states
            .iter()
            .map(|point| point.1.position())
            .collect()
    }

    pub fn point_velocities(&self) -> Vec<Vector2Df> {
        self.point_states
            .iter()
            .map(|point| point.1.velocity())
            .collect()
    }

    pub fn mount_phases(&self) -> &BTreeMap<MountId, MountPhase> {
        &self.mount_phases
    }

    pub fn broken_segments(&self) -> &BTreeSet<SegmentId> {
        &self.broken_segments
    }

    pub(crate) fn point_state(&self, point_id: &EntityPointId) -> &EntityPointState {
        self.point_states
            .get(point_id)
            .expect("Point state should exist when retrieved internally")
    }

    pub(crate) fn point_state_mut(&mut self, point_id: &EntityPointId) -> &mut EntityPointState {
        self.point_states
            .get_mut(point_id)
            .expect("Point state should exist when retrieved internally")
    }

    // This is the main physics loop that transforms an entity state
    // Returns whether the rider dismounted
    pub(super) fn process_frame(
        &mut self,
        template: &EntityTemplate,
        line_registry: &LineRegistry,
    ) -> BTreeSet<MountId> {
        let mut dismounted = BTreeSet::new();

        // Process points in order by applying gravity and previous frame forces (e.g. friction, acceleration)
        for (point_id, point) in template.points() {
            const GRAVITY_MULTIPLIER: f64 = 0.175;
            let gravity = Vector2Df::down() * GRAVITY_MULTIPLIER;
            let point_state = self.point_state_mut(point_id);
            point.apply_momentum(point_state, gravity);
        }

        // Need to clone the initial mount phase since that's what LRA uses
        let initial_mount_phases = self.mount_phases.clone();

        for _ in 0..MAX_ITERATION {
            // Process bones not connected to flutter points
            for bone in template.bones().values() {
                if !bone.is_flutter() {
                    let mount_phases = match template.remount_version() {
                        RemountVersion::LRA => &initial_mount_phases,
                        _ => &self.mount_phases,
                    };
                    match bone.connection_type() {
                        bone::ConnectionType::Segment(segment_id) => {
                            let is_remounting =
                                template.is_segment_remounting(mount_phases, segment_id);

                            bone.adjust_points(self, is_remounting);
                        }
                        bone::ConnectionType::Mount(mount_id) => {
                            let mount_phase = mount_phases
                                .get(&mount_id)
                                .expect("Mounts should have a corresponding mount phase");

                            if (mount_phase.is_remounting() || mount_phase.is_mounted())
                                && !dismounted.contains(&mount_id)
                            {
                                if bone.is_intact(self, mount_phase.is_remounting()) {
                                    bone.adjust_points(self, mount_phase.is_remounting());
                                } else {
                                    dismounted.insert(mount_id);
                                    self.mount_phases.insert(
                                        mount_id,
                                        template.get_phase_after_dismount(*mount_phase),
                                    );
                                }
                            }
                        }
                    }
                }
            }

            // Process point-line collisions by point then by line
            for (point_id, point) in template.points() {
                if point.is_contact() {
                    let point_state = self.point_state_mut(point_id);
                    for line in line_registry.lines_near_point(point_state.position()) {
                        if let Some((new_position, new_computed_previous_position)) =
                            line.check_interaction(point, point_state)
                        {
                            point_state.update(
                                Some(new_position),
                                None,
                                Some(new_computed_previous_position),
                            );
                        }
                    }
                }
            }
        }

        // Process bones connected to flutter points in order
        for bone in template.bones().values() {
            if bone.is_flutter() {
                // Mount phase is determined the same as non-flutter bones
                let is_remounting = match bone.connection_type() {
                    bone::ConnectionType::Segment(segment_id) => {
                        template.is_segment_remounting(&self.mount_phases, segment_id)
                    }
                    bone::ConnectionType::Mount(mount_id) => self
                        .mount_phases()
                        .get(&mount_id)
                        .expect("Mounts should have a corresponding mount phase")
                        .is_remounting(),
                };
                bone.adjust_points(self, is_remounting);
            }
        }

        // Process joints in order
        for joint in template.joints().values() {
            if joint.should_break(self, template) {
                let mut broken_mounts = BTreeSet::new();
                let mut broken_segments = BTreeSet::new();

                match joint.connection_type() {
                    joint::ConnectionType::Segments(segment0_id, segment1_id) => {
                        if segment0_id == segment1_id {
                            // A joint across the same segment will break the segment and mounts connected to it
                            let mounts = template
                                .segment_mounts()
                                .get(&segment0_id)
                                .expect("Segments should have their connected mounts initialized");
                            let mut all_mounts_intact = true;
                            for mount_id in mounts {
                                let mount_phase = self
                                    .mount_phases
                                    .get(mount_id)
                                    .expect("Mounts should have a corresponding mount phase");
                                all_mounts_intact = all_mounts_intact
                                    && (mount_phase.is_remounting() || mount_phase.is_mounted());
                                broken_mounts.insert(*mount_id);
                            }
                            // LRA and comv1 remounting don't break the sled after the rider is dismounted
                            if template.remount_version().is_comv2()
                                || template.remount_version().is_none()
                                || all_mounts_intact
                            {
                                broken_segments.insert(segment0_id);
                            }
                        } else {
                            // A joint across two segments will break the mount between them if one exists
                            let mounts0 = template
                                .segment_mounts()
                                .get(&segment0_id)
                                .expect("Segments should have their connected mounts initialized");
                            let mounts1 = template
                                .segment_mounts()
                                .get(&segment1_id)
                                .expect("Segments should have their connected mounts initialized");
                            let mounts = mounts0.intersection(mounts1);
                            let mut all_mounts_intact = true;
                            for mount_id in mounts {
                                let mount_phase = self
                                    .mount_phases
                                    .get(mount_id)
                                    .expect("Mounts should have a corresponding mount phase");
                                all_mounts_intact = all_mounts_intact
                                    && (mount_phase.is_remounting() || mount_phase.is_mounted());
                                broken_mounts.insert(*mount_id);
                            }
                            // LRA also breaks sled when a mount joint breaks
                            if template.remount_version().is_lra() && all_mounts_intact {
                                broken_segments.insert(segment0_id);
                                broken_segments.insert(segment1_id);
                            }
                        }
                    }
                    joint::ConnectionType::Hybrid(segment_id, mount_id) => {
                        // A joint across a segment and mount will break the mount if the segment is connected to the mount
                        let mounts = template
                            .segment_mounts()
                            .get(&segment_id)
                            .expect("Segments should have their connected mounts initialized");
                        if mounts.contains(&mount_id) {
                            broken_mounts.insert(mount_id);
                        }
                    }
                    joint::ConnectionType::Mounts(mount0_id, mount1_id) => {
                        // A joint across the same mounts will break the mount
                        if mount0_id == mount1_id {
                            broken_mounts.insert(mount0_id);
                        }
                    }
                }

                for segment_id in broken_segments {
                    // Broken segments also break the mounts around them
                    self.broken_segments.insert(segment_id);
                    let attached_mounts = template
                        .segment_mounts()
                        .get(&segment_id)
                        .expect("Segments should have their connected mounts initialized");
                    for mount_id in attached_mounts {
                        broken_mounts.insert(*mount_id);
                    }
                }

                for mount_id in broken_mounts {
                    let mount_phase = self
                        .mount_phases
                        .get_mut(&mount_id)
                        .expect("Mounts should have a corresponding mount phase");
                    // Only break mounts that are in the mount phase and haven't been dismounted this frame
                    if (mount_phase.is_mounted() || mount_phase.is_remounting())
                        && !dismounted.contains(&mount_id)
                    {
                        dismounted.insert(mount_id);
                        *mount_phase = template.get_phase_after_dismount(*mount_phase);
                    }
                }
            }
        }

        dismounted
    }

    // This retrieves the next mount phase
    pub(super) fn process_mount_phase(
        &mut self,
        template: &EntityTemplate,
        other_states: &mut Vec<EntityState>,
        dismounted: &BTreeSet<MountId>,
    ) {
        let mut new_mount_phases = self.mount_phases.clone();

        for (mount_id, current_mount_phase) in &mut new_mount_phases {
            if !dismounted.contains(mount_id) {
                *current_mount_phase = match template.remount_version() {
                    RemountVersion::LRA => {
                        // TODO removed conditional sled break check here, check if still valid
                        match current_mount_phase {
                            MountPhase::Dismounting {
                                frames_until_dismounted,
                            } => {
                                if *frames_until_dismounted == 0 {
                                    MountPhase::Dismounted {
                                        frames_until_remounting: template.remounting_timer(),
                                    }
                                } else {
                                    MountPhase::Dismounting {
                                        frames_until_dismounted: frames_until_dismounted
                                            .saturating_sub(1),
                                    }
                                }
                            }
                            MountPhase::Dismounted {
                                frames_until_remounting,
                            } => {
                                let mut can_swap = false;

                                for other_state in &mut *other_states {
                                    if self.can_swap_sleds(template, other_state, *mount_id) {
                                        can_swap = true;
                                        break;
                                    }
                                }

                                if can_swap {
                                    if *frames_until_remounting == 0 {
                                        MountPhase::Remounting {
                                            frames_until_mounted: template.mounted_timer(),
                                        }
                                    } else {
                                        MountPhase::Dismounted {
                                            frames_until_remounting: frames_until_remounting
                                                .saturating_sub(1),
                                        }
                                    }
                                } else {
                                    MountPhase::Dismounted {
                                        frames_until_remounting: template.remounting_timer(),
                                    }
                                }
                            }
                            MountPhase::Remounting {
                                frames_until_mounted,
                            } => {
                                if self.skeleton_can_enter_phase(template, false) {
                                    if *frames_until_mounted == 0 {
                                        MountPhase::Mounted
                                    } else {
                                        MountPhase::Remounting {
                                            frames_until_mounted: frames_until_mounted
                                                .saturating_sub(1),
                                        }
                                    }
                                } else {
                                    MountPhase::Remounting {
                                        frames_until_mounted: template.mounted_timer(),
                                    }
                                }
                            }
                            MountPhase::Mounted => MountPhase::Mounted,
                        }
                    }
                    RemountVersion::ComV1 | RemountVersion::ComV2 => match current_mount_phase {
                        MountPhase::Dismounting {
                            frames_until_dismounted,
                        } => {
                            let next_timer = frames_until_dismounted.saturating_sub(1);
                            if next_timer == 0 {
                                MountPhase::Dismounted {
                                    frames_until_remounting: template.remounting_timer(),
                                }
                            } else {
                                MountPhase::Dismounting {
                                    frames_until_dismounted: next_timer,
                                }
                            }
                        }
                        MountPhase::Dismounted {
                            frames_until_remounting,
                        } => {
                            let mut can_swap = false;

                            for other_state in &mut *other_states {
                                if self.can_swap_sleds(template, other_state, *mount_id) {
                                    can_swap = true;
                                    break;
                                }
                            }

                            let next_timer = if can_swap {
                                frames_until_remounting.saturating_sub(1)
                            } else {
                                template.remounting_timer()
                            };

                            if next_timer == 0 {
                                MountPhase::Remounting {
                                    frames_until_mounted: template.mounted_timer(),
                                }
                            } else {
                                MountPhase::Dismounted {
                                    frames_until_remounting: next_timer,
                                }
                            }
                        }
                        MountPhase::Remounting {
                            frames_until_mounted,
                        } => {
                            let next_timer = if self.skeleton_can_enter_phase(template, false) {
                                frames_until_mounted.saturating_sub(1)
                            } else {
                                template.mounted_timer()
                            };

                            if next_timer == 0 {
                                MountPhase::Mounted
                            } else {
                                MountPhase::Remounting {
                                    frames_until_mounted: next_timer,
                                }
                            }
                        }
                        MountPhase::Mounted => MountPhase::Mounted,
                    },
                    RemountVersion::None => *current_mount_phase,
                };
            }
        }

        // This just overrides the old mount phases with the new ones
        self.mount_phases.append(&mut new_mount_phases);
    }

    fn can_swap_sleds(
        &mut self,
        template: &EntityTemplate,
        other_state: &mut EntityState,
        mount_id: MountId,
    ) -> bool {
        let segment0_id = template
            .mount_segments()
            .get(&mount_id)
            .expect("Mounts should have their connected segments initialized")
            .0;
        let other_mount_phase = other_state
            .mount_phases
            .get(&mount_id)
            .expect("Mounts should have a corresponding mount phase");

        if !other_state.broken_segments.contains(&segment0_id) && other_mount_phase.is_dismounted()
        {
            // Swap sleds to check entity can safely remount
            self.swap_skeleton_states(template, other_state, segment0_id);

            if self.skeleton_can_enter_phase(template, true) {
                return true;
            }

            // Swap sleds back if we failed
            self.swap_skeleton_states(template, other_state, segment0_id);
        }

        false
    }

    fn swap_skeleton_states(
        &mut self,
        template: &EntityTemplate,
        other_state: &mut EntityState,
        segment_id: SegmentId,
    ) {
        match template.remount_version() {
            RemountVersion::ComV2 | RemountVersion::LRA => {
                if self.broken_segments.remove(&segment_id) {
                    other_state.broken_segments.insert(segment_id);
                }
            }
            _ => {}
        }

        // Assumes sled points are in same order, because they originate from same template
        for point_id in &template.get_segment_points(segment_id) {
            let point_state = self.point_state(point_id).clone();
            let other_point_state = other_state.point_state(point_id).clone();

            other_state.point_state_mut(point_id).update(
                Some(point_state.position()),
                Some(point_state.velocity()),
                Some(point_state.computed_previous_position()),
            );
            self.point_state_mut(point_id).update(
                Some(other_point_state.position()),
                Some(other_point_state.velocity()),
                Some(other_point_state.computed_previous_position()),
            );
        }
    }

    fn skeleton_can_enter_phase(
        &self,
        template: &EntityTemplate,
        target_phase_is_remounting: bool,
    ) -> bool {
        for bone in template.bones().values() {
            if let bone::ConnectionType::Mount(_) = bone.connection_type()
                && !bone.is_intact(self, target_phase_is_remounting)
            {
                return false;
            }
        }

        match template.remount_version() {
            RemountVersion::ComV1 | RemountVersion::ComV2 => {
                for joint in template.joints().values() {
                    if joint.should_break(self, template) {
                        return false;
                    }
                }
            }
            _ => {}
        }

        true
    }
}
