use crate::SolReadError;
use amf0::deserialize;
use geometry::{Line, Point};
use lr_format_core::{
    GridVersion, RemountVersion, RiderBuilder, SceneryLineBuilder, StandardLineBuilder, Track,
    TrackBuilder,
};
use quick_byte::QuickRead;
use std::io::{Cursor, Read, Seek};
use vector2d::Vector2Df;

/// Retrieve the number of tracks an sol file contains
pub fn get_track_count(data: &[u8]) -> u32 {
    let mut cursor = Cursor::new(data);

    // HACK: We assume header size is constant, and track list length will always be from 0x2C to 0x2F
    let _ = cursor.seek(std::io::SeekFrom::Start(0x2C));
    let num_tracks = cursor.read_u32_be().unwrap_or(0);

    num_tracks
}

pub fn read(data: &[u8], track_index: Option<u32>) -> Result<Track, SolReadError> {
    let mut track = TrackBuilder::new(GridVersion::V6_0);
    let data_size = u64::try_from(data.len())?;
    let mut bytes = Cursor::new(data);

    // Magic number
    let mut magic_number = [0u8; 2];
    bytes.read_exact(&mut magic_number)?;

    if magic_number != [0x00, 0xBF] {
        Err(SolReadError::InvalidMagicNumber(format!(
            "{:02X?}",
            &magic_number,
        )))?
    }

    // Header
    let _file_size = bytes.read_u32_be()? + 6;

    let mut tag = [0u8; 4];
    bytes.read_exact(&mut tag)?;

    if tag != [b'T', b'C', b'S', b'O'] {
        Err(SolReadError::InvalidMagicNumber(format!("{:02X?}", &tag)))?
    }

    let mut marker = [0u8; 6];
    bytes.read_exact(&mut marker)?;
    if marker != [0x00, 0x04, 0x00, 0x00, 0x00, 0x00] {
        Err(SolReadError::InvalidMagicNumber(format!(
            "{:02X?}",
            &marker,
        )))?
    }

    let sol_string_length = bytes.read_u16_be()?;
    let mut sol_name = vec![0; usize::from(sol_string_length)];
    bytes.read_exact(&mut sol_name)?;
    if str::from_utf8(&sol_name)? != "savedLines" {
        Err(SolReadError::InvalidMagicNumber(format!(
            "{:02X?}",
            &sol_name,
        )))?
    }

    let _padding = bytes.read_u32_be()?;

    let data_string_length = bytes.read_u16_be()?;
    let mut data_name = vec![0; usize::from(data_string_length)];
    bytes.read_exact(&mut data_name)?;
    if str::from_utf8(&data_name)? != "trackList" {
        Err(SolReadError::MissingTrackList)?
    }

    // Track Data
    let current_pos = bytes.position();
    // Slice from current position to last byte - 1 contains valid AMF0 format
    let mut trimmed_cursor = bytes.take(data_size.saturating_sub(1) - current_pos);
    let result = &deserialize(&mut trimmed_cursor)?;

    let track_list_amf = &result[0];
    let track_list =
        track_list_amf
            .clone()
            .get_object_properties()
            .ok_or(SolReadError::InvalidTrackList(format!(
                "{:?}",
                track_list_amf
            )))?;

    let target_track_index = match track_index {
        Some(index) => &index.to_string(),
        None => "0",
    };

    let target_track_amf =
        track_list
            .get(target_track_index)
            .ok_or(SolReadError::InvalidTrackIndex(format!(
                "{:?}",
                target_track_index
            )))?;

    let target_track =
        target_track_amf
            .clone()
            .get_object_properties()
            .ok_or(SolReadError::InvalidTrack(format!(
                "{:?}",
                target_track_amf
            )))?;

    if let Some(val) = target_track.get("label") {
        let title = val
            .clone()
            .get_string()
            .ok_or(SolReadError::InvalidLabel(format!("{:?}", val)))?;
        track.title(title);
    }

    if let Some(val) = target_track.get("version") {
        let version_string = val
            .clone()
            .get_string()
            .ok_or(SolReadError::InvalidGridVersion(format!("{:?}", val)))?;

        let grid_version = match version_string.as_str() {
            "6.0" => GridVersion::V6_0,
            "6.1" => GridVersion::V6_1,
            "6.2" => GridVersion::V6_2,
            other => Err(SolReadError::UnsupportedGridVersion(other.to_string()))?,
        };
        track.grid_version(grid_version);
    } else {
        track.grid_version(GridVersion::V6_0);
    }

    let start_position = if let Some(val) = target_track.get("startLine") {
        let start_position = val
            .clone()
            .get_object_properties()
            .ok_or(SolReadError::InvalidStartLine(format!("{:?}", val)))?;

        let start_x_amf = start_position
            .get("0")
            .ok_or(SolReadError::InvalidStartLine(format!(
                "{:?}",
                start_position
            )))?;
        let start_pos_x =
            start_x_amf
                .clone()
                .get_number()
                .ok_or(SolReadError::InvalidStartLine(format!(
                    "{:?}",
                    start_position
                )))?;

        let start_y_amf = start_position
            .get("1")
            .ok_or(SolReadError::InvalidStartLine(format!(
                "{:?}",
                start_position
            )))?;
        let start_pos_y =
            start_y_amf
                .clone()
                .get_number()
                .ok_or(SolReadError::InvalidStartLine(format!(
                    "{:?}",
                    start_position
                )))?;

        Vector2Df::new(start_pos_x, start_pos_y)
    } else {
        Vector2Df::zero()
    };

    let start_velocity = if target_track.contains_key("trackData") {
        Vector2Df::zero()
    } else {
        Vector2Df::new(0.4, 0.0)
    };

    let mut rider = RiderBuilder::new(RemountVersion::None);
    rider.start_offset(start_position);
    rider.start_velocity(start_velocity);
    track.riders().push(rider);

    if let Some(val) = target_track.get("data") {
        let lines_list = val
            .clone()
            .get_object_properties()
            .ok_or(SolReadError::InvalidLinesList(format!("{:?}", val)))?;

        let mut ordered_standard_lines = Vec::new();

        for line_amf in lines_list.values() {
            let line = line_amf
                .clone()
                .get_object_properties()
                .ok_or(SolReadError::InvalidLine(format!("{:?}", line_amf)))?;

            let x1_amf = line
                .get("0")
                .ok_or(SolReadError::InvalidLine(format!("{:?}", line_amf)))?;

            let x1 = x1_amf
                .clone()
                .get_number()
                .ok_or(SolReadError::InvalidLine(format!("{:?}", line_amf)))?;

            let y1_amf = line
                .get("1")
                .ok_or(SolReadError::InvalidLine(format!("{:?}", line_amf)))?;

            let y1 = y1_amf
                .clone()
                .get_number()
                .ok_or(SolReadError::InvalidLine(format!("{:?}", line_amf)))?;

            let x2_amf = line
                .get("2")
                .ok_or(SolReadError::InvalidLine(format!("{:?}", line_amf)))?;

            let x2 = x2_amf
                .clone()
                .get_number()
                .ok_or(SolReadError::InvalidLine(format!("{:?}", line_amf)))?;

            let y2_amf = line
                .get("3")
                .ok_or(SolReadError::InvalidLine(format!("{:?}", line_amf)))?;

            let y2 = y2_amf
                .clone()
                .get_number()
                .ok_or(SolReadError::InvalidLine(format!("{:?}", line_amf)))?;

            let ext_amf = line
                .get("4")
                .ok_or(SolReadError::InvalidLine(format!("{:?}", line_amf)))?;

            let ext = ext_amf.clone().get_number().unwrap_or(0.0);

            let left_extension = ext == 1.0 || ext == 3.0;
            let right_extension = ext == 2.0 || ext == 3.0;

            let flipped_amf = line
                .get("5")
                .ok_or(SolReadError::InvalidLine(format!("{:?}", line_amf)))?;

            let flipped = flipped_amf
                .clone()
                .get_boolean()
                .or_else(|| flipped_amf.clone().get_number().map(|num| num == 1.0))
                .ok_or(SolReadError::InvalidLine(format!("{:?}", line_amf)))?;

            let id_amf = line
                .get("8")
                .ok_or(SolReadError::InvalidLine(format!("{:?}", line_amf)))?;

            let id_float = id_amf
                .clone()
                .get_number()
                .ok_or(SolReadError::InvalidLine(format!("{:?}", line_amf)))?;

            let unsafe_id =
                if id_float.is_finite() && id_float >= 0.0 && id_float <= f64::from(u32::MAX) {
                    Some(id_float as u32)
                } else {
                    None
                };

            let id = match unsafe_id {
                Some(val) => val,
                None => Err(SolReadError::InvalidLine(format!("{:?}", line_amf)))?,
            };

            let line_type_amf = line
                .get("9")
                .ok_or(SolReadError::InvalidLine(format!("{:?}", line_amf)))?;

            let line_type_numeric = line_type_amf
                .clone()
                .get_number()
                .ok_or(SolReadError::InvalidLine(format!("{:?}", line_amf)))?;

            let is_standard_line = match line_type_numeric {
                0.0 | 1.0 => true,
                2.0 => false,
                other => Err(SolReadError::UnsupportedLineType(other.to_string()))?,
            };

            let multiplier = if line_type_numeric == 1.0 { 1.0 } else { 0.0 };

            let endpoints = Line::new(Point::new(x1, y1), Point::new(x2, y2));

            if is_standard_line {
                let mut standard_line = StandardLineBuilder::new(endpoints);
                standard_line.flipped(flipped);
                standard_line.left_extension(left_extension);
                standard_line.right_extension(right_extension);
                standard_line.multiplier(multiplier);
                ordered_standard_lines.push((id, standard_line));
            } else {
                let scenery_line = SceneryLineBuilder::new(endpoints);
                track.scenery_lines().push(scenery_line);
            }
        }

        ordered_standard_lines.sort_by_key(|t| t.0);

        for (_id, line) in ordered_standard_lines {
            track.standard_lines().push(line);
        }
    }

    Ok(track.build())
}
