use std::{
    collections::HashSet,
    io::{Cursor, Read, Seek, SeekFrom},
};

use crate::{
    FEATURE_6_1, FEATURE_BACKGROUND_COLOR_B, FEATURE_BACKGROUND_COLOR_G,
    FEATURE_BACKGROUND_COLOR_R, FEATURE_FRICTIONLESS, FEATURE_GRAVITY_WELL_SIZE,
    FEATURE_IGNORABLE_TRIGGER, FEATURE_LINE_COLOR_B, FEATURE_LINE_COLOR_G, FEATURE_LINE_COLOR_R,
    FEATURE_RED_MULTIPLIER, FEATURE_REMOUNT, FEATURE_SCENERY_WIDTH, FEATURE_SONG_INFO,
    FEATURE_START_ZOOM, FEATURE_TRIGGERS, FEATURE_X_GRAVITY, FEATURE_Y_GRAVITY, FEATURE_ZERO_START,
    TrkReadError,
};

use byteorder::{LittleEndian, ReadBytesExt};
use color::RGBColor;
use format_core::{
    track::{
        BackgroundColorEvent, CameraZoomEvent, FrameBoundsTrigger, LineColorEvent, LineHitTrigger,
        LineType, RemountVersion, Track, TrackBuilder,
    },
    util::{
        bytes_to_hex_string, from_lra_scenery_width, from_lra_zoom,
        string_parser::{Endianness, StringLength, parse_string},
    },
};
use spatial_grid::GridVersion;
use vector2d::Vector2Df;

