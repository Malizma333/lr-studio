#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use geometry::Point;
    use lr_format_core::Track;
    use lr_physics_engine::{
        PhysicsEngine,
        entity_registry::{EntityState, EntityTemplateBuilder, MountPhase, RemountVersion},
        line_registry::PhysicsLineBuilder,
    };
    use lr_physics_grid::GridVersion;
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

    fn from_track(track: &Track, lra: bool) -> PhysicsEngine {
        let grid_version = match track.grid_version() {
            lr_format_core::GridVersion::V6_0 => GridVersion::V6_0,
            lr_format_core::GridVersion::V6_1 => GridVersion::V6_1,
            lr_format_core::GridVersion::V6_2 => GridVersion::V6_2,
        };

        let mut engine = PhysicsEngine::new(grid_version);

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

        let template_none_id = engine
            .register_entity_template(EntityTemplateBuilder::default_rider(RemountVersion::None));
        let template_comv1_id = engine
            .register_entity_template(EntityTemplateBuilder::default_rider(RemountVersion::ComV1));
        let template_comv2_id = engine
            .register_entity_template(EntityTemplateBuilder::default_rider(RemountVersion::ComV2));
        let template_lra_id = engine
            .register_entity_template(EntityTemplateBuilder::default_rider(RemountVersion::LRA));

        for rider in track.riders() {
            let template_id = if lra {
                template_lra_id
            } else {
                match rider.remount_version() {
                    lr_format_core::RemountVersion::None => template_none_id,
                    lr_format_core::RemountVersion::ComV1 => template_comv1_id,
                    lr_format_core::RemountVersion::ComV2 => template_comv2_id,
                    lr_format_core::RemountVersion::LRA => template_lra_id,
                }
            };

            let entity_id = engine
                .add_entity(template_id)
                .expect("Template id should be valid");

            if let Some(offset) = rider.start_offset() {
                engine
                    .set_entity_initial_offset(entity_id, offset)
                    .expect("Entity id should be valid");
            }

            if let Some(velocity) = rider.start_velocity() {
                engine
                    .set_entity_initial_velocity(entity_id, velocity)
                    .expect("Entity id should be valid");
            }
        }

        engine
    }

    #[test]
    fn engine_fixtures() {
        let data = fs::read_to_string("../fixtures/lr_physics_engine/tests/fixture_data.json")
            .expect("Failed to read JSON file");
        let test_cases: Vec<EngineTestCase> =
            serde_json::from_str(&data).expect("Failed to parse JSON");

        let mut last_test_file = String::new();
        let mut engine = PhysicsEngine::new(GridVersion::V6_2);

        for (i, test) in test_cases.iter().enumerate() {
            println!("Engine test {}: {}", i, test.test);

            if last_test_file != test.file {
                let file_name = format!(
                    "../fixtures/lr_physics_engine/tests/{}.track.json",
                    test.file
                );
                let file = fs::read(file_name).expect("Failed to read JSON file");
                let track = lr_format_json::read(&file).expect("Failed to parse track file");
                let enable_lra = test.lra.is_some_and(|lra| lra);
                engine = from_track(&track, enable_lra);
                last_test_file = test.file.clone();
            }

            compare_states(engine.view_frame(test.frame), &test.state);
        }
    }

    fn compare_states(result: Vec<EntityState>, expected: &EngineTestCaseState) {
        let expected_entities = &expected.entities;
        assert!(
            result.len() == expected_entities.len(),
            "entity count mismatch: {} != {}",
            result.len(),
            expected_entities.len(),
        );
        for (i, expected_entity) in expected_entities.iter().enumerate() {
            let result_entity = &result.get(i).unwrap();
            if let Some(expected_mount_state) = &expected_entity.mount_state {
                let result_mount_state = match result_entity.mount_phase() {
                    MountPhase::Mounted => "MOUNTED",
                    MountPhase::Dismounted { .. } => "DISMOUNTED",
                    MountPhase::Dismounting { .. } => "DISMOUNTING",
                    MountPhase::Remounting { .. } => "REMOUNTING",
                };
                assert_eq!(
                    result_mount_state, expected_mount_state,
                    "rider {i} mount state mismatch",
                );
            }

            if let Some(expected_sled_state) = &expected_entity.sled_state {
                let result_sled_state = if result_entity.sled_intact() {
                    "INTACT"
                } else {
                    "BROKEN"
                };
                assert_eq!(
                    result_sled_state, expected_sled_state,
                    "rider {i} sled state mismatch",
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
                    (Point::new(pos_x, pos_y), Vector2Df::new(vel_x, vel_y));

                assert_eq!(
                    *result_point_positions.get(j).unwrap(),
                    expected_position,
                    "rider {i} point {j} position mismatch",
                );

                assert_eq!(
                    *result_point_velocities.get(j).unwrap(),
                    expected_velocity,
                    "rider {i} point {j} velocity mismatch",
                );
            }
        }
    }
}
