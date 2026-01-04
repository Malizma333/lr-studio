#[cfg(test)]
mod test {
    use std::fs;

    use format_core::track::{GridVersion, RemountVersion, TrackBuilder};
    use pretty_assertions::assert_eq;
    use vector2d::Vector2Df;

    #[test]
    fn all_features() {
        let file_name = "../fixtures/format_sol/all_features.sol";
        let file = fs::read(file_name).expect("Failed to read SOL file");
        let result = format_sol::read(&file, None).expect("Failed to parse track file");
        let mut expected_builder = TrackBuilder::new(GridVersion::V6_2);
        expected_builder.metadata().title("test".to_string());
        expected_builder
            .line_group()
            .add_standard_line(
                0,
                (Vector2Df::new(280.1, 155.0), Vector2Df::new(324.15, 164.2)),
            )
            .flipped(false)
            .left_extension(false)
            .right_extension(false);
        expected_builder
            .line_group()
            .add_acceleration_line(
                1,
                (
                    Vector2Df::new(291.45, 206.85),
                    Vector2Df::new(343.35, 200.45),
                ),
            )
            .flipped(false)
            .left_extension(false)
            .right_extension(false);
        expected_builder
            .line_group()
            .add_scenery_line((Vector2Df::new(366.8, 174.9), Vector2Df::new(398.1, 122.25)));
        expected_builder
            .rider_group()
            .add_rider(RemountVersion::None, 0)
            .start_angle(0.0)
            .start_position(Vector2Df::new(280.1, 130.0))
            .start_velocity(Vector2Df::new(0.4, 0.0));
        let expected = expected_builder.build();
        assert_eq!(result, expected);
    }

    #[test]
    fn multi_track_61() {
        let file_name = "../fixtures/format_sol/multi_track_61.sol";
        let file = fs::read(file_name).expect("Failed to read SOL file");
        let result = format_sol::read(&file, Some(0)).expect("Failed to parse track file");
        let mut expected_builder = TrackBuilder::new(GridVersion::V6_1);
        expected_builder.metadata().title("testv2".to_string());
        expected_builder
            .line_group()
            .add_standard_line(
                5,
                (
                    Vector2Df::new(333.35, 296.85),
                    Vector2Df::new(457.85, 294.35),
                ),
            )
            .flipped(false)
            .left_extension(false)
            .right_extension(false);
        expected_builder
            .rider_group()
            .add_rider(RemountVersion::None, 0)
            .start_angle(0.0)
            .start_position(Vector2Df::new(333.35, 271.85))
            .start_velocity(Vector2Df::new(0.4, 0.0));
        let expected = expected_builder.build();
        assert_eq!(result, expected);

        let result = format_sol::read(&file, Some(1)).expect("Failed to parse track file");
        let mut expected_builder = TrackBuilder::new(GridVersion::V6_1);
        expected_builder.metadata().title("test".to_string());
        expected_builder
            .line_group()
            .add_standard_line(
                4,
                (Vector2Df::new(335.85, 230.2), Vector2Df::new(403.75, 245.3)),
            )
            .flipped(false)
            .left_extension(false)
            .right_extension(false);
        expected_builder
            .rider_group()
            .add_rider(RemountVersion::None, 0)
            .start_angle(0.0)
            .start_position(Vector2Df::new(335.85, 205.2))
            .start_velocity(Vector2Df::new(0.4, 0.0));
        let expected = expected_builder.build();
        assert_eq!(result, expected);

        let result = format_sol::read(&file, Some(2));
        assert!(result.is_err(), "There should only be two tracks present");

        let count = format_sol::get_track_count(&file);
        assert_eq!(count, 2, "Count should match number of tracks present");
    }

    #[test]
    fn zero_start_line_props() {
        let file_name = "../fixtures/format_sol/zero_start_line_props.sol";
        let file = fs::read(file_name).expect("Failed to read SOL file");
        let result = format_sol::read(&file, None).expect("Failed to parse track file");
        let mut expected_builder = TrackBuilder::new(GridVersion::V6_2);
        expected_builder
            .metadata()
            .title("zero_start_61".to_string());
        expected_builder
            .line_group()
            .add_standard_line(
                2,
                (
                    Vector2Df::new(47.066850789344755, 0.6404081606817975),
                    Vector2Df::new(34.77192384246353, 7.0598832922392525),
                ),
            )
            .flipped(true)
            .left_extension(false)
            .right_extension(true);
        expected_builder
            .line_group()
            .add_standard_line(
                3,
                (
                    Vector2Df::new(27.590816068178903, 8.855160235810395),
                    Vector2Df::new(6.428309066688641, 8.311136919576711),
                ),
            )
            .flipped(true)
            .left_extension(true)
            .right_extension(true);
        expected_builder
            .line_group()
            .add_standard_line(
                4,
                (
                    Vector2Df::new(-0.3719823862323892, 6.951078628992505),
                    Vector2Df::new(-17.508716847593398, 2.3812827726295716),
                ),
            )
            .flipped(true)
            .left_extension(true)
            .right_extension(false);
        expected_builder
            .rider_group()
            .add_rider(RemountVersion::None, 0)
            .start_angle(0.0)
            .start_position(Vector2Df::new(0.0, 0.0))
            .start_velocity(Vector2Df::new(0.0, 0.0));
        let expected = expected_builder.build();
        assert_eq!(result, expected);
    }

    #[test]
    fn no_grid_version() {
        let file_name = "../fixtures/format_sol/no_grid_version.sol";
        let file = fs::read(file_name).expect("Failed to read SOL file");
        let result = format_sol::read(&file, None).expect("Failed to parse track file");
        let mut expected_builder = TrackBuilder::new(GridVersion::V6_0);
        expected_builder.metadata().title("60_save".to_string());
        expected_builder
            .line_group()
            .add_standard_line(
                0,
                (Vector2Df::new(308.2, 202.5), Vector2Df::new(333.35, 208.8)),
            )
            .flipped(false)
            .left_extension(false)
            .right_extension(false);
        expected_builder
            .rider_group()
            .add_rider(RemountVersion::None, 0)
            .start_angle(0.0)
            .start_position(Vector2Df::new(308.2, 177.5))
            .start_velocity(Vector2Df::new(0.4, 0.0));
        let expected = expected_builder.build();
        assert_eq!(result, expected);
    }
}