pub fn read(data: &Vec<u8>) -> Result<Track, TrkReadError> {
    let track_builder = &mut TrackBuilder::new(GridVersion::V6_2);
    let mut cursor = Cursor::new(data);

    // Magic number
    let mut magic_number = [0u8; 4];
    cursor.read_exact(&mut magic_number)?;

    if magic_number != [b'T', b'R', b'K', 0xF2] {
        return Err(TrkReadError::InvalidMagicNumber(bytes_to_hex_string(
            &magic_number,
        )));
    }

    // Version
    let version = cursor.read_u8()?;

    if version > 1 {
        return Err(TrkReadError::UnsupportedTrackVersion(version.to_string()));
    }

    let feature_string = parse_string(&mut cursor, StringLength::U16, Endianness::Little)?;
    let mut included_features: HashSet<&str> = Default::default();

    for feature in feature_string.split(';').filter(|s| !s.is_empty()) {
        included_features.insert(feature);
    }

    let grid_version = if included_features.contains(FEATURE_6_1) {
        GridVersion::V6_1
    } else {
        GridVersion::V6_2
    };
    track_builder.metadata().grid_version(grid_version);

    if included_features.contains(FEATURE_SONG_INFO) {
        let mut song_string_length = 0;
        let mut bit_shift = 0;

        loop {
            // Read 7BitEncodedInt song string length
            let byte = cursor.read_u8()?;
            song_string_length |= usize::from(byte & 0x7F) << bit_shift;

            if byte & 0x80 == 0 {
                break;
            }

            bit_shift += 7;
        }

        let song_string = parse_string(
            &mut cursor,
            StringLength::Fixed(song_string_length),
            Endianness::Little,
        )?;
        let song_data: Vec<&str> = song_string
            .split("\r\n")
            .filter(|s| !s.is_empty())
            .collect();

        if song_data.len() != 2 {
            return Err(TrkReadError::InvalidSongFormat(song_data.join(",")));
        }

        let name = song_data[0];
        let seconds_offset = song_data[1].parse::<f64>()?;
        track_builder
            .metadata()
            .audio_filename(name.to_string())
            .audio_offset(-seconds_offset);
    }

    let start_pos_x = cursor.read_f64::<LittleEndian>()?;
    let start_pos_y = cursor.read_f64::<LittleEndian>()?;
    let start_position = Vector2Df::new(start_pos_x, start_pos_y);

    let line_count = cursor.read_u32::<LittleEndian>()?;

    let mut max_id = 0;

    for _ in 0..line_count {
        let mut line_id: u32 = 0;
        let flags = cursor.read_u8()?;

        let line_type = match flags & 0x1F {
            1 => LineType::Standard,
            2 => LineType::Acceleration,
            0 => LineType::Scenery,
            other => {
                return Err(TrkReadError::UnsupportedLineType(other.to_string()));
            }
        };

        let flipped = (flags >> 7) != 0;
        let line_ext = (flags >> 5) & 0x3;

        let mut line_multiplier = 1.0;
        let mut line_scenery_width = 1.0;

        if line_type == LineType::Acceleration && included_features.contains(FEATURE_RED_MULTIPLIER)
        {
            line_multiplier = f64::from(cursor.read_u8()?);
        }

        if line_type == LineType::Scenery {
            if included_features.contains(FEATURE_SCENERY_WIDTH) {
                line_scenery_width = from_lra_scenery_width(cursor.read_u8()?);
            }
        } else {
            if included_features.contains(FEATURE_IGNORABLE_TRIGGER) {
                let has_zoom_trigger = cursor.read_u8()?;
                if has_zoom_trigger == 1 {
                    let target_zoom = from_lra_zoom(cursor.read_f32::<LittleEndian>()?);
                    let length = u32::try_from(cursor.read_i16::<LittleEndian>()?)?;
                    let zoom_event = CameraZoomEvent::new(target_zoom);
                    let line_hit = LineHitTrigger::new(line_id, length);
                    track_builder
                        .legacy_camera_zoom_group()
                        .add_trigger(zoom_event, line_hit);
                }
            }

            line_id = cursor.read_u32::<LittleEndian>()?;
            max_id = max_id.max(line_id);

            if line_ext != 0 {
                _ = cursor.read_i32::<LittleEndian>()?; // Prev line id or -1
                _ = cursor.read_i32::<LittleEndian>()?; // Next line id or -1
            }
        }

        let line_x1 = cursor.read_f64::<LittleEndian>()?;
        let line_y1 = cursor.read_f64::<LittleEndian>()?;
        let line_x2 = cursor.read_f64::<LittleEndian>()?;
        let line_y2 = cursor.read_f64::<LittleEndian>()?;
        let endpoints = (
            Vector2Df::new(line_x1, line_y1),
            Vector2Df::new(line_x2, line_y2),
        );
        let left_extension = line_ext & 0x1 != 0;
        let right_extension = line_ext & 0x2 != 0;

        match line_type {
            LineType::Standard => {
                track_builder
                    .line_group()
                    .add_standard_line(line_id, endpoints)
                    .flipped(flipped)
                    .left_extension(left_extension)
                    .right_extension(right_extension);
            }
            LineType::Acceleration => {
                track_builder
                    .line_group()
                    .add_acceleration_line(line_id, endpoints)
                    .flipped(flipped)
                    .left_extension(left_extension)
                    .right_extension(right_extension)
                    .multiplier(line_multiplier);
            }
            LineType::Scenery => {
                track_builder
                    .line_group()
                    .add_scenery_line(endpoints)
                    .width(line_scenery_width);
            }
        }
    }

    if included_features.contains(FEATURE_FRICTIONLESS) {
        track_builder.metadata().zero_friction_riders(true);
    }

    let remount_version = if included_features.contains(FEATURE_REMOUNT) {
        RemountVersion::LRA
    } else {
        RemountVersion::None
    };

    let start_velocity = if included_features.contains(FEATURE_ZERO_START) {
        Vector2Df::zero()
    } else {
        Vector2Df::new(0.4, 0.0)
    };

    track_builder
        .rider_group()
        .add_rider(remount_version, 0)
        .start_angle(0.0)
        .start_position(start_position)
        .start_velocity(start_velocity);

    let current = cursor.stream_position()?;
    let end = cursor.seek(SeekFrom::End(0))?;
    cursor.seek(SeekFrom::Start(current))?;

    if current == end {
        return Ok(track_builder.build());
    }

    // Metadata section

    let mut meta_magic_number = [0u8; 4];
    cursor.read_exact(&mut meta_magic_number)?;

    if &meta_magic_number != b"META" {
        return Err(TrkReadError::InvalidMagicNumber(bytes_to_hex_string(
            &meta_magic_number,
        )));
    }

    let num_entries = cursor.read_u16::<LittleEndian>()?;

    let mut start_zoom = None;
    let mut start_gravity_x = None;
    let mut start_gravity_y = None;
    let mut gravity_well_size = None;
    let mut start_line_color_red = None;
    let mut start_line_color_green = None;
    let mut start_line_color_blue = None;
    let mut start_bg_color_red = None;
    let mut start_bg_color_green = None;
    let mut start_bg_color_blue = None;

    for _ in 0..num_entries {
        let meta_string = parse_string(&mut cursor, StringLength::U16, Endianness::Little)?;
        let key_value_pair: Vec<&str> = meta_string.split("=").filter(|s| !s.is_empty()).collect();

        if key_value_pair.len() != 2 {
            return Err(TrkReadError::InvalidKeyValue(key_value_pair.join(",")));
        }

        let key = key_value_pair[0];
        let value = key_value_pair[1];

        match key {
            FEATURE_START_ZOOM => {
                start_zoom = Some(from_lra_zoom(value.parse::<f32>()?));
            }
            FEATURE_X_GRAVITY => {
                start_gravity_x = Some(f64::from(value.parse::<f32>()?));
            }
            FEATURE_Y_GRAVITY => {
                start_gravity_y = Some(-f64::from(value.parse::<f32>()?));
            }
            FEATURE_GRAVITY_WELL_SIZE => {
                gravity_well_size = Some(value.parse::<f64>()?);
            }
            FEATURE_BACKGROUND_COLOR_R => {
                start_bg_color_red = Some(u8::try_from(value.parse::<i32>()?)?);
            }
            FEATURE_BACKGROUND_COLOR_G => {
                start_bg_color_green = Some(u8::try_from(value.parse::<i32>()?)?);
            }
            FEATURE_BACKGROUND_COLOR_B => {
                start_bg_color_blue = Some(u8::try_from(value.parse::<i32>()?)?);
            }
            FEATURE_LINE_COLOR_R => {
                start_line_color_red = Some(u8::try_from(value.parse::<i32>()?)?);
            }
            FEATURE_LINE_COLOR_G => {
                start_line_color_green = Some(u8::try_from(value.parse::<i32>()?)?);
            }
            FEATURE_LINE_COLOR_B => {
                start_line_color_blue = Some(u8::try_from(value.parse::<i32>()?)?);
            }
            FEATURE_TRIGGERS => {
                for trigger in value.split('&').filter(|s| !s.is_empty()) {
                    let values: Vec<&str> = trigger.split(':').filter(|s| !s.is_empty()).collect();

                    if values.is_empty() {
                        return Err(TrkReadError::EmptyTriggerData);
                    }

                    match values[0] {
                        "0" => {
                            // Zoom
                            let target_zoom = from_lra_zoom(values[1].parse::<f32>()?);
                            let start_frame = u32::try_from(values[2].parse::<i32>()?)?;
                            let end_frame = u32::try_from(values[3].parse::<i32>()?)?;
                            let zoom_event = CameraZoomEvent::new(target_zoom);
                            let frame_bounds = FrameBoundsTrigger::new(start_frame, end_frame);
                            track_builder
                                .camera_zoom_group()
                                .add_trigger(zoom_event, frame_bounds);
                        }
                        "1" => {
                            // Background Color
                            let red = u8::try_from(values[1].parse::<i32>()?)?;
                            let green = u8::try_from(values[2].parse::<i32>()?)?;
                            let blue = u8::try_from(values[3].parse::<i32>()?)?;
                            let start_frame = u32::try_from(values[4].parse::<i32>()?)?;
                            let end_frame = u32::try_from(values[5].parse::<i32>()?)?;
                            let bg_color_event =
                                BackgroundColorEvent::new(RGBColor::new(red, green, blue));
                            let frame_bounds = FrameBoundsTrigger::new(start_frame, end_frame);
                            track_builder
                                .background_color_group()
                                .add_trigger(bg_color_event, frame_bounds);
                        }
                        "2" => {
                            // Line Color
                            let red = u8::try_from(values[1].parse::<i32>()?)?;
                            let green = u8::try_from(values[2].parse::<i32>()?)?;
                            let blue = u8::try_from(values[3].parse::<i32>()?)?;
                            let start_frame = u32::try_from(values[4].parse::<i32>()?)?;
                            let end_frame = u32::try_from(values[5].parse::<i32>()?)?;
                            let line_color_event =
                                LineColorEvent::new(RGBColor::new(red, green, blue));
                            let frame_bounds = FrameBoundsTrigger::new(start_frame, end_frame);
                            track_builder
                                .line_color_group()
                                .add_trigger(line_color_event, frame_bounds);
                        }
                        other => {
                            return Err(TrkReadError::UnsupportedTriggerType(other.to_string()));
                        }
                    }
                }
            }
            _ => {}
        }
    }

    // Default values assigned because LRA:CE and LRO don't write on absent features (eg gravity Y gets written when gravity X may not be)

    track_builder
        .metadata()
        .start_zoom(start_zoom.unwrap_or(from_lra_zoom(4.0)));

    track_builder.metadata().start_gravity(Vector2Df::new(
        start_gravity_x.unwrap_or(0.0),
        start_gravity_y.unwrap_or(-1.0),
    ));

    track_builder
        .metadata()
        .gravity_well_size(gravity_well_size.unwrap_or(10.0));

    track_builder
        .metadata()
        .start_background_color(RGBColor::new(
            start_bg_color_red.unwrap_or(244),
            start_bg_color_green.unwrap_or(245),
            start_bg_color_blue.unwrap_or(249),
        ));

    track_builder.metadata().start_line_color(RGBColor::new(
        start_line_color_red.unwrap_or(0),
        start_line_color_green.unwrap_or(0),
        start_line_color_blue.unwrap_or(0),
    ));

    Ok(track_builder.build())
}
