#[cfg(test)]
mod tests {
    use format_core::track::GridVersion as FormatGridVersion;
    use format_json;
    use geometry::Point;
    use physics::{
        AccelerationLine as PhysicsAccelerationLine, EngineBuilder, EngineState,
        GridVersion as PhysicsGridVersion, MountPhase, NormalLine as PhysicsNormalLine,
    };
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
        file: String,
        state: EngineTestCaseState,
    }

    #[ignore = "engine not implemented"]
    #[test]
    fn engine_fixtures() {
        let data =
            fs::read_to_string("tests/fixture_tests.json").expect("Failed to read JSON file");
        let test_cases: Vec<EngineTestCase> =
            serde_json::from_str(&data).expect("Failed to parse JSON");

        for test in test_cases {
            println!("Running test {}", test.test);

            let file_name = format!("tests/fixtures/{}.track.json", test.file);
            let file = fs::read(file_name).expect("Failed to read JSON file");
            let track = format_json::read(file).expect("Failed to parse track file");

            // TODO duplication across libraries
            let version = match track.metadata().grid_version() {
                FormatGridVersion::V6_0 => PhysicsGridVersion::V6_0,
                FormatGridVersion::V6_1 => PhysicsGridVersion::V6_1,
                FormatGridVersion::V6_2 => PhysicsGridVersion::V6_2,
            };
            let mut engine = EngineBuilder::new(version).build();
            for line in track.line_group().acceleration_lines() {
                let acceleration_line = PhysicsAccelerationLine::new(
                    (
                        Point::new(line.x1(), line.y1()),
                        Point::new(line.x2(), line.y2()),
                    ),
                    line.flipped(),
                    line.left_extension(),
                    line.right_extension(),
                    line.multiplier().unwrap_or(1.0),
                );
                engine.create_line(Box::new(acceleration_line));
            }

            for line in track.line_group().standard_lines() {
                let normal_line = PhysicsNormalLine::new(
                    (
                        Point::new(line.x1(), line.y1()),
                        Point::new(line.x2(), line.y2()),
                    ),
                    line.flipped(),
                    line.left_extension(),
                    line.right_extension(),
                );
                engine.create_line(Box::new(normal_line));
            }

            let frame_data = engine.view_frame(test.frame);

            compare_states(frame_data, &test.state);
        }
    }

    fn compare_states(result_state: &EngineState, expected_state: &EngineTestCaseState) {
        let expected_entities = &expected_state.entities;
        let result_entities = result_state.skeletons();
        assert!(
            result_entities.len() == expected_entities.len(),
            "entity count mismatch"
        );
        for (i, expected_entity) in expected_entities.iter().enumerate() {
            let result_entity = result_entities[i];
            if let Some(expected_mount_state) = &expected_entity.mount_state {
                let result_mount_state = match result_entity.mount_phase() {
                    MountPhase::Mounted => "MOUNTED",
                    MountPhase::Dismounted {
                        frames_until_can_remount: _,
                    } => "DISMOUNTED",
                    MountPhase::Dismounting {
                        frames_until_dismounted: _,
                    } => "DISMOUNTING",
                    MountPhase::Remounting {
                        frames_until_remounted: _,
                    } => "REMOUNTING",
                };
                assert!(
                    expected_mount_state == result_mount_state,
                    "rider {i} mount state mismatch"
                );
            }

            if let Some(expected_sled_state) = &expected_entity.sled_state {
                let result_sled_state = if result_entity.sled_intact() {
                    "INTACT"
                } else {
                    "BROKEN"
                };
                assert!(
                    expected_sled_state == result_sled_state,
                    "rider {i} sled state mismatch"
                );
            }

            let expected_points = &expected_entity.points;
            let result_points = result_entity.points();

            assert!(
                result_points.len() >= expected_points.len(),
                "rider {i} point count mismatch",
            );

            for (j, expected_point_data) in expected_points.iter().enumerate() {
                let (expected_position, expected_velocity) =
                    convert_to_vectors(expected_point_data);
                assert!(
                    expected_position == result_points[j].0,
                    "rider {i} point {j} position mismatch: {:?} != {:?}",
                    expected_position,
                    result_points[j].0,
                );
                assert!(
                    expected_velocity == result_points[j].1,
                    "rider {i} point {j} velocity mismatch: {:?} != {:?}",
                    expected_position,
                    result_points[j].1,
                );
            }
        }
    }

    fn convert_to_vectors(f64_hex_string: &String) -> (Vector2Df, Vector2Df) {
        assert_eq!(f64_hex_string.len(), 64, "Expected 64-character hex string");

        let px = f64::from_bits(u64::from_str_radix(&f64_hex_string[0..16], 16).unwrap());
        let py = f64::from_bits(u64::from_str_radix(&f64_hex_string[16..32], 16).unwrap());
        let vx = f64::from_bits(u64::from_str_radix(&f64_hex_string[32..48], 16).unwrap());
        let vy = f64::from_bits(u64::from_str_radix(&f64_hex_string[48..64], 16).unwrap());

        (Vector2Df::new(px, py), Vector2Df::new(vx, vy))
    }
}
