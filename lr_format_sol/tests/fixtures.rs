#[cfg(test)]
mod test {
    use geometry::{Line, Point};
    use lr_format_core::{
        GridVersion, RemountVersion, RiderBuilder, SceneryLineBuilder, StandardLineBuilder,
        TrackBuilder,
    };
    use pretty_assertions::assert_eq;
    use std::fs;
    use vector2d::Vector2Df;

    #[test]
    fn all_features() {
        let file_name = "../fixtures/lr_format_sol/all_features.sol";
        let file = fs::read(file_name).expect("Failed to read SOL file");
        let result = lr_format_sol::read(&file, None).expect("Failed to parse track file");
        let mut expected = TrackBuilder::new(GridVersion::V6_2);
        expected.title("test".to_string());

        let mut line = StandardLineBuilder::new(Line::new(
            Point::new(280.1, 155.0),
            Point::new(324.15, 164.2),
        ));
        line.flipped(false);
        line.left_extension(false);
        line.right_extension(false);
        expected.standard_lines().push(line);

        let mut line = StandardLineBuilder::new(Line::new(
            Point::new(291.45, 206.85),
            Point::new(343.35, 200.45),
        ));
        line.flipped(false);
        line.right_extension(false);
        line.left_extension(false);
        line.multiplier(1.0);
        expected.standard_lines().push(line);

        let line = SceneryLineBuilder::new(Line::new(
            Point::new(366.8, 174.9),
            Point::new(398.1, 122.25),
        ));
        expected.scenery_lines().push(line);

        let mut rider = RiderBuilder::new(RemountVersion::None);
        rider.start_offset(Vector2Df::new(280.1, 130.0));
        rider.start_velocity(Vector2Df::new(0.4, 0.0));
        expected.riders().push(rider);

        assert_eq!(result, expected.build());
    }

    #[test]
    fn multi_track_61() {
        let file_name = "../fixtures/lr_format_sol/multi_track_61.sol";
        let file = fs::read(file_name).expect("Failed to read SOL file");
        let result = lr_format_sol::read(&file, Some(0)).expect("Failed to parse track file");
        let mut expected = TrackBuilder::new(GridVersion::V6_1);
        expected.title("testv2".to_string());

        let mut line = StandardLineBuilder::new(Line::new(
            Point::new(333.35, 296.85),
            Point::new(457.85, 294.35),
        ));
        line.flipped(false);
        line.left_extension(false);
        line.right_extension(false);
        expected.standard_lines().push(line);

        let mut rider = RiderBuilder::new(RemountVersion::None);
        rider.start_offset(Vector2Df::new(333.35, 271.85));
        rider.start_velocity(Vector2Df::new(0.4, 0.0));
        expected.riders().push(rider);

        assert_eq!(result, expected.build());

        let result = lr_format_sol::read(&file, Some(1)).expect("Failed to parse track file");
        let mut expected = TrackBuilder::new(GridVersion::V6_1);
        expected.title("test".to_string());

        let mut line = StandardLineBuilder::new(Line::new(
            Point::new(335.85, 230.2),
            Point::new(403.75, 245.3),
        ));
        line.flipped(false);
        line.left_extension(false);
        line.right_extension(false);
        expected.standard_lines().push(line);

        let mut rider = RiderBuilder::new(RemountVersion::None);
        rider.start_offset(Vector2Df::new(335.85, 205.2));
        rider.start_velocity(Vector2Df::new(0.4, 0.0));
        expected.riders().push(rider);

        assert_eq!(result, expected.build());

        let result = lr_format_sol::read(&file, Some(2));
        assert!(result.is_err(), "There should only be two tracks present");

        let count = lr_format_sol::get_track_count(&file);
        assert_eq!(count, 2, "Count should match number of tracks present");
    }

    #[test]
    fn zero_start_line_props() {
        let file_name = "../fixtures/lr_format_sol/zero_start_line_props.sol";
        let file = fs::read(file_name).expect("Failed to read SOL file");
        let result = lr_format_sol::read(&file, None).expect("Failed to parse track file");
        let mut expected = TrackBuilder::new(GridVersion::V6_2);
        expected.title("zero_start_61".to_string());

        let mut line = StandardLineBuilder::new(Line::new(
            Point::new(47.066850789344755, 0.6404081606817975),
            Point::new(34.77192384246353, 7.0598832922392525),
        ));
        line.flipped(true);
        line.left_extension(false);
        line.right_extension(true);
        expected.standard_lines().push(line);

        let mut line = StandardLineBuilder::new(Line::new(
            Point::new(27.590816068178903, 8.855160235810395),
            Point::new(6.428309066688641, 8.311136919576711),
        ));
        line.flipped(true);
        line.left_extension(true);
        line.right_extension(true);
        expected.standard_lines().push(line);

        let mut line = StandardLineBuilder::new(Line::new(
            Point::new(-0.3719823862323892, 6.951078628992505),
            Point::new(-17.508716847593398, 2.3812827726295716),
        ));
        line.flipped(true);
        line.left_extension(true);
        line.right_extension(false);
        expected.standard_lines().push(line);

        let mut rider = RiderBuilder::new(RemountVersion::None);
        rider.start_offset(Vector2Df::zero());
        rider.start_velocity(Vector2Df::zero());
        expected.riders().push(rider);

        assert_eq!(result, expected.build());
    }

    #[test]
    fn no_grid_version() {
        let file_name = "../fixtures/lr_format_sol/no_grid_version.sol";
        let file = fs::read(file_name).expect("Failed to read SOL file");
        let result = lr_format_sol::read(&file, None).expect("Failed to parse track file");
        let mut expected = TrackBuilder::new(GridVersion::V6_0);
        expected.title("60_save".to_string());

        let mut line = StandardLineBuilder::new(Line::new(
            Point::new(308.2, 202.5),
            Point::new(333.35, 208.8),
        ));
        line.flipped(false);
        line.left_extension(false);
        line.right_extension(false);
        expected.standard_lines().push(line);

        let mut rider = RiderBuilder::new(RemountVersion::None);
        rider.start_offset(Vector2Df::new(308.2, 177.5));
        rider.start_velocity(Vector2Df::new(0.4, 0.0));
        expected.riders().push(rider);

        assert_eq!(result, expected.build());
    }
}
