#[cfg(test)]
mod test {
    use geometry::{Line, Point};
    use lr_format_core::{GridVersion, RemountVersion, Rider, SceneryLine, StandardLine, Track};
    use pretty_assertions::assert_eq;
    use std::fs;
    use vector2d::Vector2Df;

    #[test]
    fn lra_features() {
        let file_name = "../fixtures/lr_format_trk/lra_features.trk";
        let file = fs::read(file_name).expect("Failed to read TRK file");
        let result = lr_format_trk::read(&file).expect("Failed to parse track file");
        let mut expected_track = Track::new(GridVersion::V6_2);
        expected_track.set_audio_filename("Buzz.ogg".to_string());
        expected_track.set_audio_offset_until_start(-1.5);

        let mut rider = Rider::new(RemountVersion::None);
        rider.set_start_offset(Vector2Df::zero());
        rider.set_start_velocity(Vector2Df::new(0.4, 0.0));
        expected_track.riders_mut().push(rider);

        let mut line = StandardLine::new(Line::new(
            Point::new(-7.771946501344374, 11.286702185318813),
            Point::new(9.192174944315191, 16.798703790690848),
        ));
        line.set_flipped(false);
        line.set_left_extension(false);
        line.set_right_extension(false);
        expected_track.standard_lines_mut().push(line);

        let mut line = StandardLine::new(Line::new(
            Point::new(28.870594229154598, 11.701041971830191),
            Point::new(17.816032174771323, 11.701041971830193),
        ));
        line.set_flipped(true);
        line.set_left_extension(true);
        line.set_right_extension(true);
        line.set_multiplier(3.0);
        expected_track.standard_lines_mut().push(line);

        let mut line = SceneryLine::new(Line::new(
            Point::new(16.742731128704364, 27.630652379947886),
            Point::new(34.02351098451611, 46.64916428829936),
        ));
        line.set_width(3.0);
        expected_track.scenery_lines_mut().push(line);

        // expected_track.legacy_camera_zoom_group().add_trigger(
        //     CameraZoomEvent::new(from_lra_zoom(2.0)),
        //     LineHitTrigger::new(0, 30),
        // );
        assert_eq!(result, expected_track);
    }

    #[test]
    fn lrace_features() {
        let file_name = "../fixtures/lr_format_trk/lrace_features.trk";
        let file = fs::read(file_name).expect("Failed to read TRK file");
        let result = lr_format_trk::read(&file).expect("Failed to parse track file");
        let mut expected_track = Track::new(GridVersion::V6_1);
        expected_track.set_audio_filename("Really_Long_Song_Name_That_Takes_Up_More_Than_128_Characters_To_Test_7BitEncodedInt_Overflow_0123456789012345678901234567890123456789.ogg".to_string());
        expected_track.set_audio_offset_until_start(-1.5);
        // .start_zoom(from_lra_zoom(2.0))
        // .start_background_color(RGBColor::new(1, 2, 3))
        // .start_gravity(from_lra_gravity(Vector2Df::new(1.0, 0.0)))
        // .start_line_color(RGBColor::new(4, 5, 6));

        let mut rider = Rider::new(RemountVersion::LRA);
        rider.set_start_offset(Vector2Df::zero());
        rider.set_start_velocity(Vector2Df::new(0.0, 0.0));
        expected_track.riders_mut().push(rider);

        let mut line = StandardLine::new(Line::new(
            Point::new(-7.771946501344374, 11.286702185318813),
            Point::new(9.192174944315191, 16.798703790690848),
        ));
        line.set_flipped(false);
        line.set_left_extension(false);
        line.set_right_extension(false);
        line.set_height(5.0);
        expected_track.standard_lines_mut().push(line);

        let mut line = StandardLine::new(Line::new(
            Point::new(28.870594229154598, 11.701041971830191),
            Point::new(17.816032174771323, 11.701041971830193),
        ));
        line.set_flipped(true);
        line.set_left_extension(true);
        line.set_right_extension(true);
        line.set_multiplier(3.0);
        line.set_height(5.0);
        expected_track.standard_lines_mut().push(line);

        let mut line = SceneryLine::new(Line::new(
            Point::new(16.742731128704364, 27.630652379947886),
            Point::new(34.02351098451611, 46.64916428829936),
        ));
        line.set_width(3.0);
        expected_track.scenery_lines_mut().push(line);
        // expected_track.camera_zoom_group().add_trigger(
        //     CameraZoomEvent::new(from_lra_zoom(1.0)),
        //     FrameBoundsTrigger::new(57, 97),
        // );
        // expected_track.background_color_group().add_trigger(
        //     BackgroundColorEvent::new(RGBColor::new(23, 23, 23)),
        //     FrameBoundsTrigger::new(57, 87),
        // );
        // expected_track.line_color_group().add_trigger(
        //     LineColorEvent::new(RGBColor::new(2, 2, 2)),
        //     FrameBoundsTrigger::new(57, 67),
        // );
        assert_eq!(result, expected_track);
    }
}
