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
    fn lra_features() {
        let file_name = "../fixtures/lr_format_trk/lra_features.trk";
        let file = fs::read(file_name).expect("Failed to read TRK file");
        let result = lr_format_trk::read(&file).expect("Failed to parse track file");
        let mut expected = TrackBuilder::new(GridVersion::V6_2);
        expected.audio_filename("Buzz.ogg".to_string());
        expected.audio_offset_until_start(-1.5);

        let mut rider = RiderBuilder::new(RemountVersion::None);
        rider.start_offset(Vector2Df::zero());
        rider.start_velocity(Vector2Df::new(0.4, 0.0));
        expected.riders().push(rider);

        let mut line = StandardLineBuilder::new(Line::new(
            Point::new(-7.771946501344374, 11.286702185318813),
            Point::new(9.192174944315191, 16.798703790690848),
        ));
        line.flipped(false);
        line.left_extension(false);
        line.right_extension(false);
        expected.standard_lines().push(line);

        let mut line = StandardLineBuilder::new(Line::new(
            Point::new(28.870594229154598, 11.701041971830191),
            Point::new(17.816032174771323, 11.701041971830193),
        ));
        line.flipped(true);
        line.left_extension(true);
        line.right_extension(true);
        line.multiplier(3.0);
        expected.standard_lines().push(line);

        let mut line = SceneryLineBuilder::new(Line::new(
            Point::new(16.742731128704364, 27.630652379947886),
            Point::new(34.02351098451611, 46.64916428829936),
        ));
        line.width(3.0);
        expected.scenery_lines().push(line);

        assert_eq!(result, expected.build());
    }

    #[test]
    fn lrace_features() {
        let file_name = "../fixtures/lr_format_trk/lrace_features.trk";
        let file = fs::read(file_name).expect("Failed to read TRK file");
        let result = lr_format_trk::read(&file).expect("Failed to parse track file");
        let mut expected = TrackBuilder::new(GridVersion::V6_1);
        expected.audio_filename("Really_Long_Song_Name_That_Takes_Up_More_Than_128_Characters_To_Test_7BitEncodedInt_Overflow_0123456789012345678901234567890123456789.ogg".to_string());
        expected.audio_offset_until_start(-1.5);

        let mut rider = RiderBuilder::new(RemountVersion::LRA);
        rider.start_offset(Vector2Df::zero());
        rider.start_velocity(Vector2Df::new(0.0, 0.0));
        expected.riders().push(rider);

        let mut line = StandardLineBuilder::new(Line::new(
            Point::new(-7.771946501344374, 11.286702185318813),
            Point::new(9.192174944315191, 16.798703790690848),
        ));
        line.flipped(false);
        line.left_extension(false);
        line.right_extension(false);
        line.height(5.0);
        expected.standard_lines().push(line);

        let mut line = StandardLineBuilder::new(Line::new(
            Point::new(28.870594229154598, 11.701041971830191),
            Point::new(17.816032174771323, 11.701041971830193),
        ));
        line.flipped(true);
        line.left_extension(true);
        line.right_extension(true);
        line.multiplier(3.0);
        line.height(5.0);
        expected.standard_lines().push(line);

        let mut line = SceneryLineBuilder::new(Line::new(
            Point::new(16.742731128704364, 27.630652379947886),
            Point::new(34.02351098451611, 46.64916428829936),
        ));
        line.width(3.0);
        expected.scenery_lines().push(line);

        assert_eq!(result, expected.build());
    }
}
