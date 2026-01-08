#[cfg(test)]
mod test {
    use geometry::{Line, Point};
    use lr_format_core::{GridVersion, RemountVersion, Rider, SceneryLine, StandardLine, Track};
    use pretty_assertions::assert_eq;
    use std::fs;
    use vector2d::Vector2Df;

    #[test]
    fn all_features() {
        let file_name = "../fixtures/lr_format_sol/all_features.sol";
        let file = fs::read(file_name).expect("Failed to read SOL file");
        let result = lr_format_sol::read(&file, None).expect("Failed to parse track file");
        let mut expected_track = Track::new(GridVersion::V6_2);
        expected_track.set_title("test".to_string());

        let mut line = StandardLine::new(Line::new(
            Point::new(280.1, 155.0),
            Point::new(324.15, 164.2),
        ));
        line.set_flipped(false);
        line.set_left_extension(false);
        line.set_right_extension(false);
        expected_track.standard_lines_mut().push(line);

        let mut line = StandardLine::new(Line::new(
            Point::new(291.45, 206.85),
            Point::new(343.35, 200.45),
        ));
        line.set_flipped(false);
        line.set_right_extension(false);
        line.set_left_extension(false);
        line.set_multiplier(1.0);
        expected_track.standard_lines_mut().push(line);

        let line = SceneryLine::new(Line::new(
            Point::new(366.8, 174.9),
            Point::new(398.1, 122.25),
        ));
        expected_track.scenery_lines_mut().push(line);

        let mut rider = Rider::new(RemountVersion::None);
        rider.set_start_offset(Vector2Df::new(280.1, 130.0));
        rider.set_start_velocity(Vector2Df::new(0.4, 0.0));
        expected_track.riders_mut().push(rider);

        assert_eq!(result, expected_track);
    }

    #[test]
    fn multi_track_61() {
        let file_name = "../fixtures/lr_format_sol/multi_track_61.sol";
        let file = fs::read(file_name).expect("Failed to read SOL file");
        let result = lr_format_sol::read(&file, Some(0)).expect("Failed to parse track file");
        let mut expected_track = Track::new(GridVersion::V6_1);
        expected_track.set_title("testv2".to_string());

        let mut line = StandardLine::new(Line::new(
            Point::new(333.35, 296.85),
            Point::new(457.85, 294.35),
        ));
        line.set_flipped(false);
        line.set_left_extension(false);
        line.set_right_extension(false);
        expected_track.standard_lines_mut().push(line);

        let mut rider = Rider::new(RemountVersion::None);
        rider.set_start_offset(Vector2Df::new(333.35, 271.85));
        rider.set_start_velocity(Vector2Df::new(0.4, 0.0));
        expected_track.riders_mut().push(rider);

        assert_eq!(result, expected_track);

        let result = lr_format_sol::read(&file, Some(1)).expect("Failed to parse track file");
        let mut expected_track = Track::new(GridVersion::V6_1);
        expected_track.set_title("test".to_string());

        let mut line = StandardLine::new(Line::new(
            Point::new(335.85, 230.2),
            Point::new(403.75, 245.3),
        ));
        line.set_flipped(false);
        line.set_left_extension(false);
        line.set_right_extension(false);
        expected_track.standard_lines_mut().push(line);

        let mut rider = Rider::new(RemountVersion::None);
        rider.set_start_offset(Vector2Df::new(335.85, 205.2));
        rider.set_start_velocity(Vector2Df::new(0.4, 0.0));
        expected_track.riders_mut().push(rider);

        assert_eq!(result, expected_track);

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
        let mut expected_track = Track::new(GridVersion::V6_2);
        expected_track.set_title("zero_start_61".to_string());

        let mut line = StandardLine::new(Line::new(
            Point::new(47.066850789344755, 0.6404081606817975),
            Point::new(34.77192384246353, 7.0598832922392525),
        ));
        line.set_flipped(true);
        line.set_left_extension(false);
        line.set_right_extension(true);
        expected_track.standard_lines_mut().push(line);

        let mut line = StandardLine::new(Line::new(
            Point::new(27.590816068178903, 8.855160235810395),
            Point::new(6.428309066688641, 8.311136919576711),
        ));
        line.set_flipped(true);
        line.set_left_extension(true);
        line.set_right_extension(true);
        expected_track.standard_lines_mut().push(line);

        let mut line = StandardLine::new(Line::new(
            Point::new(-0.3719823862323892, 6.951078628992505),
            Point::new(-17.508716847593398, 2.3812827726295716),
        ));
        line.set_flipped(true);
        line.set_left_extension(true);
        line.set_right_extension(false);
        expected_track.standard_lines_mut().push(line);

        let mut rider = Rider::new(RemountVersion::None);
        rider.set_start_offset(Vector2Df::zero());
        rider.set_start_velocity(Vector2Df::zero());
        expected_track.riders_mut().push(rider);

        assert_eq!(result, expected_track);
    }

    #[test]
    fn no_grid_version() {
        let file_name = "../fixtures/lr_format_sol/no_grid_version.sol";
        let file = fs::read(file_name).expect("Failed to read SOL file");
        let result = lr_format_sol::read(&file, None).expect("Failed to parse track file");
        let mut expected_track = Track::new(GridVersion::V6_0);
        expected_track.set_title("60_save".to_string());

        let mut line = StandardLine::new(Line::new(
            Point::new(308.2, 202.5),
            Point::new(333.35, 208.8),
        ));
        line.set_flipped(false);
        line.set_left_extension(false);
        line.set_right_extension(false);
        expected_track.standard_lines_mut().push(line);

        let mut rider = Rider::new(RemountVersion::None);
        rider.set_start_offset(Vector2Df::new(308.2, 177.5));
        rider.set_start_velocity(Vector2Df::new(0.4, 0.0));
        expected_track.riders_mut().push(rider);

        assert_eq!(result, expected_track);
    }
}
