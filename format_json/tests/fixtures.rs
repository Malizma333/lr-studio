#[cfg(test)]
mod test {
    use color::RGBColor;
    use pretty_assertions::assert_eq;
    use std::fs;

    use format_core::{
        track::{
            BackgroundColorEvent, CameraZoomEvent, FrameBoundsTrigger, GridVersion, LineColorEvent,
            LineHitTrigger, RemountVersion, TrackBuilder,
        },
        unit_conversion::from_lra_zoom,
    };
    use vector2d::Vector2Df;

    #[test]
    fn web_features() {
        let file_name = "../fixtures/format_json/web_features.track.json";
        let file = fs::read(file_name).expect("Failed to read JSON file");
        let result = format_json::read(&file).expect("Failed to parse track file");
        let mut expected_builder = TrackBuilder::new(GridVersion::V6_2);
        expected_builder
            .metadata()
            .title(String::new())
            .artist(String::new())
            .description(String::new())
            .duration(1200);
        expected_builder
            .rider_group()
            .add_rider(RemountVersion::None, 0)
            .start_position(Vector2Df::zero())
            .start_velocity(Vector2Df::new(0.4, 0.0))
            .start_angle(0.0);
        expected_builder
            .layer_group()
            .add_layer(0, 0)
            .name("Base Layer".to_string())
            .color(RGBColor::new(255, 0, 0))
            .visible(true)
            .editable(true);
        expected_builder
            .layer_group()
            .add_layer_folder(1, 1, 0)
            .name("Folder".to_string())
            .visible(true)
            .editable(true);
        expected_builder
            .layer_group()
            .add_layer(2, 2)
            .name(String::new())
            .visible(true)
            .editable(true)
            .folder_id(1);
        expected_builder
            .line_group()
            .add_standard_line(1, (Vector2Df::zero(), Vector2Df::up()))
            .flipped(false)
            .left_extension(false)
            .right_extension(false);
        expected_builder
            .line_group()
            .add_acceleration_line(2, (Vector2Df::zero(), Vector2Df::up()))
            .flipped(false)
            .left_extension(false)
            .right_extension(false)
            .multiplier(1.0);
        expected_builder
            .line_group()
            .add_scenery_line((Vector2Df::zero(), Vector2Df::up()))
            .width(0.5);
        let expected = expected_builder.build();
        assert_eq!(result, expected);
    }

    #[test]
    fn lrl_features() {
        let file_name = "../fixtures/format_json/lrl_features.track.json";
        let file = fs::read(file_name).expect("Failed to read JSON file");
        let result = format_json::read(&file).expect("Failed to parse track file");
        let mut expected_builder = TrackBuilder::new(GridVersion::V6_2);
        expected_builder
            .metadata()
            .title("test".to_string())
            .start_zoom(from_lra_zoom(4.0));
        expected_builder
            .rider_group()
            .add_rider(RemountVersion::LRA, 0)
            .start_position(Vector2Df::zero())
            .start_velocity(Vector2Df::new(0.4, 0.0))
            .start_angle(0.0);
        expected_builder
            .line_group()
            .add_standard_line(1, (Vector2Df::new(-2.0, 5.0), Vector2Df::new(31.0, 9.0)))
            .flipped(false)
            .left_extension(false)
            .right_extension(false);
        expected_builder
            .line_group()
            .add_acceleration_line(2, (Vector2Df::new(40.0, 26.0), Vector2Df::new(-1.0, 25.0)))
            .flipped(true)
            .left_extension(false)
            .right_extension(false)
            .multiplier(3.0);
        expected_builder
            .line_group()
            .add_scenery_line((Vector2Df::new(36.0, 19.0), Vector2Df::new(88.0, 21.0)));
        expected_builder.legacy_camera_zoom_group().add_trigger(
            CameraZoomEvent::new(from_lra_zoom(4.0)),
            LineHitTrigger::new(1, 20),
        );
        let expected = expected_builder.build();
        assert_eq!(result, expected);
    }

    #[test]
    fn lro_features() {
        let file_name = "../fixtures/format_json/lro_features.track.json";
        let file = fs::read(file_name).expect("Failed to read JSON file");
        let result = format_json::read(&file).expect("Failed to parse track file");
        let mut expected_builder = TrackBuilder::new(GridVersion::V6_2);
        expected_builder
            .metadata()
            .title("test".to_string())
            .start_zoom(from_lra_zoom(4.0))
            .start_background_color(RGBColor::new(255, 255, 255))
            .start_line_color(RGBColor::new(0, 0, 0))
            .start_gravity(Vector2Df::up())
            .gravity_well_size(10.0);
        expected_builder
            .rider_group()
            .add_rider(RemountVersion::LRA, 0)
            .start_position(Vector2Df::zero())
            .start_velocity(Vector2Df::new(0.0, 0.0))
            .start_angle(0.0);
        expected_builder
            .line_group()
            .add_standard_line(
                0,
                (Vector2Df::new(-36.0, -47.0), Vector2Df::new(-3.0, -48.0)),
            )
            .flipped(false)
            .left_extension(true)
            .right_extension(true);
        expected_builder
            .line_group()
            .add_acceleration_line(
                1,
                (Vector2Df::new(1.0, -43.0), Vector2Df::new(-35.0, -42.0)),
            )
            .flipped(true)
            .left_extension(true)
            .right_extension(true)
            .multiplier(2.0);
        expected_builder
            .line_group()
            .add_scenery_line((Vector2Df::new(-33.0, -33.0), Vector2Df::new(-3.0, -32.0)));
        expected_builder.camera_zoom_group().add_trigger(
            CameraZoomEvent::new(from_lra_zoom(4.0)),
            FrameBoundsTrigger::new(0, 40),
        );
        expected_builder.background_color_group().add_trigger(
            BackgroundColorEvent::new(RGBColor::new(255, 255, 255)),
            FrameBoundsTrigger::new(1, 41),
        );
        expected_builder.line_color_group().add_trigger(
            LineColorEvent::new(RGBColor::new(0, 0, 0)),
            FrameBoundsTrigger::new(1, 41),
        );
        let expected = expected_builder.build();
        assert_eq!(result, expected);
    }

