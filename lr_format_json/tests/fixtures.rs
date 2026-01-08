#[cfg(test)]
mod test {
    use color::RGBColor;
    use geometry::{Line, Point};
    use lr_format_core::{
        GridVersion, Layer, LayerFolder, RemountVersion, Rider, SceneryLine, StandardLine, Track,
    };
    use pretty_assertions::assert_eq;
    use std::fs;
    use vector2d::Vector2Df;

    #[test]
    fn web_features() {
        let file_name = "../fixtures/lr_format_json/web_features.track.json";
        let file = fs::read(file_name).expect("Failed to read JSON file");
        let result = lr_format_json::read(&file).expect("Failed to parse track file");
        let mut expected_track = Track::new(GridVersion::V6_2);

        expected_track.set_title(String::new());
        expected_track.set_artist(String::new());
        expected_track.set_description(String::new());
        expected_track.set_duration(1200);

        let mut rider = Rider::new(RemountVersion::None);
        rider.set_start_offset(Vector2Df::zero());
        rider.set_start_velocity(Vector2Df::new(0.4, 0.0));
        expected_track.riders_mut().push(rider);

        let mut layer = Layer::new(0);
        layer.set_name("Base Layer".to_string());
        layer.set_color(RGBColor::new(255, 0, 0));
        layer.set_visible(true);
        layer.set_editable(true);
        expected_track.layers_mut().push(layer);

        let mut layer = LayerFolder::new(1);
        layer.set_name("Folder".to_string());
        layer.set_visible(true);
        layer.set_editable(true);
        expected_track.layer_folders_mut().push(layer);

        let mut layer = Layer::new(2);
        layer.set_name(String::new());
        layer.set_visible(true);
        layer.set_editable(true);
        layer.set_folder_id(1);
        expected_track.layers_mut().push(layer);

        let mut line = StandardLine::new(Line::new(Point::zero(), Point::new(0.0, 1.0)));
        line.set_flipped(false);
        line.set_left_extension(false);
        line.set_right_extension(false);
        expected_track.standard_lines_mut().push(line);

        let mut line = StandardLine::new(Line::new(Point::zero(), Point::new(0.0, 1.0)));
        line.set_flipped(false);
        line.set_left_extension(false);
        line.set_right_extension(false);
        line.set_multiplier(1.0);
        expected_track.standard_lines_mut().push(line);

        let mut line = SceneryLine::new(Line::new(Point::zero(), Point::new(0.0, 1.0)));
        line.set_width(0.5);
        expected_track.scenery_lines_mut().push(line);

        assert_eq!(result, expected_track);
    }

    #[test]
    fn lrl_features() {
        let file_name = "../fixtures/lr_format_json/lrl_features.track.json";
        let file = fs::read(file_name).expect("Failed to read JSON file");
        let result = lr_format_json::read(&file).expect("Failed to parse track file");
        let mut expected_track = Track::new(GridVersion::V6_2);

        expected_track.set_title("test".to_string());
        // expected_track.set_start_zoom(from_lra_zoom(4.0));

        let mut rider = Rider::new(RemountVersion::LRA);
        rider.set_start_offset(Vector2Df::zero());
        rider.set_start_velocity(Vector2Df::new(0.4, 0.0));
        expected_track.riders_mut().push(rider);

        let mut line = StandardLine::new(Line::new(Point::new(-2.0, 5.0), Point::new(31.0, 9.0)));
        line.set_flipped(false);
        line.set_left_extension(false);
        line.set_right_extension(false);
        expected_track.standard_lines_mut().push(line);

        let mut line = StandardLine::new(Line::new(Point::new(40.0, 26.0), Point::new(-1.0, 25.0)));
        line.set_flipped(true);
        line.set_left_extension(false);
        line.set_right_extension(false);
        line.set_multiplier(3.0);
        expected_track.standard_lines_mut().push(line);

        let line = SceneryLine::new(Line::new(Point::new(36.0, 19.0), Point::new(88.0, 21.0)));
        expected_track.scenery_lines_mut().push(line);

        // expected_track.legacy_camera_zoom_group().add_trigger(
        //     CameraZoomEvent::new(from_lra_zoom(4.0)),
        //     LineHitTrigger::new(1, 20),
        // );

        assert_eq!(result, expected_track);
    }

