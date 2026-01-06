use geometry::Point;
use vector2d::Vector2Df;

use crate::{MountPhase, engine::EngineState, entity::registry::EntityRegistry};

pub struct SkeletonView {
    point_positions: Vec<Point>,
    point_velocities: Vec<Vector2Df>,
    mount_phase: MountPhase,
    sled_intact: bool,
}

impl SkeletonView {
    pub fn point_positions(&self) -> &Vec<Point> {
        &self.point_positions
    }

    pub fn point_velocities(&self) -> &Vec<Vector2Df> {
        &self.point_velocities
    }

    pub fn mount_phase(&self) -> MountPhase {
        self.mount_phase
    }

    pub fn sled_intact(&self) -> bool {
        self.sled_intact
    }
}

pub struct EngineView {
    skeletons: Vec<SkeletonView>,
}

impl EngineView {
    pub(super) fn new(registry: &EntityRegistry, state: &EngineState) -> Self {
        let mut skeleton_views = Vec::new();

        let skeletons = registry.skeletons();

        for (skeleton_id, skeleton) in skeletons {
            let skeleton_state = state.skeletons().get(skeleton_id).unwrap();
            let mut point_positions = Vec::new();
            let mut point_velocities = Vec::new();

            for point_id in skeleton.points() {
                let point_state = state.points().get(point_id).unwrap();
                point_positions.push(point_state.position());
                point_velocities.push(point_state.velocity());
            }

            skeleton_views.push(SkeletonView {
                point_positions,
                point_velocities,
                mount_phase: skeleton_state.mount_phase(),
                sled_intact: skeleton_state.sled_intact(),
            });
        }

        EngineView {
            skeletons: skeleton_views,
        }
    }

    pub fn skeletons(&self) -> &Vec<SkeletonView> {
        &self.skeletons
    }
}
