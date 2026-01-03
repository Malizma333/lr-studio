#[cfg(test)]
mod tests {
    use format_core::track::RemountVersion as FormatRemountVersion;
    use format_json;
    use geometry::Point;
    use physics::{
        AccelerationLine as PhysicsAccelerationLine, Engine, EngineBuilder, EngineView,
        EntitySkeletonInitialProperties, Hitbox, MountPhase, NormalLine as PhysicsNormalLine,
        RemountVersion, build_default_rider,
    };
    use serde::Deserialize;
    use spatial_grid::GridVersion;
    use std::fs;
    use vector2d::Vector2Df;

    #[derive(Deserialize)]
    struct EngineTestCaseEntity {
        points: Vec<String>,
        mount_state: Option<String>,
        sled_state: Option<String>,
    }

    #[derive(Deserialize)]
    struct EngineTestCaseState {
        entities: Vec<EngineTestCaseEntity>,
    }

    #[derive(Deserialize)]
    struct EngineTestCase {
        test: String,
        frame: u32,
        lra: Option<bool>,
        file: String,
        state: EngineTestCaseState,
    }

    #[test]
    fn engine_fixtures() {
        let data =
            fs::read_to_string("tests/fixture_tests.json").expect("Failed to read JSON file");
        let mut test_cases: Vec<EngineTestCase> =
            serde_json::from_str(&data).expect("Failed to parse JSON");

        test_cases.sort_by_key(|test_case| test_case.file.clone());
        let mut last_test_file = String::new();
        let mut engine = EngineBuilder::new(GridVersion::V6_2).build();

        for (i, test) in test_cases.iter().enumerate() {
            println!("Engine test {}: {}", i, test.test);

            if &last_test_file != &test.file {
                engine = create_engine(&test.file, test.lra.is_some_and(|lra| lra));
                last_test_file = test.file.clone();
            }

            compare_states(engine.view_frame(test.frame), &test.state);
        }
    }

    fn create_engine(file: &String, lra: bool) -> Engine {
        let file_name = format!("../fixtures/physics/{}.track.json", file);
        let file = fs::read(file_name).expect("Failed to read JSON file");
        let track = format_json::read(&file).expect("Failed to parse track file");

        let grid_version = track.metadata().grid_version();
        let mut engine = EngineBuilder::new(grid_version).build();
        let mut ordered_lines: Vec<(u32, Box<dyn Hitbox>)> = Vec::new();

        for line in track.line_group().acceleration_lines() {
            let p0 = Point::new(line.x1(), line.y1());
            let p1 = Point::new(line.x2(), line.y2());
            let acceleration_line = PhysicsAccelerationLine::new(
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

            let normal_line = PhysicsNormalLine::new(
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
                        FormatRemountVersion::None => default_skeleton_template_id_none,
                        FormatRemountVersion::ComV1 => default_skeleton_template_id_comv1,
                        FormatRemountVersion::ComV2 => default_skeleton_template_id_comv2,
                        FormatRemountVersion::LRA => default_skeleton_template_id_lra,
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

    fn compare_states(result: EngineView, expected: &EngineTestCaseState) {
        let expected_entities = &expected.entities;
        let result_entities = result.skeletons();
        assert!(
            result_entities.len() == expected_entities.len(),
            "entity count mismatch: {} != {}",
            result_entities.len(),
            expected_entities.len(),
        );
        for (i, expected_entity) in expected_entities.iter().enumerate() {
            let result_entity = &result_entities[i];
            if let Some(expected_mount_state) = &expected_entity.mount_state {
                let result_mount_state = match result_entity.mount_phase() {
                    MountPhase::Mounted => "MOUNTED",
                    MountPhase::Dismounted {
                        frames_until_remounting: _,
                    } => "DISMOUNTED",
                    MountPhase::Dismounting {
                        frames_until_dismounted: _,
                    } => "DISMOUNTING",
                    MountPhase::Remounting {
                        frames_until_mounted: _,
                    } => "REMOUNTING",
                };
                assert!(
                    result_mount_state == expected_mount_state,
                    "rider {i} mount state mismatch: {} != {}",
                    result_mount_state,
                    expected_mount_state,
                );
            }

            if let Some(expected_sled_state) = &expected_entity.sled_state {
                let result_sled_state = if result_entity.sled_intact() {
                    "INTACT"
                } else {
                    "BROKEN"
                };
                assert!(
                    result_sled_state == expected_sled_state,
                    "rider {} sled state mismatch: {} != {}",
                    i,
                    result_sled_state,
                    expected_sled_state,
                );
            }

            let expected_points = &expected_entity.points;
            let result_point_positions = result_entity.point_positions();
            let result_point_velocities = result_entity.point_velocities();

            assert!(
                result_point_positions.len() >= expected_points.len(),
                "rider {} point position count mismatch: {} >= {}",
                i,
                result_point_positions.len(),
                expected_points.len(),
            );

            assert!(
                result_point_velocities.len() >= expected_points.len(),
                "rider {} point velocity count mismatch: {} >= {}",
                i,
                result_point_velocities.len(),
                expected_points.len(),
            );

            for (j, expected_point) in expected_points.iter().enumerate() {
                assert_eq!(expected_point.len(), 64, "Expected 64-character hex string");

                let pos_x =
                    f64::from_bits(u64::from_str_radix(&expected_point[0..16], 16).unwrap());
                let pos_y =
                    f64::from_bits(u64::from_str_radix(&expected_point[16..32], 16).unwrap());
                let vel_x =
                    f64::from_bits(u64::from_str_radix(&expected_point[32..48], 16).unwrap());
                let vel_y =
                    f64::from_bits(u64::from_str_radix(&expected_point[48..64], 16).unwrap());

                let (expected_position, expected_velocity) =
                    (Vector2Df::new(pos_x, pos_y), Vector2Df::new(vel_x, vel_y));

                assert!(
                    result_point_positions[j] == expected_position,
                    "rider {i} point {j} position mismatch: {:?} != {:?}",
                    result_point_positions[j],
                    expected_position,
                );

                assert!(
                    result_point_velocities[j] == expected_velocity,
                    "rider {i} point {j} velocity mismatch: {:?} != {:?}",
                    result_point_velocities[j],
                    expected_position,
                );
            }
        }
    }
}