    #[test]
    fn lro_features() {
        let file_name = "../fixtures/lr_format_json/lro_features.track.json";
        let file = fs::read(file_name).expect("Failed to read JSON file");
        let result = lr_format_json::read(&file).expect("Failed to parse track file");
        let mut expected_track = Track::new(GridVersion::V6_2);

        expected_track.set_title("test".to_string());
        // expected_track.set_start_zoom(from_lra_zoom(4.0))
        // expected_track.set_start_background_color(RGBColor::new(255, 255, 255))
        // expected_track.set_start_line_color(RGBColor::new(0, 0, 0))
        // expected_track.set_start_gravity(Vector2Df::up())

        let mut rider = Rider::new(RemountVersion::LRA);
        rider.set_start_offset(Vector2Df::zero());
        rider.set_start_velocity(Vector2Df::zero());
        expected_track.riders_mut().push(rider);

        let mut line =
            StandardLine::new(Line::new(Point::new(-36.0, -47.0), Point::new(-3.0, -48.0)));
        line.set_flipped(false);
        line.set_left_extension(true);
        line.set_right_extension(true);
        line.set_height(5.0);
        expected_track.standard_lines_mut().push(line);

        let mut line =
            StandardLine::new(Line::new(Point::new(1.0, -43.0), Point::new(-35.0, -42.0)));
        line.set_flipped(true);
        line.set_left_extension(true);
        line.set_right_extension(true);
        line.set_multiplier(2.0);
        line.set_height(5.0);
        expected_track.standard_lines_mut().push(line);

        let line = SceneryLine::new(Line::new(Point::new(-33.0, -33.0), Point::new(-3.0, -32.0)));
        expected_track.scenery_lines_mut().push(line);

        // expected_track.camera_zoom_group().add_trigger(
        //     CameraZoomEvent::new(from_lra_zoom(4.0)),
        //     FrameBoundsTrigger::new(0, 40),
        // );
        // expected_track.background_color_group().add_trigger(
        //     BackgroundColorEvent::new(RGBColor::new(255, 255, 255)),
        //     FrameBoundsTrigger::new(1, 41),
        // );
        // expected_track.line_color_group().add_trigger(
        //     LineColorEvent::new(RGBColor::new(0, 0, 0)),
        //     FrameBoundsTrigger::new(1, 41),
        // );

        assert_eq!(result, expected_track);
    }

    #[test]
    fn empty_track() {
        let file_name = "../fixtures/lr_format_json/empty_61.track.json";
        let file = fs::read(file_name).expect("Failed to read JSON file");
        let result = lr_format_json::read(&file).expect("Failed to parse track file");
        let mut expected_track = Track::new(GridVersion::V6_1);

        let mut rider = Rider::new(RemountVersion::LRA);
        rider.set_start_offset(Vector2Df::new(0.0, 0.0));
        rider.set_start_velocity(Vector2Df::new(0.4, 0.0));
        expected_track.riders_mut().push(rider);

        assert_eq!(result, expected_track);
    }

