#[cfg(test)]
mod test {
    use color::RGBColor;
    use geometry::{Line, Point};
    use lr_format_core::{
        GridVersion, LayerBuilder, LayerFolderBuilder, RemountVersion, RiderBuilder,
        SceneryLineBuilder, StandardLineBuilder, TrackBuilder,
    };
    use pretty_assertions::assert_eq;
    use std::fs;
    use vector2d::Vector2Df;

    #[test]
    fn web_features() {
        let file_name = "../fixtures/lr_format_json/web_features.track.json";
        let file = fs::read(file_name).expect("Failed to read JSON file");
        let result = lr_format_json::read(&file).expect("Failed to parse track file");
        let mut expected = TrackBuilder::new(GridVersion::V6_2);

        expected.title(String::new());
        expected.artist(String::new());
        expected.description(String::new());
        expected.duration(1200);

        let mut rider = RiderBuilder::new(RemountVersion::None);
        rider.start_offset(Vector2Df::zero());
        rider.start_velocity(Vector2Df::new(0.4, 0.0));
        expected.riders().push(rider);

        let mut layer = LayerBuilder::new(0);
        layer.name("Base Layer".to_string());
        layer.color(RGBColor::new(255, 0, 0));
        layer.visible(true);
        layer.editable(true);
        expected.layers().push(layer);

        let mut layer = LayerFolderBuilder::new(1);
        layer.name("Folder".to_string());
        layer.visible(true);
        layer.editable(true);
        expected.layer_folders().push(layer);

        let mut layer = LayerBuilder::new(2);
        layer.name(String::new());
        layer.visible(true);
        layer.editable(true);
        layer.folder_id(1);
        expected.layers().push(layer);

        let mut line = StandardLineBuilder::new(Line::new(Point::zero(), Point::new(0.0, 1.0)));
        line.flipped(false);
        line.left_extension(false);
        line.right_extension(false);
        expected.standard_lines().push(line);

        let mut line = StandardLineBuilder::new(Line::new(Point::zero(), Point::new(0.0, 1.0)));
        line.flipped(false);
        line.left_extension(false);
        line.right_extension(false);
        line.multiplier(1.0);
        expected.standard_lines().push(line);

        let mut line = SceneryLineBuilder::new(Line::new(Point::zero(), Point::new(0.0, 1.0)));
        line.width(0.5);
        expected.scenery_lines().push(line);

        assert_eq!(result, expected.build());
    }

    #[test]
    fn lrl_features() {
        let file_name = "../fixtures/lr_format_json/lrl_features.track.json";
        let file = fs::read(file_name).expect("Failed to read JSON file");
        let result = lr_format_json::read(&file).expect("Failed to parse track file");
        let mut expected = TrackBuilder::new(GridVersion::V6_2);

        expected.title("test".to_string());

        let mut rider = RiderBuilder::new(RemountVersion::LRA);
        rider.start_offset(Vector2Df::zero());
        rider.start_velocity(Vector2Df::new(0.4, 0.0));
        expected.riders().push(rider);

        let mut line =
            StandardLineBuilder::new(Line::new(Point::new(-2.0, 5.0), Point::new(31.0, 9.0)));
        line.flipped(false);
        line.left_extension(false);
        line.right_extension(false);
        expected.standard_lines().push(line);

        let mut line =
            StandardLineBuilder::new(Line::new(Point::new(40.0, 26.0), Point::new(-1.0, 25.0)));
        line.flipped(true);
        line.left_extension(false);
        line.right_extension(false);
        line.multiplier(3.0);
        expected.standard_lines().push(line);

        let line =
            SceneryLineBuilder::new(Line::new(Point::new(36.0, 19.0), Point::new(88.0, 21.0)));
        expected.scenery_lines().push(line);

        assert_eq!(result, expected.build());
    }

    #[test]
    fn lro_features() {
        let file_name = "../fixtures/lr_format_json/lro_features.track.json";
        let file = fs::read(file_name).expect("Failed to read JSON file");
        let result = lr_format_json::read(&file).expect("Failed to parse track file");
        let mut expected = TrackBuilder::new(GridVersion::V6_2);

        expected.title("test".to_string());

        let mut rider = RiderBuilder::new(RemountVersion::LRA);
        rider.start_offset(Vector2Df::zero());
        rider.start_velocity(Vector2Df::zero());
        expected.riders().push(rider);

        let mut line =
            StandardLineBuilder::new(Line::new(Point::new(-36.0, -47.0), Point::new(-3.0, -48.0)));
        line.flipped(false);
        line.left_extension(true);
        line.right_extension(true);
        line.height(5.0);
        expected.standard_lines().push(line);

        let mut line =
            StandardLineBuilder::new(Line::new(Point::new(1.0, -43.0), Point::new(-35.0, -42.0)));
        line.flipped(true);
        line.left_extension(true);
        line.right_extension(true);
        line.multiplier(2.0);
        line.height(5.0);
        expected.standard_lines().push(line);

        let line =
            SceneryLineBuilder::new(Line::new(Point::new(-33.0, -33.0), Point::new(-3.0, -32.0)));
        expected.scenery_lines().push(line);

        assert_eq!(result, expected.build());
    }

