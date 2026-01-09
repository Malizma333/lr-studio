use crate::TrkReadError;
use color::RGBColor;
use geometry::{Line, Point};
use lr_format_core::{
    GridVersion, RemountVersion, RiderBuilder, SceneryLineBuilder, StandardLineBuilder, Track,
    TrackBuilder,
    unit_conversion::{
        from_lra_audio_offset, from_lra_gravity, from_lra_scenery_width, from_lra_zoom,
    },
};
use quick_byte::QuickRead;
use std::{
    collections::HashSet,
    io::{Cursor, Read, Seek, SeekFrom},
};
use vector2d::Vector2Df;

// These string literals are implementation-specific, do not modify
const FEATURE_RED_MULTIPLIER: &str = "REDMULTIPLIER";
const FEATURE_SCENERY_WIDTH: &str = "SCENERYWIDTH";
const FEATURE_SONG_INFO: &str = "SONGINFO";
const FEATURE_IGNORABLE_TRIGGER: &str = "IGNORABLE_TRIGGER";
const FEATURE_6_1: &str = "6.1";
const FEATURE_ZERO_START: &str = "ZEROSTART";
const FEATURE_REMOUNT: &str = "REMOUNT";
const _FEATURE_FRICTIONLESS: &str = "FRICTIONLESS";
const FEATURE_START_ZOOM: &str = "STARTZOOM";
const FEATURE_X_GRAVITY: &str = "XGRAVITY";
const FEATURE_Y_GRAVITY: &str = "YGRAVITY";
const FEATURE_GRAVITY_WELL_SIZE: &str = "GRAVITYWELLSIZE";
const FEATURE_BACKGROUND_COLOR_R: &str = "BGCOLORR";
const FEATURE_BACKGROUND_COLOR_G: &str = "BGCOLORG";
const FEATURE_BACKGROUND_COLOR_B: &str = "BGCOLORB";
const FEATURE_LINE_COLOR_R: &str = "LINECOLORR";
const FEATURE_LINE_COLOR_G: &str = "LINECOLORG";
const FEATURE_LINE_COLOR_B: &str = "LINECOLORB";
const FEATURE_TRIGGERS: &str = "TRIGGERS";

#[derive(PartialEq)]
enum LineType {
    Standard,
    Acceleration,
    Scenery,
}

