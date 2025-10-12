use std::collections::HashMap;

use geometry::Point;
use vector2d::Vector2Df;

use crate::{
    entity::{
        entity_registry::{EntityBoneId, EntityPointId, EntityRegistry, EntitySkeletonId},
        logic::{bone::EntityBoneLogic, joint::EntityJointLogic, point::EntityPointLogic},
        point::EntityPointState,
    },
    grid::{Grid, GridVersion, LineId},
    line::hitbox::Hitbox,
};

enum PhysicsMoment {
    MomentumTick,
    AccelerationTick,
    FrictionTick,
    GravityTick,
    Iteration {
        index: u8,
        sub_iteration: Option<u8>,
    },
}

struct EngineState {
    point_states: HashMap<EntityPointId, EntityPointState>,
}

impl Clone for EngineState {
    fn clone(&self) -> Self {
        Self {
            // hashmap clone is implemented as a deep copy
            point_states: self.point_states.clone(),
        }
    }
}

impl EngineState {
    fn new() -> Self {
        Self {
            point_states: HashMap::new(),
        }
    }
}

pub struct Engine {
    grid: Grid,
    line_lookup: HashMap<LineId, Box<dyn Hitbox>>,
    registry: EntityRegistry,
    state: EngineState,
    current_frame: u32,
    // use .truncate when clearing cache
    state_snapshots: Vec<EngineState>,
}

impl Engine {
    pub fn new(grid_version: GridVersion) -> Engine {
        let initial_state = EngineState::new();

        Engine {
            current_frame: 0,
            grid: Grid::new(grid_version),
            line_lookup: HashMap::new(),
            registry: EntityRegistry::new(),
            state_snapshots: vec![initial_state.clone()],
            state: initial_state,
        }
    }

    // TODO: Should be overridable
    fn get_gravity_at_time(&self, _frame: u32) -> Vector2Df {
        Vector2Df::new(0.0, 0.175)
    }

    // TODO: Should be overridable
    fn get_skeleton_frozen_at_time(&self, _skeleton: EntitySkeletonId, _frame: u32) -> bool {
        false
    }

    fn get_grid(&self) -> &Grid {
        &self.grid
    }

    fn get_line_lookup(&self) -> &HashMap<LineId, Box<dyn Hitbox>> {
        &self.line_lookup
    }

    /** Generates the next frame by advancing the current physics state of the entity registry */
    fn get_next_unknown_frame(&mut self) {
        self.current_frame = self.state_snapshots.len() as u32;
        for skeleton_id in self.registry.list_skeletons() {
            self.process_skeleton(skeleton_id);
        }
        self.process_remount();
        self.state_snapshots.push(self.state.clone());
    }

    /** Jumps to a target instance by retrieving its snapshot and stepping forward if needed */
    pub fn get_target_moment(&mut self, frame: u32, instance: PhysicsMoment) -> &EngineState {
        while (self.state_snapshots.len() as u32) <= frame {
            self.get_next_unknown_frame();
        }
        &self.state_snapshots[frame as usize]
    }

    fn process_bone(&mut self, bone_id: EntityBoneId, remounting: bool) {
        let bone = self
            .registry
            .get_bone(bone_id)
            .get_snapshot(&self.registry, remounting);
        let adjustment = bone.get_adjustment();
        let point_indices = self.registry.get_bone(bone_id).get_points();
        let p0 = self.registry.get_point_mut(point_indices.0);
        p0.update(
            p0.position() - adjustment.0,
            p0.velocity(),
            p0.previous_position(),
        );
        let p1 = self.registry.get_point_mut(point_indices.1);
        p1.update(
            p1.position() - adjustment.1,
            p1.velocity(),
            p1.previous_position(),
        );
    }

    fn process_collision(
        &mut self,
        point_id: EntityPointId,
        new_position: Point,
        new_previous_position: Point,
    ) {
        let mut_point = self.registry.get_point_mut(point_id);
        mut_point.update(new_position, mut_point.velocity(), new_previous_position)
    }

    fn process_skeleton(&mut self, skeleton_id: EntitySkeletonId) {
        let mut dismounted_this_frame = false;
        let mut intact_this_frame = true;
        let gravity = self.get_gravity_at_time(self.current_frame);
        let skeleton = self.registry.get_skeleton(skeleton_id);

        for point_index in skeleton.points() {
            let point = self.registry.get_point_mut(*point_index);
            let computed_velocity = point.position() - point.previous_position();
            let new_velocity = (computed_velocity * (1.0 - point.air_friction())) + gravity;
            let new_position = point.position() + new_velocity;
            point.update(new_position, new_velocity, point.position());
        }

        for _ in 0..6 {
            for bone_index in skeleton.bones() {
                let bone = self
                    .registry
                    .get_bone(*bone_index)
                    .get_snapshot(&self.registry, skeleton.is_remounting());
                if !bone.is_repel() && !bone.is_flutter() {
                    self.process_bone(*bone_index, skeleton.is_remounting());
                }
            }

            if skeleton.is_mounted() {
                for bone_index in skeleton.mount_bones() {
                    let bone = self
                        .registry
                        .get_bone(*bone_index)
                        .get_snapshot(&self.registry, skeleton.is_remounting());
                    let bone_intact = bone.get_intact();
                    if dismounted_this_frame {
                        if bone_intact {
                            self.process_bone(*bone_index, skeleton.is_remounting());
                        } else {
                            dismounted_this_frame = true;
                        }
                    }
                }
            }

            for bone_index in skeleton.bones() {
                let bone = self
                    .registry
                    .get_bone(*bone_index)
                    .get_snapshot(&self.registry, skeleton.is_remounting());
                if bone.is_repel() && !bone.is_flutter() {
                    self.process_bone(*bone_index, skeleton.is_remounting());
                }
            }

            let mut collisions = vec![];

            for point_index in skeleton.points() {
                let point = self.registry.get_point(*point_index);
                if point.is_contact() {
                    let interacting_lines = self.grid.get_lines_near_point(point.position());
                    for line_id in interacting_lines {
                        let optional_line = self.line_lookup.get(&line_id);
                        if let Some(line) = optional_line {
                            if let Some((new_position, new_previous_position)) =
                                line.check_interaction(point)
                            {
                                collisions.push((point_index, new_position, new_previous_position));
                            }
                        } else {
                            // TODO: gracefully handle this
                            unreachable!("Line was not found")
                        }
                    }
                }
            }

            for (point_index, new_position, new_previous_position) in collisions {
                self.process_collision(*point_index, new_position, new_previous_position);
            }
        }

        for bone_index in skeleton.bones() {
            let bone = self
                .registry
                .get_bone(*bone_index)
                .get_snapshot(&self.registry, skeleton.is_remounting());
            if bone.is_flutter() {
                self.process_bone(*bone_index, skeleton.is_remounting());
            }
        }

        if skeleton.is_mounted() {
            for joint_index in skeleton.mount_joints() {
                let joint = self.registry.get_joint(*joint_index);
                if joint.get_snapshot(&self.registry).should_break() && !dismounted_this_frame {
                    dismounted_this_frame = true;
                }
            }
        }

        for joint_index in skeleton.joints() {
            let joint = self.registry.get_joint(*joint_index);
            if joint.get_snapshot(&self.registry).should_break() && !dismounted_this_frame {
                intact_this_frame = false;
            }
        }

        if dismounted_this_frame {
            skeleton.dismount();
        }

        if !intact_this_frame {
            skeleton.dismount();
            skeleton.destroy();
        }
    }

    fn process_remount(&mut self) {
        todo!()
    }
}
