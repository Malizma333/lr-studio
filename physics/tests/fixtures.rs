#[cfg(test)]
mod tests {
    use format_core::track::GridVersion;
    use format_json;
    use physics::{Engine, EngineView, MountPhase};
    use serde::Deserialize;
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
        let data = fs::read_to_string("tests/fixture_data.json").expect("Failed to read JSON file");
        let mut test_cases: Vec<EngineTestCase> =
            serde_json::from_str(&data).expect("Failed to parse JSON");

        test_cases.sort_by_key(|test_case| test_case.file.clone());
        let mut last_test_file = String::new();
        let mut engine = Engine::new(GridVersion::V6_2);

        for (i, test) in test_cases.iter().enumerate() {
            println!("Engine test {}: {}", i, test.test);

            if &last_test_file != &test.file {
                let file_name = format!("../fixtures/physics/tests/{}.track.json", test.file);
                let file = fs::read(file_name).expect("Failed to read JSON file");
                let track = format_json::read(&file).expect("Failed to parse track file");
                let enable_lra = test.lra.is_some_and(|lra| lra);
                engine = Engine::from_track(track, enable_lra);
                last_test_file = test.file.clone();
            }

            compare_states(engine.view_frame(test.frame), &test.state);
        }
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