pub fn read(data: &Vec<u8>) -> Result<Track, TrkReadError> {
    let mut track = TrackBuilder::new(GridVersion::V6_2);
    let mut cursor = Cursor::new(data);

    // Magic number
    let mut magic_number = [0u8; 4];
    cursor.read_exact(&mut magic_number)?;

    if magic_number != [b'T', b'R', b'K', 0xF2] {
        Err(TrkReadError::InvalidMagicNumber(format!(
            "{:02X?}",
            &magic_number,
        )))?
    }

    // Version
    let version = cursor.read_u8()?;

    if version > 1 {
        Err(TrkReadError::UnsupportedTrackVersion(version.to_string()))?
    }

    let feature_string_length = cursor.read_u16_le()?;
    let mut buffer = vec![0; usize::from(feature_string_length)];
    cursor.read_exact(&mut buffer)?;
    let feature_string = str::from_utf8(&buffer)?;
    let mut included_features: HashSet<&str> = Default::default();

    for feature in feature_string.split(';').filter(|s| !s.is_empty()) {
        included_features.insert(feature);
    }

    let grid_version = if included_features.contains(FEATURE_6_1) {
        GridVersion::V6_1
    } else {
        GridVersion::V6_2
    };

    track.grid_version(grid_version);

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

        let mut buffer = vec![0; song_string_length];
        cursor.read_exact(&mut buffer)?;
        let song_string = str::from_utf8(&buffer)?;
        let song_data: Vec<&str> = song_string
            .split("\r\n")
            .filter(|s| !s.is_empty())
            .collect();

        if song_data.len() != 2 {
            Err(TrkReadError::InvalidSongFormat(song_data.join(",")))?
        }

        let name = song_data[0];
        let seconds_offset = song_data[1].parse::<f64>()?;
        track
            .audio_filename(name.to_string())
            .audio_offset_until_start(from_lra_audio_offset(seconds_offset));
    }

    let start_pos_x = cursor.read_f64_le()?;
    let start_pos_y = cursor.read_f64_le()?;
    let start_position = Vector2Df::new(start_pos_x, start_pos_y);

    let line_count = cursor.read_u32_le()?;

    let mut ordered_standard_lines = Vec::new();

    for _ in 0..line_count {
        let mut line_id: u32 = 0;
        let flags = cursor.read_u8()?;

        let line_type = match flags & 0x1F {
            1 => LineType::Standard,
            2 => LineType::Acceleration,
            0 => LineType::Scenery,
            other => Err(TrkReadError::UnsupportedLineType(other.to_string()))?,
        };

        let flipped = (flags >> 7) != 0;
        let line_ext = (flags >> 5) & 0x3;

        let mut multiplier = 1.0;
        let mut width = 1.0;

        if line_type == LineType::Acceleration && included_features.contains(FEATURE_RED_MULTIPLIER)
        {
            multiplier = f64::from(cursor.read_u8()?);
        }

        if line_type == LineType::Scenery {
            if included_features.contains(FEATURE_SCENERY_WIDTH) {
                width = from_lra_scenery_width(cursor.read_u8()?);
            }
        } else {
            if included_features.contains(FEATURE_IGNORABLE_TRIGGER) {
                let has_zoom_trigger = cursor.read_u8()?;
                if has_zoom_trigger == 1 {
                    let _target_zoom = from_lra_zoom(cursor.read_f32_le()?);
                    let _length = u32::try_from(cursor.read_i16_le()?)?;
                }
            }

            line_id = cursor.read_u32_le()?;

            if line_ext != 0 {
                _ = cursor.read_i32_le()?; // Prev line id or -1
                _ = cursor.read_i32_le()?; // Next line id or -1
            }
        }

        let line_x1 = cursor.read_f64_le()?;
        let line_y1 = cursor.read_f64_le()?;
        let line_x2 = cursor.read_f64_le()?;
        let line_y2 = cursor.read_f64_le()?;
        let endpoints = Line::new(Point::new(line_x1, line_y1), Point::new(line_x2, line_y2));
        let left_extension = line_ext & 0x1 != 0;
        let right_extension = line_ext & 0x2 != 0;

        match line_type {
            LineType::Standard => {
                let mut line = StandardLineBuilder::new(endpoints);
                line.flipped(flipped);
                line.left_extension(left_extension);
                line.right_extension(right_extension);
                ordered_standard_lines.push((line_id, line));
            }
            LineType::Acceleration => {
                let mut line = StandardLineBuilder::new(endpoints);
                line.flipped(flipped);
                line.left_extension(left_extension);
                line.right_extension(right_extension);
                line.multiplier(multiplier);
                ordered_standard_lines.push((line_id, line));
            }
            LineType::Scenery => {
                let mut line = SceneryLineBuilder::new(endpoints);
                line.width(width);
                track.scenery_lines().push(line);
            }
        }
    }

    ordered_standard_lines.sort_by_key(|t| t.0);

    for (_id, line) in ordered_standard_lines {
        track.standard_lines().push(line);
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

    let mut rider = RiderBuilder::new(remount_version);
    rider.start_offset(start_position);
    rider.start_velocity(start_velocity);
    track.riders().push(rider);

    let current = cursor.stream_position()?;
    let end = cursor.seek(SeekFrom::End(0))?;
    cursor.seek(SeekFrom::Start(current))?;

    if current == end {
        return Ok(track.build());
    }

    // Metadata section

    let mut meta_magic_number = [0u8; 4];
    cursor.read_exact(&mut meta_magic_number)?;

    if &meta_magic_number != b"META" {
        Err(TrkReadError::InvalidMagicNumber(format!(
            "{:02X?}",
            &meta_magic_number,
        )))?
    }

    let num_entries = cursor.read_u16_le()?;

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
        let meta_string_length = cursor.read_u16_le()?;
        let mut buffer = vec![0; usize::from(meta_string_length)];
        cursor.read_exact(&mut buffer)?;
        let meta_string = str::from_utf8(&buffer)?;
        let key_value_pair: Vec<&str> = meta_string.split("=").filter(|s| !s.is_empty()).collect();

        if key_value_pair.len() != 2 {
            Err(TrkReadError::InvalidKeyValue(key_value_pair.join(",")))?
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
                start_gravity_y = Some(f64::from(value.parse::<f32>()?));
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
                        Err(TrkReadError::EmptyTriggerData)?
                    }

                    match values[0] {
                        "0" => {
                            // Zoom
                            let _target_zoom = from_lra_zoom(values[1].parse::<f32>()?);
                            let _start_frame = u32::try_from(values[2].parse::<i32>()?)?;
                            let _end_frame = u32::try_from(values[3].parse::<i32>()?)?;
                        }
                        "1" => {
                            // Background Color
                            let _red = u8::try_from(values[1].parse::<i32>()?)?;
                            let _green = u8::try_from(values[2].parse::<i32>()?)?;
                            let _blue = u8::try_from(values[3].parse::<i32>()?)?;
                            let _start_frame = u32::try_from(values[4].parse::<i32>()?)?;
                            let _end_frame = u32::try_from(values[5].parse::<i32>()?)?;
                        }
                        "2" => {
                            // Line Color
                            let _red = u8::try_from(values[1].parse::<i32>()?)?;
                            let _green = u8::try_from(values[2].parse::<i32>()?)?;
                            let _blue = u8::try_from(values[3].parse::<i32>()?)?;
                            let _start_frame = u32::try_from(values[4].parse::<i32>()?)?;
                            let _end_frame = u32::try_from(values[5].parse::<i32>()?)?;
                        }
                        other => Err(TrkReadError::UnsupportedTriggerType(other.to_string()))?,
                    }
                }
            }
            _ => {}
        }
    }

    // Default values assigned because LRA:CE and LRO don't write on absent features (eg gravity Y gets written when gravity X may not be)

    let _start_zoom = start_zoom.unwrap_or(from_lra_zoom(4.0));

    let _start_gravity = from_lra_gravity(Vector2Df::new(
        start_gravity_x.unwrap_or(0.0),
        start_gravity_y.unwrap_or(1.0),
    ));

    let _start_background_color = RGBColor::new(
        start_bg_color_red.unwrap_or(244),
        start_bg_color_green.unwrap_or(245),
        start_bg_color_blue.unwrap_or(249),
    );

    let _start_line_color = RGBColor::new(
        start_line_color_red.unwrap_or(0),
        start_line_color_green.unwrap_or(0),
        start_line_color_blue.unwrap_or(0),
    );

    for line in track.standard_lines() {
        line.height(gravity_well_size.unwrap_or(10.0));
    }

    Ok(track.build())
}
