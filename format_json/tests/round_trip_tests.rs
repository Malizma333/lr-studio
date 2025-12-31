/// Does fuzz testing of serializing then deserializing data,
/// based on the .track.json spec. Defaults are chosen to best
/// match linerider.com compatibility (e.g. start position must
/// not be undefined).
#[cfg(test)]
mod tests {
    use format_core::track::{RemountVersion, TrackBuilder};
    use format_json::{read, write};
    use lcg_random::Random;
    use spatial_grid::GridVersion;
    use vector2d::Vector2Df;

    #[test]
    fn label() {
        let mut rng = Random::new();

        let mut track_builder = TrackBuilder::new(GridVersion::V6_2);
        let expected_string = dbg!(rng.rand_str(10));
        track_builder.metadata().title(expected_string.clone());
        let track = track_builder.build();
        let result = read(&write(&track).unwrap()).unwrap();
        assert_eq!(
            result.metadata().title().as_ref().unwrap(),
            &expected_string,
            "Label should be parsed correctly"
        );

        let track_builder = TrackBuilder::new(GridVersion::V6_2);
        let track = track_builder.build();
        let result = read(&write(&track).unwrap()).unwrap();
        assert!(
            result.metadata().title().is_none(),
            "No label should remain None"
        );
    }

    #[test]
    fn version() {
        for version in vec![GridVersion::V6_1, GridVersion::V6_2] {
            let track_builder = TrackBuilder::new(version);
            let expected = track_builder.build();
            let result = read(&write(&expected).unwrap()).unwrap();
            assert_eq!(
                result.metadata().grid_version(),
                expected.metadata().grid_version(),
                "Grid version should be correct"
            );
        }
        let track_builder = TrackBuilder::new(GridVersion::V6_0);
        let track = track_builder.build();
        let result = write(&track);
        assert!(result.is_err(), "6.0 grid should give error");
    }

    #[test]
    fn start_position() {
        let mut rng = Random::new();

        let mut track_builder = TrackBuilder::new(GridVersion::V6_2);
        let start_x = dbg!(rng.rand_range_f64(-1000.0, 1000.0).unwrap());
        let start_y = dbg!(rng.rand_range_f64(-1000.0, 1000.0).unwrap());
        let expected_position = Vector2Df::new(start_x, start_y);
        track_builder.metadata().start_position(expected_position);
        let track = track_builder.build();
        let result = read(&write(&track).unwrap()).unwrap();
        assert_eq!(
            result.metadata().start_position().unwrap(),
            expected_position,
            "Start position should be parsed correctly"
        );

        let track_builder = TrackBuilder::new(GridVersion::V6_2);
        let track = track_builder.build();
        let result = read(&write(&track).unwrap()).unwrap();
        assert_eq!(
            result.metadata().start_position().unwrap(),
            Vector2Df::zero(),
            "No start position should be zero vector"
        );
    }

    #[test]
    fn creator() {
        let mut rng = Random::new();

        let mut track_builder = TrackBuilder::new(GridVersion::V6_2);
        let expected_string = dbg!(rng.rand_str(10));
        track_builder.metadata().artist(expected_string.clone());
        let track = track_builder.build();
        let result = read(&write(&track).unwrap()).unwrap();
        assert_eq!(
            result.metadata().artist().as_ref().unwrap(),
            &expected_string,
            "Creator should be parsed correctly"
        );

        let track_builder = TrackBuilder::new(GridVersion::V6_2);
        let track = track_builder.build();
        let result = read(&write(&track).unwrap()).unwrap();
        assert!(
            result.metadata().artist().is_none(),
            "No creator should remain None"
        );
    }

    #[test]
    fn description() {
        let mut rng = Random::new();

        let mut track_builder = TrackBuilder::new(GridVersion::V6_2);
        let expected_string = dbg!(rng.rand_str(10));
        track_builder
            .metadata()
            .description(expected_string.clone());
        let track = track_builder.build();
        let result = read(&write(&track).unwrap()).unwrap();
        assert_eq!(
            result.metadata().description().as_ref().unwrap(),
            &expected_string,
            "Description should be parsed correctly"
        );

        let track_builder = TrackBuilder::new(GridVersion::V6_2);
        let track = track_builder.build();
        let result = read(&write(&track).unwrap()).unwrap();
        assert!(
            result.metadata().description().is_none(),
            "No description should remain None"
        );
    }

    #[test]
    fn script() {
        let mut rng = Random::new();

        let mut track_builder = TrackBuilder::new(GridVersion::V6_2);
        let expected_string = dbg!(rng.rand_str(1000));
        track_builder.metadata().script(expected_string.clone());
        let track = track_builder.build();
        let result = read(&write(&track).unwrap()).unwrap();
        assert_eq!(
            result.metadata().script().as_ref().unwrap(),
            &expected_string,
            "Script should be parsed correctly"
        );

        let track_builder = TrackBuilder::new(GridVersion::V6_2);
        let track = track_builder.build();
        let result = read(&write(&track).unwrap()).unwrap();
        assert!(
            result.metadata().script().is_none(),
            "No script should remain None"
        );
    }

