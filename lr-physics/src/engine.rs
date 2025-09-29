use geometry::Point;
use vector2d::Vector2Df;

use crate::{
    entity::{bone::EntityBone, joint::EntityJoint, point::EntityPoint, skeleton::EntitySkeleton},
    grid::Grid,
    line_manager::PhysicsLineManager,
};

// TODO data structure w/ operations:
// - adding and removing individual skeletons (cascade delete)
// - adding and removing mounts (unlink skeletons)
// - caching point states for each frame (basic clone)
// - caching mount states for each frame (basic clone)
// - removing from and adding to cache size whenever skeletons/mounts change
// - clearing front of cache whenever lines or skeletons (remounting) change
// - hot swapping cached point states whenever frame info requested

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

// This value represents how often we take a snapshot, in frames
// TODO tentative, maybe instead we should base off of a heuristic of how many calculations performed each frame?
const SNAPSHOT_FRAME_MODULO: u32 = 3;

pub type EntityRegistryIndex = usize;

struct EntityRegistry {
    points: Vec<EntityPoint>,
    bones: Vec<EntityBone>,
    joints: Vec<EntityJoint>,
    skeletons: Vec<EntitySkeleton>,
}

struct EngineState {
    frame: u32,
    // Entities that will always exist when playing through the track
    stable_entities: EntityRegistry,
    // Entities that may be created or destroyed when playing through the track
    unstable_entities: EntityRegistry,
}

struct EngineStateSnapshot {
    frame: u32,
    // TODO
}

pub struct Engine {
    grid: Grid,
    line_manager: PhysicsLineManager,
    state: EngineState,
    snapshots: Vec<EngineStateSnapshot>,
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

    fn get_line_manager(&self) -> &PhysicsLineManager {
        &self.line_manager
    }

    pub fn get_point(&self, index: EntityRegistryIndex) -> &EntityPoint {
        &self.state.stable_entities.points[index]
    }

    pub fn get_point_mut(&mut self, index: EntityRegistryIndex) -> &mut EntityPoint {
        &mut self.state.stable_entities.points[index]
    }

    pub fn get_bone(&self, index: EntityRegistryIndex) -> &EntityBone {
        &self.state.stable_entities.bones[index]
    }

    pub fn get_joint(&self, index: EntityRegistryIndex) -> &EntityJoint {
        &self.state.stable_entities.joints[index]
    }

    pub fn get_unstable_bone(&self, index: EntityRegistryIndex) -> &EntityBone {
        &self.state.unstable_entities.bones[index]
    }

    pub fn get_unstable_joint(&self, index: EntityRegistryIndex) -> &EntityJoint {
        &self.state.unstable_entities.joints[index]
    }

    /** Copies all necessary information from the current entity registry into a light-weight representation of the state at that instance */
    fn create_snapshot(&self) -> EngineStateSnapshot {
        todo!()
    }

    /** Restores the state from a snapshot back into the entity registry, overriding current values */
    fn restore_snapshot(&self, snapshot: &EngineStateSnapshot) {
        todo!()
    }

    /** Generates the next frame by advancing the current physics state of the entity registry */
    pub fn get_next_frame(&self) {
        //             for entity in new_entities:
        //                 entity.process_skeleton(gravity, self.grid)
        //
        //             for entity in new_entities:
        //                 entity.process_remount(new_entities)
    }

    /** Jumps to a target instance by retrieving its snapshot and stepping forward if needed */
    pub fn get_target_instance(&self, frame: u32, instance: PhysicsInstance) {
        todo!()
    }

    fn process_bone(&mut self, bone_index: EntityRegistryIndex, remounting: bool) {
        let bone = self.get_bone(bone_index);
        let adjustment = bone.get_adjustment(self, remounting);
        let point_indices = bone.get_points();
        let p0 = self.get_point_mut(point_indices.0);
        p0.update(
            p0.position() - adjustment.0,
            p0.velocity(),
            p0.previous_position(),
        );
        let p1 = self.get_point_mut(point_indices.1);
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
        let mut_point = self.get_point_mut(point_index);
        mut_point.update(new_position, mut_point.velocity(), new_previous_position)
    }

    // TODO this can be on its own thread (however, need to work out mutable registry access)
    fn process_skeleton(&mut self, skeleton: &mut EntitySkeleton) {
        let mut dismounted_this_frame = false;
        let mut intact_this_frame = true;
        let gravity = self.get_gravity_at_time(self.state.frame);

        for point_index in skeleton.points() {
            let point = self.get_point_mut(*point_index);
            let computed_velocity = point.position() - point.previous_position();
            let new_velocity = (computed_velocity * (1.0 - point.air_friction())) + gravity;
            let new_position = point.position() + new_velocity;
            point.update(new_position, new_velocity, point.position());
        }

        // let initial_mount_phase = self.mount_state.mount_phase;

        for _ in 0..6 {
            // TODO process other skeleton in mount's bones as well
            for bone_index in skeleton.bones() {
                let bone = self.get_bone(*bone_index);
                if !bone.is_repel() && bone.is_contact(self) {
                    self.process_bone(*bone_index, skeleton.is_remounting());
                }
            }

            if skeleton.is_mounted() {
                for bone_index in skeleton.mount_bones() {
                    let bone = self.get_unstable_bone(*bone_index);
                    let bone_intact = bone.get_intact(self, skeleton.is_remounting());
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
                let bone = self.get_bone(*bone_index);
                if bone.is_repel() && bone.is_contact(self) {
                    self.process_bone(*bone_index, skeleton.is_remounting());
                }
            }

            let mut collisions = vec![];

            for point_index in skeleton.points() {
                let point = self.get_point(*point_index);
                if point.is_contact() {
                    let interacting_lines = self.grid.get_lines_near_point(point.position());
                    for line_id in interacting_lines {
                        let optional_line = self.line_manager.get_line(line_id);
                        if let Some(line) = optional_line {
                            if let Some((new_position, new_previous_position)) =
                                line.check_interaction(point)
                            {
                                collisions.push((
                                    *point_index,
                                    new_position,
                                    new_previous_position,
                                ));
                            }
                        } else {
                            // TODO: gracefully handle this
                            unreachable!("Line was not found")
                        }
                    }
                }
            }

            for (point_index, new_position, new_previous_position) in collisions {
                self.process_collision(point_index, new_position, new_previous_position);
            }
        }

        for bone_index in skeleton.bones() {
            let bone = self.get_bone(*bone_index);
            if !bone.is_contact(self) {
                self.process_bone(*bone_index, skeleton.is_remounting());
            }
        }

        if skeleton.is_mounted() {
            for joint_index in skeleton.mount_joints() {
                let joint = self.get_unstable_joint(*joint_index);
                if !joint.get_intact(self) && !dismounted_this_frame {
                    dismounted_this_frame = true;
                }
            }
        }

        for joint_index in skeleton.joints() {
            let joint = self.get_joint(*joint_index);
            if !joint.get_intact(self) && !dismounted_this_frame {
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

    // TODO this should block the skeleton processing thread until all have processed
    fn update_remount_phases(&self) {}
}
