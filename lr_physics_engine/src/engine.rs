use crate::{
    InitialProperties, MountPhase, PhysicsLine, PhysicsLineBuilder, build_default_rider,
    engine::state::EngineState,
    entity::{
        joint::entity::EntityJoint,
        point::state::EntityPointState,
        registry::{EntityRegistry, EntitySkeletonId, EntitySkeletonTemplateId},
        skeleton::{
            RemountVersion, builder::EntitySkeletonBuilder, entity::EntitySkeleton,
            state::EntitySkeletonState,
        },
    },
    line_store::{LineId, LineStore},
};
use lr_format_core::Track;
use lr_physics_grid::GridVersion;
use vector2d::Vector2Df;
mod moment;
mod state;
mod view;
pub use moment::PhysicsMoment;
pub use view::EngineView;

pub struct Engine {
    line_store: LineStore,
    registry: EntityRegistry,
    // The initial state of the engine as a reference point
    initial_state: EngineState,
    // A list of cached state snapshots we can jump to
    state_snapshots: Vec<EngineState>,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new(GridVersion::V6_2)
    }
}

impl Engine {
    pub fn new(grid_version: GridVersion) -> Self {
        Engine {
            line_store: LineStore::new(grid_version),
            // TODO adding or removing from the registry should modify state_snapshots and initial_state
            // Changing the initial state should clear the state snapshots
            // So the entity registry should be part of the initial state?
            registry: EntityRegistry::new(),
            state_snapshots: Vec::new(),
            initial_state: EngineState::new(),
        }
    }

    /// Changes the engine's grid version and reregisters all physics lines
    pub fn set_grid_version(&mut self, grid_version: GridVersion) {
        self.line_store.set_grid_version(grid_version);
    }

    /// Provides a view of entities during a specific frame by simulating up to that frame
    pub fn view_frame(&mut self, frame: u32) -> EngineView {
        self.fill_snapshots_up_to_frame(frame);
        let index = (frame as usize).saturating_sub(1);
        let state = self
            .state_snapshots
            .get(index)
            .unwrap_or(&self.initial_state);
        EngineView::new(&self.registry, state)
    }

    /// Provides a view of entities during a specific moment by simulating up to that frame and moment
    pub fn view_moment(&mut self, frame: u32, moment: PhysicsMoment) -> EngineView {
        self.fill_snapshots_up_to_frame(frame);
        let index = (frame as usize).saturating_sub(1);
        let target_frame_state = self
            .state_snapshots
            .get(index)
            .unwrap_or(&self.initial_state)
            .clone();
        let state = self.get_next_state(target_frame_state, frame, Some(moment));
        EngineView::new(&self.registry, &state)
    }

    pub fn add_line(&mut self, line: PhysicsLine) -> LineId {
        let id = self.line_store.add_line(line);
        self.invalidate_snapshots();
        id
    }

    pub fn get_line(&self, id: LineId) -> Option<&PhysicsLine> {
        self.line_store.get_line(id)
    }

    pub fn replace_line(&mut self, id: LineId, new_line: PhysicsLine) {
        self.line_store.replace_line(id, new_line);
        self.invalidate_snapshots();
    }

    pub fn remove_line(&mut self, id: LineId) {
        self.line_store.remove_line(id);
        self.invalidate_snapshots();
    }

    fn invalidate_snapshots(&mut self) {
        // the best way to do this is probably track grid cells affected each frame,
        // then invalidate the first frame that was effected by a grid cell changing
        self.state_snapshots.truncate(0);
    }