    #[test]
    fn remount_versions() {
        let file_name = "../fixtures/lr_format_json/remount_versions.track.json";
        let file = fs::read(file_name).expect("Failed to read JSON file");
        let result = lr_format_json::read(&file).expect("Failed to parse track file");
        let mut expected_track = Track::new(GridVersion::V6_2);

        let mut rider = Rider::new(RemountVersion::None);
        rider.set_start_offset(Vector2Df::new(0.0, 0.0));
        rider.set_start_velocity(Vector2Df::new(0.0, 0.0));
        expected_track.riders_mut().push(rider);

        let mut rider = Rider::new(RemountVersion::None);
        rider.set_start_offset(Vector2Df::new(0.0, 0.0));
        rider.set_start_velocity(Vector2Df::new(0.0, 0.0));
        expected_track.riders_mut().push(rider);

        let mut rider = Rider::new(RemountVersion::ComV1);
        rider.set_start_offset(Vector2Df::new(0.0, 0.0));
        rider.set_start_velocity(Vector2Df::new(0.0, 0.0));
        expected_track.riders_mut().push(rider);

        let mut rider = Rider::new(RemountVersion::None);
        rider.set_start_offset(Vector2Df::new(0.0, 0.0));
        rider.set_start_velocity(Vector2Df::new(0.0, 0.0));
        expected_track.riders_mut().push(rider);

        let mut rider = Rider::new(RemountVersion::ComV2);
        rider.set_start_offset(Vector2Df::new(0.0, 0.0));
        rider.set_start_velocity(Vector2Df::new(0.0, 0.0));
        expected_track.riders_mut().push(rider);

        assert_eq!(result, expected_track);
    }

    #[test]
    fn empty_layers() {
        let file_name = "../fixtures/lr_format_json/empty_layers.track.json";
        let file = fs::read(file_name).expect("Failed to read JSON file");
        let result = lr_format_json::read(&file).expect("Failed to parse track file");
        let mut expected_track = Track::new(GridVersion::V6_2);

        let mut layer = Layer::new(0);
        layer.set_name("Base Layer".to_string());
        layer.set_visible(true);
        expected_track.layers_mut().push(layer);

        let mut layer = Layer::new(1);
        layer.set_name("#invalid color".to_string());
        layer.set_visible(true);
        layer.set_editable(true);
        expected_track.layers_mut().push(layer);

        assert_eq!(result, expected_track);
    }

    #[test]
    fn line_flags() {
        let file_name = "../fixtures/lr_format_json/line_flags.track.json";
        let file = fs::read(file_name).expect("Failed to read JSON file");
        let result = lr_format_json::read(&file).expect("Failed to parse track file");
        let mut expected_track = Track::new(GridVersion::V6_2);

        let mut line = StandardLine::new(Line::new(Point::zero(), Point::new(0.0, 1.0)));
        line.set_flipped(false);
        line.set_left_extension(false);
        line.set_right_extension(false);
        expected_track.standard_lines_mut().push(line);

        let mut line = StandardLine::new(Line::new(Point::zero(), Point::new(0.0, 1.0)));
        line.set_flipped(false);
        line.set_left_extension(false);
        line.set_right_extension(false);
        expected_track.standard_lines_mut().push(line);

        let mut line = StandardLine::new(Line::new(Point::zero(), Point::new(0.0, 1.0)));
        line.set_flipped(true);
        line.set_left_extension(true);
        line.set_right_extension(true);
        expected_track.standard_lines_mut().push(line);

        let mut line = StandardLine::new(Line::new(Point::zero(), Point::new(0.0, 1.0)));
        line.set_flipped(false);
        line.set_left_extension(false);
        line.set_right_extension(false);
        expected_track.standard_lines_mut().push(line);

        let mut line = StandardLine::new(Line::new(Point::zero(), Point::new(0.0, 1.0)));
        line.set_flipped(false);
        line.set_left_extension(true);
        line.set_right_extension(false);
        expected_track.standard_lines_mut().push(line);

        let mut line = StandardLine::new(Line::new(Point::zero(), Point::new(0.0, 1.0)));
        line.set_flipped(false);
        line.set_left_extension(false);
        line.set_right_extension(true);
        expected_track.standard_lines_mut().push(line);

        let mut line = StandardLine::new(Line::new(Point::zero(), Point::new(0.0, 1.0)));
        line.set_flipped(false);
        line.set_left_extension(true);
        line.set_right_extension(true);
        expected_track.standard_lines_mut().push(line);

        let mut line = StandardLine::new(Line::new(Point::zero(), Point::new(0.0, 1.0)));
        line.set_flipped(true);
        line.set_left_extension(true);
        line.set_right_extension(true);
        expected_track.standard_lines_mut().push(line);

        assert_eq!(result, expected_track);
    }
}
