#[cfg(test)]
mod test {
    use std::fs;

    use color::RGBColor;
    use format_core::{
        track::{
            BackgroundColorEvent, CameraZoomEvent, FrameBoundsTrigger, LineColorEvent,
            LineHitTrigger, RemountVersion, TrackBuilder,
        },
        util::from_lra_zoom,
    };
    use pretty_assertions::assert_eq;
    use spatial_grid::GridVersion;
    use vector2d::Vector2Df;

    #[test]
    fn lra_features() {
        let file_name = "../fixtures/format_trk/lra_features.trk";
        let file = fs::read(file_name).expect("Failed to read TRK file");
        let result = format_trk::read(&file).expect("Failed to parse track file");
        let mut expected_builder = TrackBuilder::new(GridVersion::V6_2);
        expected_builder
            .metadata()
            .audio_filename("Buzz.ogg".to_string())
            .audio_offset(-1.5);
        expected_builder
            .rider_group()
            .add_rider(RemountVersion::None, 0)
            .start_angle(0.0)
            .start_position(Vector2Df::zero())
            .start_velocity(Vector2Df::new(0.4, 0.0));
        expected_builder
            .line_group()
            .add_standard_line(
                1,
                (
                    Vector2Df::new(-7.771946501344374, 11.286702185318813),
                    Vector2Df::new(9.192174944315191, 16.798703790690848),
                ),
            )
            .flipped(false)
            .left_extension(false)
            .right_extension(false);
        expected_builder
            .line_group()
            .add_acceleration_line(
                5,
                (
                    Vector2Df::new(28.870594229154598, 11.701041971830191),
                    Vector2Df::new(17.816032174771323, 11.701041971830193),
                ),
            )
            .flipped(true)
            .left_extension(true)
            .right_extension(true)
            .multiplier(3.0);
        expected_builder
            .line_group()
            .add_scenery_line((
                Vector2Df::new(16.742731128704364, 27.630652379947886),
                Vector2Df::new(34.02351098451611, 46.64916428829936),
            ))
            .width(3.0);
        expected_builder.legacy_camera_zoom_group().add_trigger(
            CameraZoomEvent::new(from_lra_zoom(2.0)),
            LineHitTrigger::new(0, 30),
        );
        let expected = expected_builder.build();
        assert_eq!(result, expected);
    }

    #[test]
    fn lrace_features() {
        let file_name = "../fixtures/format_trk/lrace_features.trk";
        let file = fs::read(file_name).expect("Failed to read TRK file");
        let result = format_trk::read(&file).expect("Failed to parse track file");
        let mut expected_builder = TrackBuilder::new(GridVersion::V6_1);
        expected_builder
            .metadata()
            .start_zoom(from_lra_zoom(2.0))
            .audio_filename("Really_Long_Song_Name_That_Takes_Up_More_Than_128_Characters_To_Test_7BitEncodedInt_Overflow_0123456789012345678901234567890123456789.ogg".to_string())
            .audio_offset(-1.5)
            .gravity_well_size(5.0)
            .start_background_color(RGBColor::new(1, 2, 3))
            .start_gravity(Vector2Df::new(1.0, 0.0))
            .start_line_color(RGBColor::new(4, 5, 6))
            .zero_friction_riders(true);
        expected_builder
            .rider_group()
            .add_rider(RemountVersion::LRA, 0)
            .start_angle(0.0)
            .start_position(Vector2Df::zero())
            .start_velocity(Vector2Df::new(0.0, 0.0));
        expected_builder
            .line_group()
            .add_standard_line(
                1,
                (
                    Vector2Df::new(-7.771946501344374, 11.286702185318813),
                    Vector2Df::new(9.192174944315191, 16.798703790690848),
                ),
            )
            .flipped(false)
            .left_extension(false)
            .right_extension(false);
        expected_builder
            .line_group()
            .add_acceleration_line(
                5,
                (
                    Vector2Df::new(28.870594229154598, 11.701041971830191),
                    Vector2Df::new(17.816032174771323, 11.701041971830193),
                ),
            )
            .flipped(true)
            .left_extension(true)
            .right_extension(true)
            .multiplier(3.0);
        expected_builder
            .line_group()
            .add_scenery_line((
                Vector2Df::new(16.742731128704364, 27.630652379947886),
                Vector2Df::new(34.02351098451611, 46.64916428829936),
            ))
            .width(3.0);
        expected_builder.camera_zoom_group().add_trigger(
            CameraZoomEvent::new(from_lra_zoom(1.0)),
            FrameBoundsTrigger::new(57, 97),
        );
        expected_builder.background_color_group().add_trigger(
            BackgroundColorEvent::new(RGBColor::new(23, 23, 23)),
            FrameBoundsTrigger::new(57, 87),
        );
        expected_builder.line_color_group().add_trigger(
            LineColorEvent::new(RGBColor::new(2, 2, 2)),
            FrameBoundsTrigger::new(57, 67),
        );
        let expected = expected_builder.build();
        assert_eq!(result, expected);
    }
}