    #[test]
    fn empty_track() {
        let file_name = "../fixtures/lr_format_json/empty_61.track.json";
        let file = fs::read(file_name).expect("Failed to read JSON file");
        let result = lr_format_json::read(&file).expect("Failed to parse track file");
        let mut expected = TrackBuilder::new(GridVersion::V6_1);

        let mut rider = RiderBuilder::new(RemountVersion::LRA);
        rider.start_offset(Vector2Df::new(0.0, 0.0));
        rider.start_velocity(Vector2Df::new(0.4, 0.0));
        expected.riders().push(rider);

        assert_eq!(result, expected.build());
    }

    #[test]
    fn remount_versions() {
        let file_name = "../fixtures/lr_format_json/remount_versions.track.json";
        let file = fs::read(file_name).expect("Failed to read JSON file");
        let result = lr_format_json::read(&file).expect("Failed to parse track file");
        let mut expected = TrackBuilder::new(GridVersion::V6_2);

        let mut rider = RiderBuilder::new(RemountVersion::None);
        rider.start_offset(Vector2Df::new(0.0, 0.0));
        rider.start_velocity(Vector2Df::new(0.0, 0.0));
        expected.riders().push(rider);

        let mut rider = RiderBuilder::new(RemountVersion::None);
        rider.start_offset(Vector2Df::new(0.0, 0.0));
        rider.start_velocity(Vector2Df::new(0.0, 0.0));
        expected.riders().push(rider);

        let mut rider = RiderBuilder::new(RemountVersion::ComV1);
        rider.start_offset(Vector2Df::new(0.0, 0.0));
        rider.start_velocity(Vector2Df::new(0.0, 0.0));
        expected.riders().push(rider);

        let mut rider = RiderBuilder::new(RemountVersion::None);
        rider.start_offset(Vector2Df::new(0.0, 0.0));
        rider.start_velocity(Vector2Df::new(0.0, 0.0));
        expected.riders().push(rider);

        let mut rider = RiderBuilder::new(RemountVersion::ComV2);
        rider.start_offset(Vector2Df::new(0.0, 0.0));
        rider.start_velocity(Vector2Df::new(0.0, 0.0));
        expected.riders().push(rider);

        assert_eq!(result, expected.build());
    }

    #[test]
    fn empty_layers() {
        let file_name = "../fixtures/lr_format_json/empty_layers.track.json";
        let file = fs::read(file_name).expect("Failed to read JSON file");
        let result = lr_format_json::read(&file).expect("Failed to parse track file");
        let mut expected = TrackBuilder::new(GridVersion::V6_2);

        let mut layer = LayerBuilder::new(0);
        layer.name("Base Layer".to_string());
        layer.visible(true);
        expected.layers().push(layer);

        let mut layer = LayerBuilder::new(1);
        layer.name("#invalid color".to_string());
        layer.visible(true);
        layer.editable(true);
        expected.layers().push(layer);

        assert_eq!(result, expected.build());
    }

    #[test]
    fn line_flags() {
        let file_name = "../fixtures/lr_format_json/line_flags.track.json";
        let file = fs::read(file_name).expect("Failed to read JSON file");
        let result = lr_format_json::read(&file).expect("Failed to parse track file");
        let mut expected = TrackBuilder::new(GridVersion::V6_2);

        let mut line = StandardLineBuilder::new(Line::new(Point::zero(), Point::new(0.0, 1.0)));
        line.flipped(false);
        line.left_extension(false);
        line.right_extension(false);
        expected.standard_lines().push(line);

        let mut line = StandardLineBuilder::new(Line::new(Point::zero(), Point::new(0.0, 1.0)));
        line.flipped(false);
        line.left_extension(false);
        line.right_extension(false);
        expected.standard_lines().push(line);

        let mut line = StandardLineBuilder::new(Line::new(Point::zero(), Point::new(0.0, 1.0)));
        line.flipped(true);
        line.left_extension(true);
        line.right_extension(true);
        expected.standard_lines().push(line);

        let mut line = StandardLineBuilder::new(Line::new(Point::zero(), Point::new(0.0, 1.0)));
        line.flipped(false);
        line.left_extension(false);
        line.right_extension(false);
        expected.standard_lines().push(line);

        let mut line = StandardLineBuilder::new(Line::new(Point::zero(), Point::new(0.0, 1.0)));
        line.flipped(false);
        line.left_extension(true);
        line.right_extension(false);
        expected.standard_lines().push(line);

        let mut line = StandardLineBuilder::new(Line::new(Point::zero(), Point::new(0.0, 1.0)));
        line.flipped(false);
        line.left_extension(false);
        line.right_extension(true);
        expected.standard_lines().push(line);

        let mut line = StandardLineBuilder::new(Line::new(Point::zero(), Point::new(0.0, 1.0)));
        line.flipped(false);
        line.left_extension(true);
        line.right_extension(true);
        expected.standard_lines().push(line);

        let mut line = StandardLineBuilder::new(Line::new(Point::zero(), Point::new(0.0, 1.0)));
        line.flipped(true);
        line.left_extension(true);
        line.right_extension(true);
        expected.standard_lines().push(line);

        assert_eq!(result, expected.build());
    }
}
