mod engine;
mod entity;
mod line;
mod premade;

pub use engine::{Engine, EngineBuilder, EngineView};
pub use entity::{EntitySkeletonInitialProperties, MountPhase};
pub use line::{ComputedLineProperties, ComputedProperties, Hitbox};
pub use premade::{AccelerationLine, NormalLine, build_default_rider};

#[cfg(test)]
pub mod test_utils {
    use format_core::track::{RemountVersion, Track};
    use geometry::Point;
    use vector2d::Vector2Df;

    use crate::{
        AccelerationLine, Engine, EngineBuilder, EntitySkeletonInitialProperties, Hitbox,
        NormalLine, build_default_rider,
    };

    // For now, this is just a test function, but may exist in another glue package later on
    pub fn create_engine(track: Track, lra: bool) -> Engine {
        let grid_version = track.metadata().grid_version();
        let mut engine = EngineBuilder::new(grid_version).build();
        let mut ordered_lines: Vec<(u32, Box<dyn Hitbox>)> = Vec::new();

        for line in track.line_group().acceleration_lines() {
            let p0 = Point::new(line.x1(), line.y1());
            let p1 = Point::new(line.x2(), line.y2());
            let acceleration_line = AccelerationLine::new(
                (p0, p1),
                line.flipped(),
                line.left_extension(),
                line.right_extension(),
                line.multiplier().unwrap_or(1.0),
            );
            ordered_lines.push((line.id(), Box::new(acceleration_line)));
        }

        for line in track.line_group().standard_lines() {
            let p0 = Point::new(line.x1(), line.y1());
            let p1 = Point::new(line.x2(), line.y2());

            let normal_line = NormalLine::new(
                (p0, p1),
                line.flipped(),
                line.left_extension(),
                line.right_extension(),
            );
            ordered_lines.push((line.id(), Box::new(normal_line)));
        }

        ordered_lines.sort_by_key(|(key, _)| *key);

        for line in ordered_lines {
            engine.create_line(line.1);
        }

        let default_skeleton_template_id_none =
            build_default_rider(&mut engine, RemountVersion::None);
        let default_skeleton_template_id_comv1 =
            build_default_rider(&mut engine, RemountVersion::ComV1);
        let default_skeleton_template_id_comv2 =
            build_default_rider(&mut engine, RemountVersion::ComV2);
        let default_skeleton_template_id_lra =
            build_default_rider(&mut engine, RemountVersion::LRA);

        if let Some(rider_group) = track.rider_group() {
            for rider in rider_group.riders() {
                let mut initial_properties = EntitySkeletonInitialProperties::new();
                let target_skeleton_template_id = if lra {
                    default_skeleton_template_id_lra
                } else {
                    match rider.remount_version() {
                        RemountVersion::None => default_skeleton_template_id_none,
                        RemountVersion::ComV1 => default_skeleton_template_id_comv1,
                        RemountVersion::ComV2 => default_skeleton_template_id_comv2,
                        RemountVersion::LRA => default_skeleton_template_id_lra,
                    }
                };
                let id = engine.add_skeleton(target_skeleton_template_id);
                if let Some(initial_position) = rider.start_position() {
                    initial_properties.set_start_position(initial_position);
                }
                if let Some(initial_velocity) = rider.start_velocity() {
                    initial_properties.set_start_velocity(initial_velocity);
                }
                engine.set_skeleton_initial_properties(id, initial_properties);
            }
        } else {
            let mut initial_properties = EntitySkeletonInitialProperties::new();
            let id = engine.add_skeleton(default_skeleton_template_id_none);
            initial_properties.set_start_velocity(Vector2Df::new(0.4, 0.0));
            engine.set_skeleton_initial_properties(id, initial_properties);
        }

        engine
    }
}