    #[test]
    fn empty_track() {
        let file_name = "../fixtures/format_json/empty_61.track.json";
        let file = fs::read(file_name).expect("Failed to read JSON file");
        let result = format_json::read(&file).expect("Failed to parse track file");
        let mut expected_builder = TrackBuilder::new(GridVersion::V6_1);
        expected_builder
            .rider_group()
            .add_rider(RemountVersion::LRA, 0)
            .start_position(Vector2Df::new(0.0, 0.0))
            .start_velocity(Vector2Df::new(0.4, 0.0))
            .start_angle(0.0);
        let expected = expected_builder.build();
        assert_eq!(result, expected);
    }

    #[test]
    fn remount_versions() {
        let file_name = "../fixtures/format_json/remount_versions.track.json";
        let file = fs::read(file_name).expect("Failed to read JSON file");
        let result = format_json::read(&file).expect("Failed to parse track file");
        let mut expected_builder = TrackBuilder::new(GridVersion::V6_2);
        expected_builder
            .rider_group()
            .add_rider(RemountVersion::None, 0)
            .start_position(Vector2Df::new(0.0, 0.0))
            .start_velocity(Vector2Df::new(0.0, 0.0));
        expected_builder
            .rider_group()
            .add_rider(RemountVersion::None, 1)
            .start_position(Vector2Df::new(0.0, 0.0))
            .start_velocity(Vector2Df::new(0.0, 0.0));
        expected_builder
            .rider_group()
            .add_rider(RemountVersion::ComV1, 2)
            .start_position(Vector2Df::new(0.0, 0.0))
            .start_velocity(Vector2Df::new(0.0, 0.0));
        expected_builder
            .rider_group()
            .add_rider(RemountVersion::None, 3)
            .start_position(Vector2Df::new(0.0, 0.0))
            .start_velocity(Vector2Df::new(0.0, 0.0));
        expected_builder
            .rider_group()
            .add_rider(RemountVersion::ComV2, 4)
            .start_position(Vector2Df::new(0.0, 0.0))
            .start_velocity(Vector2Df::new(0.0, 0.0));
        let expected = expected_builder.build();
        assert_eq!(result, expected);
    }

    #[test]
    fn empty_layers() {
        let file_name = "../fixtures/format_json/empty_layers.track.json";
        let file = fs::read(file_name).expect("Failed to read JSON file");
        let result = format_json::read(&file).expect("Failed to parse track file");
        let mut expected_builder = TrackBuilder::new(GridVersion::V6_2);
        expected_builder
            .layer_group()
            .add_layer(0, 0)
            .name("Base Layer".to_string())
            .visible(true);
        expected_builder
            .layer_group()
            .add_layer(1, 1)
            .name("#invalid color".to_string())
            .visible(true)
            .editable(true);
        let expected = expected_builder.build();
        assert_eq!(result, expected);
    }

    #[test]
    fn line_flags() {
        let file_name = "../fixtures/format_json/line_flags.track.json";
        let file = fs::read(file_name).expect("Failed to read JSON file");
        let result = format_json::read(&file).expect("Failed to parse track file");
        let mut expected_builder = TrackBuilder::new(GridVersion::V6_2);
        expected_builder
            .line_group()
            .add_standard_line(0, (Vector2Df::zero(), Vector2Df::up()))
            .flipped(false)
            .left_extension(false)
            .right_extension(false);
        expected_builder
            .line_group()
            .add_standard_line(7, (Vector2Df::zero(), Vector2Df::up()))
            .flipped(true)
            .left_extension(true)
            .right_extension(true);
        expected_builder
            .line_group()
            .add_standard_line(1, (Vector2Df::zero(), Vector2Df::up()))
            .flipped(false)
            .left_extension(false)
            .right_extension(false);
        expected_builder
            .line_group()
            .add_standard_line(2, (Vector2Df::zero(), Vector2Df::up()))
            .flipped(true)
            .left_extension(true)
            .right_extension(true);
        expected_builder
            .line_group()
            .add_standard_line(3, (Vector2Df::zero(), Vector2Df::up()))
            .flipped(false)
            .left_extension(false)
            .right_extension(false);
        expected_builder
            .line_group()
            .add_standard_line(4, (Vector2Df::zero(), Vector2Df::up()))
            .flipped(false)
            .left_extension(true)
            .right_extension(false);
        expected_builder
            .line_group()
            .add_standard_line(5, (Vector2Df::zero(), Vector2Df::up()))
            .flipped(false)
            .left_extension(false)
            .right_extension(true);
        expected_builder
            .line_group()
            .add_standard_line(6, (Vector2Df::zero(), Vector2Df::up()))
            .flipped(false)
            .left_extension(true)
            .right_extension(true);
        let expected = expected_builder.build();
        assert_eq!(result, expected);
    }
}