    #[test]
    fn duration() {
        let mut rng = Random::new();

        let mut track_builder = TrackBuilder::new(GridVersion::V6_2);
        let random_number = dbg!(rng.rand_range_u32(0, 10000).unwrap());
        track_builder.metadata().duration(random_number);
        let track = track_builder.build();
        let result = read(&write(&track).unwrap()).unwrap();
        assert_eq!(
            result.metadata().duration().unwrap(),
            random_number,
            "Duration should be parsed correctly"
        );

        let track_builder = TrackBuilder::new(GridVersion::V6_2);
        let expected = track_builder.build();
        let result = read(&write(&expected).unwrap()).unwrap();
        assert!(
            result.metadata().duration().is_none(),
            "No duration should remain None"
        );
    }

    #[test]
    fn zero_start() {
        let mut track_builder = TrackBuilder::new(GridVersion::V6_2);
        track_builder.metadata().zero_velocity_start_riders(true);
        let track = track_builder.build();
        let result = read(&write(&track).unwrap()).unwrap();
        assert_eq!(
            result.rider_group().as_ref().unwrap().riders()[0]
                .start_velocity()
                .unwrap(),
            Vector2Df::zero(),
            "Zero start should impact start velocity of riders"
        );
    }

    #[test]
    fn riders() {
        let track_builder = TrackBuilder::new(GridVersion::V6_2);
        let track = track_builder.build();
        let mut expected_builder = TrackBuilder::new(GridVersion::V6_2);
        expected_builder
            .rider_group()
            .add_rider(RemountVersion::ComV2)
            .start_angle(0.0)
            .start_position(Vector2Df::zero())
            .start_velocity(Vector2Df::new(0.4, 0.0));
        let expected = expected_builder.build();
        let result = read(&write(&track).unwrap()).unwrap();
        assert_eq!(
            result.rider_group().as_ref().unwrap().riders().len(),
            1,
            "Uninitialized riders should be initialized with default rider"
        );
        assert_eq!(
            result.rider_group().as_ref().unwrap().riders()[0],
            expected.rider_group().as_ref().unwrap().riders()[0],
            "Uninitialized riders props should match"
        );

        // start position
        // start velocity
        // remountable version 0
        // remountable version 1
        // invalid remount version (lra)
    }

    #[test]
    fn layers() {
        let track_builder = TrackBuilder::new(GridVersion::V6_2);
        let track = track_builder.build();
        let mut expected_builder = TrackBuilder::new(GridVersion::V6_2);
        expected_builder
            .layer_group()
            .add_layer(0, 0)
            .editable(true)
            .visible(true)
            .name("Base Layer".to_string());
        let expected = expected_builder.build();
        let result = read(&write(&track).unwrap()).unwrap();
        assert_eq!(
            result.layer_group().as_ref().unwrap().layers().len(),
            1,
            "Uninitialized layers should be initialized with base layer"
        );
        assert_eq!(
            result.layer_group().as_ref().unwrap().layers()[0],
            expected.layer_group().as_ref().unwrap().layers()[0],
            "Uninitialized base layer props should match"
        );
        assert!(
            result
                .layer_group()
                .as_ref()
                .unwrap()
                .layer_folders()
                .is_none(),
            "Uninitialized layers should have disabled layer folders"
        );

        let mut track_builder = TrackBuilder::new(GridVersion::V6_2);
        track_builder
            .layer_group()
            .add_layer(0, 0)
            .name("test".to_string())
            .visible(true)
            .editable(true)
            .folder_id(1);
        let expected = track_builder.build();
        let result = read(&write(&expected).unwrap()).unwrap();
        assert_eq!(
            result.layer_group().as_ref().unwrap().layers()[0],
            expected.layer_group().as_ref().unwrap().layers()[0],
            "Initialized layer should match"
        );
        assert_eq!(
            result
                .layer_group()
                .as_ref()
                .unwrap()
                .layer_folders()
                .as_ref()
                .unwrap()
                .len(),
            0,
            "Layer with folder id without layer folders should enable layer folder feature"
        );
    }

    #[test]
    fn lines() {
        let track_builder = TrackBuilder::new(GridVersion::V6_2);
        let expected = track_builder.build();
        let result = read(&write(&expected).unwrap()).unwrap();
        assert_eq!(
            result.line_group().standard_lines().len()
                + result.line_group().acceleration_lines().len()
                + result.line_group().scenery_lines().len(),
            0,
            "No lines should be empty"
        );
    }
}