    pub fn build_skeleton(&mut self) -> EntitySkeletonBuilder<'_> {
        self.registry.skeleton_template_builder()
    }

    pub fn add_skeleton(
        &mut self,
        skeleton_template_id: EntitySkeletonTemplateId,
    ) -> EntitySkeletonId {
        let skeleton_id = self.registry.create_skeleton(skeleton_template_id);
        let skeleton = self.registry.get_skeleton(skeleton_id);

        self.initial_state.skeletons_mut().insert(
            skeleton_id,
            EntitySkeletonState::new(MountPhase::Mounted, true),
        );

        for point_id in skeleton.points() {
            let point = self.registry.get_point(*point_id);
            let offset = point.initial_position();
            self.initial_state.points_mut().insert(
                *point_id,
                EntityPointState::new(offset, Vector2Df::zero(), offset),
            );
        }

        self.invalidate_snapshots();
        skeleton_id
    }

    pub fn set_skeleton_initial_properties(
        &mut self,
        skeleton_id: EntitySkeletonId,
        initial_properties: InitialProperties,
    ) {
        let skeleton = self.registry.get_skeleton(skeleton_id);

        for point_id in skeleton.points() {
            let point = self.registry.get_point(*point_id);
            let local_offset = point.initial_position();
            let position = local_offset.translated_by(initial_properties.start_offset());
            let velocity = initial_properties.start_velocity();
            self.initial_state
                .points_mut()
                .get_mut(point_id)
                .unwrap()
                .update(
                    Some(position),
                    Some(velocity),
                    Some(position.translated_by(-1.0 * velocity)),
                );
        }

        self.invalidate_snapshots();
    }

    pub fn remove_skeleton(&mut self, skeleton_id: EntitySkeletonId) {
        let skeleton = self.registry.get_skeleton(skeleton_id);

        self.initial_state.skeletons_mut().remove(&skeleton_id);

        for point_id in skeleton.points() {
            self.initial_state.points_mut().remove(point_id);
        }

        self.registry.delete_skeleton(skeleton_id);

        self.invalidate_snapshots();
    }

    fn fill_snapshots_up_to_frame(&mut self, target_frame: u32) {
        let mut current_state = self
            .state_snapshots
            .last()
            .unwrap_or(&self.initial_state)
            .clone();

        while (self.state_snapshots.len() as u32) < target_frame {
            let next_state =
                self.get_next_state(current_state, self.state_snapshots.len() as u32, None);
            self.state_snapshots.push(next_state.clone());
            current_state = next_state.clone();
        }
    }

    // The main loop of the physics engine
    fn get_next_state(
        &mut self,
        mut current_state: EngineState,
        _frame: u32,
        _moment: Option<PhysicsMoment>,
    ) -> EngineState {
        let mut dismount_flags = Vec::new();

        // Physics step
        for (skeleton_id, skeleton) in self.registry.skeletons() {
            let mut dismounted_this_frame = false;

            // momentum
            for point_id in skeleton.points() {
                let point = self.registry.get_point(*point_id);
                let point_state = current_state.points_mut().get_mut(point_id).unwrap();
                const GRAVITY_MULTIPLIER: f64 = 0.175;
                let gravity = Vector2Df::down() * GRAVITY_MULTIPLIER;
                point.process_initial_step(point_state, gravity.flip_vertical());
            }

            let initial_mount_phase = current_state
                .skeletons()
                .get(skeleton_id)
                .unwrap()
                .mount_phase();

            for _ in 0..6 {
                // bones
                for bone_id in skeleton.bones() {
                    let bone = self.registry.get_bone(*bone_id);

                    if !bone.is_flutter() {
                        let point_states = (
                            current_state.points().get(&bone.points().0).unwrap(),
                            current_state.points().get(&bone.points().1).unwrap(),
                        );

                        let mount_phase = match skeleton.remount_version() {
                            RemountVersion::LRA => initial_mount_phase,
                            _ => current_state
                                .skeletons()
                                .get(skeleton_id)
                                .unwrap()
                                .mount_phase(),
                        };

                        if !bone.is_breakable() {
                            let adjusted =
                                bone.get_adjusted(point_states, mount_phase.remounting());
                            current_state
                                .points_mut()
                                .get_mut(&bone.points().0)
                                .unwrap()
                                .update(Some(adjusted.0), None, None);
                            current_state
                                .points_mut()
                                .get_mut(&bone.points().1)
                                .unwrap()
                                .update(Some(adjusted.1), None, None);
                        } else if (mount_phase.remounting() || mount_phase.mounted())
                            && !dismounted_this_frame
                        {
                            if bone.get_intact(point_states, mount_phase.remounting()) {
                                let adjusted =
                                    bone.get_adjusted(point_states, mount_phase.remounting());
                                current_state
                                    .points_mut()
                                    .get_mut(&bone.points().0)
                                    .unwrap()
                                    .update(Some(adjusted.0), None, None);
                                current_state
                                    .points_mut()
                                    .get_mut(&bone.points().1)
                                    .unwrap()
                                    .update(Some(adjusted.1), None, None);
                            } else {
                                dismounted_this_frame = true;

                                let next_mount_phase = match skeleton.remount_version() {
                                    RemountVersion::None => MountPhase::Dismounted {
                                        frames_until_remounting: 0,
                                    },
                                    _ => {
                                        if mount_phase.mounted() {
                                            MountPhase::Dismounting {
                                                frames_until_dismounted: skeleton
                                                    .dismounted_timer(),
                                            }
                                        } else if mount_phase.remounting() {
                                            MountPhase::Dismounted {
                                                frames_until_remounting: skeleton
                                                    .remounting_timer(),
                                            }
                                        } else {
                                            mount_phase
                                        }
                                    }
                                };

                                current_state
                                    .skeletons_mut()
                                    .get_mut(skeleton_id)
                                    .unwrap()
                                    .set_mount_phase(next_mount_phase);
                            }
                        }
                    }
                }

                // line collisions
                for point_id in skeleton.points() {
                    let point = self.registry.get_point(*point_id);
                    let point_state = current_state.points_mut().get_mut(point_id).unwrap();
                    for line in self.line_store.lines_near_point(point_state.position()) {
                        if let Some((new_position, new_external_velocity)) =
                            line.check_interaction(point, point_state)
                        {
                            point_state.update(
                                Some(new_position),
                                None,
                                Some(new_external_velocity),
                            );
                        }
                    }
                }
            }

            // flutter bones
            for bone_id in skeleton.bones() {
                let bone = self.registry.get_bone(*bone_id);
                if bone.is_flutter() {
                    let point_states = (
                        current_state.points().get(&bone.points().0).unwrap(),
                        current_state.points().get(&bone.points().1).unwrap(),
                    );
                    let mount_phase = current_state
                        .skeletons()
                        .get(skeleton_id)
                        .unwrap()
                        .mount_phase();
                    let adjusted = bone.get_adjusted(point_states, mount_phase.remounting());
                    current_state
                        .points_mut()
                        .get_mut(&bone.points().0)
                        .unwrap()
                        .update(Some(adjusted.0), None, None);
                    current_state
                        .points_mut()
                        .get_mut(&bone.points().1)
                        .unwrap()
                        .update(Some(adjusted.1), None, None);
                }
            }

            // check dismount
            let mount_phase = current_state
                .skeletons()
                .get(skeleton_id)
                .unwrap()
                .mount_phase();

            if mount_phase.mounted() || mount_phase.remounting() {
                for joint_id in skeleton.joints() {
                    let joint = self.registry.get_joint(*joint_id);
                    if joint.is_mount()
                        && self.get_joint_should_break(joint, &current_state)
                        && !dismounted_this_frame
                    {
                        dismounted_this_frame = true;

                        let next_mount_phase = match skeleton.remount_version() {
                            RemountVersion::None => MountPhase::Dismounted {
                                frames_until_remounting: 0,
                            },
                            _ => {
                                if mount_phase.mounted() {
                                    MountPhase::Dismounting {
                                        frames_until_dismounted: skeleton.dismounted_timer(),
                                    }
                                } else if mount_phase.remounting() {
                                    MountPhase::Dismounted {
                                        frames_until_remounting: skeleton.remounting_timer(),
                                    }
                                } else {
                                    mount_phase
                                }
                            }
                        };

                        current_state
                            .skeletons_mut()
                            .get_mut(skeleton_id)
                            .unwrap()
                            .set_mount_phase(next_mount_phase);

                        match skeleton.remount_version() {
                            RemountVersion::LRA => current_state
                                .skeletons_mut()
                                .get_mut(skeleton_id)
                                .unwrap()
                                .set_sled_intact(false),
                            _ => {}
                        }
                    }
                }
            }

            // check skeleton break (like sled break)
            let mount_phase = current_state
                .skeletons()
                .get(skeleton_id)
                .unwrap()
                .mount_phase();
            let sled_intact = current_state
                .skeletons()
                .get(skeleton_id)
                .unwrap()
                .sled_intact();

            let sled_break_version = match skeleton.remount_version() {
                RemountVersion::None | RemountVersion::ComV2 => true,
                _ => false,
            };

            if mount_phase.mounted() || mount_phase.remounting() || sled_break_version {
                for joint_id in skeleton.joints() {
                    let joint = self.registry.get_joint(*joint_id);
                    if !joint.is_mount()
                        && self.get_joint_should_break(joint, &current_state)
                        && sled_intact
                    {
                        current_state
                            .skeletons_mut()
                            .get_mut(skeleton_id)
                            .unwrap()
                            .set_sled_intact(false);
                    }
                }
            }

            dismount_flags.push(dismounted_this_frame);
        }

        let mut dismount_flag_index = 0;

        // Remount step
        for (skeleton_id, skeleton) in self.registry.skeletons() {
            let dismounted_this_frame = dismount_flags[dismount_flag_index];
            dismount_flag_index += 1;

            if !dismounted_this_frame {
                let current_mount_phase = current_state
                    .skeletons()
                    .get(skeleton_id)
                    .unwrap()
                    .mount_phase();

                let sled_intact = current_state
                    .skeletons()
                    .get(skeleton_id)
                    .unwrap()
                    .sled_intact();

                let next_mount_phase = match skeleton.remount_version() {
                    RemountVersion::LRA => {
                        if !sled_intact {
                            MountPhase::Dismounted {
                                frames_until_remounting: 0,
                            }
                        } else {
                            match current_mount_phase {
                                MountPhase::Dismounting {
                                    frames_until_dismounted,
                                } => {
                                    if frames_until_dismounted == 0 {
                                        MountPhase::Dismounted {
                                            frames_until_remounting: skeleton.remounting_timer(),
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
                                    if self.skeleton_can_swap_sleds(&mut current_state, skeleton_id)
                                    {
                                        if frames_until_remounting == 0 {
                                            MountPhase::Remounting {
                                                frames_until_mounted: skeleton.mounted_timer(),
                                            }
                                        } else {
                                            MountPhase::Dismounted {
                                                frames_until_remounting: frames_until_remounting
                                                    .saturating_sub(1),
                                            }
                                        }
                                    } else {
                                        MountPhase::Dismounted {
                                            frames_until_remounting: skeleton.remounting_timer(),
                                        }
                                    }
                                }
                                MountPhase::Remounting {
                                    frames_until_mounted,
                                } => {
                                    if self.skeleton_can_enter_phase(
                                        &current_state,
                                        skeleton,
                                        false,
                                    ) {
                                        if frames_until_mounted == 0 {
                                            MountPhase::Mounted
                                        } else {
                                            MountPhase::Remounting {
                                                frames_until_mounted: frames_until_mounted
                                                    .saturating_sub(1),
                                            }
                                        }
                                    } else {
                                        MountPhase::Remounting {
                                            frames_until_mounted: skeleton.mounted_timer(),
                                        }
                                    }
                                }
                                MountPhase::Mounted => MountPhase::Mounted,
                            }
                        }
                    }
                    RemountVersion::ComV1 | RemountVersion::ComV2 => match current_mount_phase {
                        MountPhase::Dismounting {
                            frames_until_dismounted,
                        } => {
                            let next_timer = frames_until_dismounted.saturating_sub(1);
                            if next_timer == 0 {
                                MountPhase::Dismounted {
                                    frames_until_remounting: skeleton.remounting_timer(),
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
                            let next_timer =
                                if self.skeleton_can_swap_sleds(&mut current_state, skeleton_id) {
                                    frames_until_remounting.saturating_sub(1)
                                } else {
                                    skeleton.remounting_timer()
                                };

                            if next_timer == 0 {
                                MountPhase::Remounting {
                                    frames_until_mounted: skeleton.mounted_timer(),
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
                            let next_timer =
                                if self.skeleton_can_enter_phase(&current_state, skeleton, false) {
                                    frames_until_mounted.saturating_sub(1)
                                } else {
                                    skeleton.mounted_timer()
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
                    RemountVersion::None => current_mount_phase,
                };

                current_state
                    .skeletons_mut()
                    .get_mut(skeleton_id)
                    .unwrap()
                    .set_mount_phase(next_mount_phase);
            }
        }

        current_state
    }

    fn get_joint_should_break(&self, joint: &EntityJoint, current_state: &EngineState) -> bool {
        let bones = (
            self.registry.get_bone(joint.bones().0),
            self.registry.get_bone(joint.bones().1),
        );
        let bone0_p0 = current_state.points().get(&bones.0.points().0).unwrap();
        let bone0_p1 = current_state.points().get(&bones.0.points().1).unwrap();
        let bone1_p0 = current_state.points().get(&bones.1.points().0).unwrap();
        let bone1_p1 = current_state.points().get(&bones.1.points().1).unwrap();
        let bone_vectors = (
            bone0_p0.position().vector_from(bone0_p1.position()),
            bone1_p0.position().vector_from(bone1_p1.position()),
        );
        joint.should_break(bone_vectors)
    }

    fn swap_skeleton_sleds(
        &self,
        current_state: &mut EngineState,
        target_skeleton_id: &EntitySkeletonId,
        other_skeleton_id: &EntitySkeletonId,
    ) {
        let target_skeleton = self.registry.get_skeleton(*target_skeleton_id);
        let other_skeleton = self.registry.get_skeleton(*other_skeleton_id);

        match target_skeleton.remount_version() {
            RemountVersion::ComV2 | RemountVersion::LRA => {
                let sled_intact = current_state
                    .skeletons()
                    .get(target_skeleton_id)
                    .unwrap()
                    .sled_intact();
                let other_sled_intact = current_state
                    .skeletons()
                    .get(other_skeleton_id)
                    .unwrap()
                    .sled_intact();
                current_state
                    .skeletons_mut()
                    .get_mut(other_skeleton_id)
                    .unwrap()
                    .set_sled_intact(sled_intact);
                current_state
                    .skeletons_mut()
                    .get_mut(target_skeleton_id)
                    .unwrap()
                    .set_sled_intact(other_sled_intact);
            }
            _ => {}
        }

        // Assumes sled points are in same order, because they originate from same template
        for (index, target_point_id) in target_skeleton.sled_points().iter().enumerate() {
            let other_points = other_skeleton.sled_points();
            let other_point_id = other_points.get(index).unwrap();
            let target_state = current_state.points().get(target_point_id).unwrap().clone();
            let other_state = current_state.points().get(other_point_id).unwrap().clone();

            current_state
                .points_mut()
                .get_mut(other_point_id)
                .unwrap()
                .update(
                    Some(target_state.position()),
                    Some(target_state.velocity()),
                    Some(target_state.external_velocity()),
                );
            current_state
                .points_mut()
                .get_mut(target_point_id)
                .unwrap()
                .update(
                    Some(other_state.position()),
                    Some(other_state.velocity()),
                    Some(other_state.external_velocity()),
                );
        }
    }

    fn skeleton_can_swap_sleds(
        &self,
        current_state: &mut EngineState,
        target_skeleton_id: &EntitySkeletonId,
    ) -> bool {
        let target_skeleton = self.registry.get_skeleton(*target_skeleton_id);
        for (other_skeleton_id, skeleton) in self.registry.skeletons() {
            let skeleton_state = current_state.skeletons().get(other_skeleton_id).unwrap();
            if skeleton_state.sled_intact()
                && skeleton_state.mount_phase().dismounted()
                && skeleton.template_id() == target_skeleton.template_id()
            {
                // Swap sleds to check entity can safely remount
                self.swap_skeleton_sleds(current_state, target_skeleton_id, other_skeleton_id);

                if self.skeleton_can_enter_phase(current_state, target_skeleton, true) {
                    return true;
                }

                // Swap sleds back if we failed
                self.swap_skeleton_sleds(current_state, target_skeleton_id, other_skeleton_id);
            }
        }

        false
    }

    fn skeleton_can_enter_phase(
        &self,
        current_state: &EngineState,
        skeleton: &EntitySkeleton,
        target_phase_is_remounting: bool,
    ) -> bool {
        for bone_id in skeleton.bones() {
            let bone = self.registry.get_bone(*bone_id);
            let point_states = (
                current_state.points().get(&bone.points().0).unwrap(),
                current_state.points().get(&bone.points().1).unwrap(),
            );

            if bone.is_breakable() && !bone.get_intact(point_states, target_phase_is_remounting) {
                return false;
            }
        }

        match skeleton.remount_version() {
            RemountVersion::ComV1 | RemountVersion::ComV2 => {
                for joint_id in skeleton.joints() {
                    let joint = self.registry.get_joint(*joint_id);
                    if !joint.is_mount() && self.get_joint_should_break(joint, &current_state) {
                        return false;
                    }
                }

                for joint_id in skeleton.joints() {
                    let joint = self.registry.get_joint(*joint_id);
                    if joint.is_mount() && self.get_joint_should_break(joint, &current_state) {
                        return false;
                    }
                }
            }
            _ => {}
        }

        true
    }

    // TODO make these only visible by tests

    pub fn clear_frame_cache(&mut self) {
        self.state_snapshots.truncate(0);
    }

    pub fn from_track(track: &Track, lra: bool) -> Self {
        let grid_version = match track.grid_version() {
            lr_format_core::GridVersion::V6_0 => GridVersion::V6_0,
            lr_format_core::GridVersion::V6_1 => GridVersion::V6_1,
            lr_format_core::GridVersion::V6_2 => GridVersion::V6_2,
        };
        let mut engine = Engine::new(grid_version);

        for line in track.standard_lines() {
            let physics_line = PhysicsLineBuilder::new(line.endpoints())
                .flipped(line.flipped())
                .left_extension(line.left_extension())
                .right_extension(line.right_extension())
                .height(line.height())
                .acceleration_multiplier(line.multiplier())
                .build();
            engine.add_line(physics_line);
        }

        let template_none = build_default_rider(&mut engine, RemountVersion::None);
        let template_comv1 = build_default_rider(&mut engine, RemountVersion::ComV1);
        let template_comv2 = build_default_rider(&mut engine, RemountVersion::ComV2);
        let template_lra = build_default_rider(&mut engine, RemountVersion::LRA);

        for rider in track.riders() {
            let mut initial_properties = InitialProperties::new();
            let target_skeleton_template_id = if lra {
                template_lra
            } else {
                match rider.remount_version() {
                    lr_format_core::RemountVersion::None => template_none,
                    lr_format_core::RemountVersion::ComV1 => template_comv1,
                    lr_format_core::RemountVersion::ComV2 => template_comv2,
                    lr_format_core::RemountVersion::LRA => template_lra,
                }
            };
            let id = engine.add_skeleton(target_skeleton_template_id);
            if let Some(initial_position) = rider.start_offset() {
                initial_properties.set_start_offset(initial_position);
            }
            if let Some(initial_velocity) = rider.start_velocity() {
                initial_properties.set_start_velocity(initial_velocity);
            }
            engine.set_skeleton_initial_properties(id, initial_properties);
        }

        engine
    }
}
