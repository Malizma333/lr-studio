// TODO Should version specific features (eg LRA Remount or .com Scarf) be plugins?

// - adding and removing individual skeletons (cascade delete)
// - adding and removing mounts (unlink skeletons)
// - caching point states for each frame (basic clone)
// - caching mount states for each frame (basic clone)
// - removing from and adding to cache size whenever mounts change
// - clearing front of cache whenever lines change
// - swapping cached point states whenever frame info requested
// - moving green lines?

// Engine Constraints
// Skeletons: <= 50
// Bones: <= 2,500
// Points: <= 2,500
// Mounts: <= 25
// Physics Lines <= 10,000
// Scenery Lines <= ~100,000,000?

/*
Call free on skeleton/mount, which frees up lower components?
When adding a new one, rebuild from reference data
  Reset cache back to frame 0? (or use smart caching)
Processing in order (Vec)
 */

use std::collections::HashMap;

use geometry::Point;
use vector2d::Vector2Df;

use crate::{
    engine::entity_registry::{EntityRegistry, EntityRegistryIndex},
    entity::{
        bone::EntityBoneLogic,
        joint::EntityJointLogic,
        point::EntityPointState,
        skeleton::{EntitySkeleton, EntitySkeletonState},
    },
    grid::{Grid, GridVersion, LineId},
    line::hitbox::Hitbox,
};

struct EngineState {
    frame: u32,
    point_states: HashMap<EntityRegistryIndex, EntityPointState>,
    skeleton_states: HashMap<EntityRegistryIndex, EntitySkeletonState>,
}

impl Clone for EngineState {
    fn clone(&self) -> Self {
        Self {
            frame: self.frame,
            // hashmap clone is implemented as a deep copy
            point_states: self.point_states.clone(),
            skeleton_states: self.skeleton_states.clone(),
        }
    }
}

impl EngineState {
    fn new() -> Self {
        Self {
            frame: 0,
            point_states: HashMap::new(),
            skeleton_states: HashMap::new(),
        }
    }
}

pub struct Engine {
    grid: Grid,
    line_lookup: HashMap<LineId, Box<dyn Hitbox>>,
    registry: EntityRegistry,
    state: EngineState,
    // use .truncate when clearing cache
    state_snapshots: Vec<EngineState>,
}

enum PhysicsInstance {
    MomentumTick,
    AccelerationTick,
    FrictionTick,
    GravityTick,
    Iteration {
        index: u8,
        sub_iteration: Option<u8>,
    },
}

impl Engine {
    pub fn new(grid_version: GridVersion) -> Engine {
        Engine {
            grid: Grid::new(grid_version),
            line_lookup: HashMap::new(),
            registry: EntityRegistry::new(),
            state: EngineState::new(),
            state_snapshots: Vec::new(),
        }
    }

    // TODO: Should be overridable
    fn get_gravity_at_time(&self, _frame: u32) -> Vector2Df {
        Vector2Df::new(0.0, 0.175)
    }

    // TODO: Should be overridable
    fn get_entity_frozen_at_time(&self, _entity: EntityRegistryIndex, _frame: u32) -> bool {
        false
    }

    fn get_grid(&self) -> &Grid {
        &self.grid
    }

    fn get_line_lookup(&self) -> &HashMap<LineId, Box<dyn Hitbox>> {
        &self.line_lookup
    }

    /** Generates the next frame by advancing the current physics state of the entity registry */
    pub fn get_next_frame(&self) {}

    /** Jumps to a target instance by retrieving its snapshot and stepping forward if needed */
    pub fn get_target_instance(&self, frame: u32, instance: PhysicsInstance) {
        todo!()
    }

    fn process_bone(&mut self, bone_index: EntityRegistryIndex, remounting: bool) {
        let bone = self
            .registry
            .get_bone(bone_index)
            .get_snapshot(&self.registry, remounting);
        let adjustment = bone.get_adjustment();
        let point_indices = self.registry.get_bone(bone_index).get_points();
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
        point_index: EntityRegistryIndex,
        new_position: Point,
        new_previous_position: Point,
    ) {
        let mut_point = self.registry.get_point_mut(point_index);
        mut_point.update(new_position, mut_point.velocity(), new_previous_position)
    }

    fn process_skeleton(&mut self, skeleton: &mut EntitySkeleton) {
        let mut dismounted_this_frame = false;
        let mut intact_this_frame = true;
        let gravity = self.get_gravity_at_time(self.state.frame);

        for point_index in skeleton.points() {
            let point = self.registry.get_point_mut(*point_index);
            let computed_velocity = point.position() - point.previous_position();
            let new_velocity = (computed_velocity * (1.0 - point.air_friction())) + gravity;
            let new_position = point.position() + new_velocity;
            point.update(new_position, new_velocity, point.position());
        }

        for _ in 0..6 {
            // TODO process other skeleton in mount's bones as well
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
                if !joint.get_snapshot(&self.registry).is_intact() && !dismounted_this_frame {
                    dismounted_this_frame = true;
                }
            }
        }

        for joint_index in skeleton.joints() {
            let joint = self.registry.get_joint(*joint_index);
            if !joint.get_snapshot(&self.registry).is_intact() && !dismounted_this_frame {
                intact_this_frame = false;
            }
        }

        if dismounted_this_frame {
            skeleton.dismount();
        }

        if !intact_this_frame {
            todo!("set intact to false")
        }
    }

    fn update_remount_phases(&self) {}
}
