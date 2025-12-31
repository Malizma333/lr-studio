#[cfg(test)]
mod tests {
    use format_core::track::TrackBuilder;
    use format_json::{read, write};
    use lcg_random::Random;
    use spatial_grid::GridVersion;
    use vector2d::Vector2Df;

    #[test]
    fn label() {
        let mut rng = Random::new();

        let mut track_builder = TrackBuilder::new(GridVersion::V6_2);
        let random_string = dbg!(rng.rand_str(10));
        track_builder.metadata().title(random_string);
        let expected = track_builder.build();
        let result = read(write(&expected).unwrap()).unwrap();
        assert_eq!(
            result.metadata().title(),
            expected.metadata().title(),
            "Label should be parsed correctly"
        );

        let track_builder = TrackBuilder::new(GridVersion::V6_2);
        let expected = track_builder.build();
        let result = read(write(&expected).unwrap()).unwrap();
        assert_eq!(
            result.metadata().title(),
            expected.metadata().title(),
            "No label should remain None"
        );
    }

    #[test]
    fn version() {
        for version in vec![GridVersion::V6_1, GridVersion::V6_2] {
            let track_builder = TrackBuilder::new(version);
            let expected = track_builder.build();
            let result = read(write(&expected).unwrap()).unwrap();
            assert_eq!(
                result.metadata().grid_version(),
                expected.metadata().grid_version(),
                "Grid version should be correct"
            );
        }
        let track_builder = TrackBuilder::new(GridVersion::V6_0);
        let expected = track_builder.build();
        let result = write(&expected);
        assert!(result.is_err(), "6.0 grid should give error");
    }

    #[test]
    fn start_position() {
        let mut rng = Random::new();

        let mut track_builder = TrackBuilder::new(GridVersion::V6_2);
        let start_x = dbg!(rng.rand_range_f64(-1000.0, 1000.0).unwrap());
        let start_y = dbg!(rng.rand_range_f64(-1000.0, 1000.0).unwrap());
        track_builder
            .metadata()
            .start_position(Vector2Df::new(start_x, start_y));
        let expected = track_builder.build();
        let result = read(write(&expected).unwrap()).unwrap();
        assert_eq!(
            result.metadata().start_position(),
            expected.metadata().start_position(),
            "Start position should be parsed correctly"
        );

        let track_builder = TrackBuilder::new(GridVersion::V6_2);
        let expected = track_builder.build();
        let result = read(write(&expected).unwrap()).unwrap();
        assert_eq!(
            result.metadata().start_position(),
            Some(Vector2Df::zero()),
            "No start position should be zero vector"
        );
    }
}
