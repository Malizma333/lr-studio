use crate::{FaultyBool, JsonReadError, JsonTrack, json_array_line::LRAJsonArrayLine};
use color::RGBColor;
use geometry::{Line, Point};
use lr_format_core::{
    GridVersion, LayerBuilder, LayerFolderBuilder, RemountVersion, RiderBuilder,
    SceneryLineBuilder, StandardLineBuilder, Track, TrackBuilder,
    unit_conversion::{from_lra_gravity, from_lra_zoom},
};
use vector2d::Vector2Df;

#[derive(PartialEq)]
enum LineType {
    Standard,
    Acceleration,
    Scenery,
}

pub fn read(bytes: &[u8]) -> Result<Track, JsonReadError> {
    let json_string = str::from_utf8(bytes)?;
    let json_track: JsonTrack = serde_json::from_str(&json_string)?;

    let grid_version = match json_track.version.as_str() {
        "6.0" => GridVersion::V6_0,
        "6.1" => GridVersion::V6_1,
        "6.2" => GridVersion::V6_2,
        other => Err(JsonReadError::UnsupportedGridVersion(other.to_string()))?,
    };

    let mut track = TrackBuilder::new(grid_version);

    let mut ordered_standard_lines = Vec::new();

    if let Some(line_list) = json_track.lines {
        for line in line_list {
            let line_type = match line.line_type {
                0 => LineType::Standard,
                1 => LineType::Acceleration,
                2 => LineType::Scenery,
                other => Err(JsonReadError::UnsupportedLineType(other.to_string()))?,
            };

            let endpoints = Line::new(Point::new(line.x1, line.y1), Point::new(line.x2, line.y2));

            let (left_extension, right_extension) = if line_type == LineType::Scenery {
                (false, false)
            } else if let Some(ext) = line.extended {
                (ext & 1 != 0, ext & 2 != 0)
            } else if let (Some(left_ext), Some(right_ext)) = (line.left_ext, line.right_ext) {
                (bool::from(left_ext), bool::from(right_ext))
            } else {
                (false, false)
            };

            let flipped = match line.flipped {
                None => false,
                Some(flipped) => bool::from(flipped),
            };

            let multiplier = if line_type == LineType::Acceleration {
                if let Some(multiplier) = line.multiplier {
                    multiplier
                } else {
                    1.0
                }
            } else {
                0.0
            };

            let width = if line_type == LineType::Scenery {
                if let Some(width) = line.width {
                    width
                } else {
                    1.0
                }
            } else {
                1.0
            };

            if line_type != LineType::Scenery {
                let mut standard_line = StandardLineBuilder::new(endpoints);
                standard_line.flipped(flipped);
                standard_line.left_extension(left_extension);
                standard_line.right_extension(right_extension);
                standard_line.multiplier(multiplier);
                ordered_standard_lines.push((line.id, standard_line));
            } else {
                let mut scenery_line = SceneryLineBuilder::new(endpoints);
                scenery_line.width(width);
                track.scenery_lines().push(scenery_line);
            }
        }
    }

    // Legacy line array
    if let Some(line_list) = json_track.line_array {
        for line in line_list {
            match line {
                LRAJsonArrayLine::Standard(id, x1, y1, x2, y2, extended, flipped) => {
                    let endpoints = Line::new(Point::new(x1, y1), Point::new(x2, y2));
                    let left_extension = extended & 0x1 != 0;
                    let right_extension = extended & 0x2 != 0;
                    let mut standard_line = StandardLineBuilder::new(endpoints);
                    standard_line.flipped(flipped);
                    standard_line.left_extension(left_extension);
                    standard_line.right_extension(right_extension);
                    ordered_standard_lines.push((id, standard_line));
                }
                LRAJsonArrayLine::Acceleration(
                    id,
                    x1,
                    y1,
                    x2,
                    y2,
                    extended,
                    flipped,
                    _,
                    _,
                    multiplier,
                ) => {
                    let endpoints = Line::new(Point::new(x1, y1), Point::new(x2, y2));
                    let left_extension = extended & 0x1 != 0;
                    let right_extension = extended & 0x2 != 0;
                    let mut standard_line = StandardLineBuilder::new(endpoints);
                    standard_line.flipped(flipped);
                    standard_line.left_extension(left_extension);
                    standard_line.right_extension(right_extension);
                    standard_line.multiplier(f64::from(multiplier));
                    ordered_standard_lines.push((id, standard_line));
                }
                LRAJsonArrayLine::Scenery(_id, x1, y1, x2, y2) => {
                    let endpoints = Line::new(Point::new(x1, y1), Point::new(x2, y2));
                    let scenery_line = SceneryLineBuilder::new(endpoints);
                    track.scenery_lines().push(scenery_line);
                }
            }
        }
    }

    ordered_standard_lines.sort_by_key(|t| t.0);

    for (_id, line) in ordered_standard_lines {
        track.standard_lines().push(line);
    }

    if let Some(layers) = json_track.layers {
        for json_layer in layers {
            let layer_is_folder = json_layer.size.is_some();

            if !layer_is_folder {
                let (color, name) =
                    if json_layer.name.len() < 7 || !json_layer.name.starts_with('#') {
                        (None, json_layer.name.clone())
                    } else {
                        let hex = &json_layer.name[..7];
                        let r = u8::from_str_radix(&hex[1..3], 16).ok();
                        let g = u8::from_str_radix(&hex[3..5], 16).ok();
                        let b = u8::from_str_radix(&hex[5..7], 16).ok();

                        match (r, g, b) {
                            (Some(r), Some(g), Some(b)) => (
                                Some(RGBColor::new(r, g, b)),
                                json_layer.name[7..].to_string(),
                            ),
                            _ => (None, json_layer.name.clone()),
                        }
                    };

                let mut layer = LayerBuilder::new(json_layer.id);
                layer.name(name);
                layer.visible(json_layer.visible);

                if let Some(color) = color {
                    layer.color(color);
                }

                if let Some(editable) = json_layer.editable {
                    layer.editable(editable);
                }

                if let Some(folder_id) = &json_layer.folder_id {
                    let folder_id = Option::<u32>::from(*folder_id);
                    if let Some(folder_id) = folder_id {
                        layer.folder_id(folder_id);
                    }
                }

                track.layers().push(layer);
            } else {
                let mut layer_folder = LayerFolderBuilder::new(json_layer.id);
                layer_folder.name(json_layer.name.to_string());
                layer_folder.visible(json_layer.visible);

                if let Some(editable) = json_layer.editable {
                    layer_folder.editable(editable);
                }

                track.layer_folders().push(layer_folder);
            }
        }
    }

    let rider_global_offset = if let Some(start_pos) = json_track.start_pos {
        Vector2Df::new(start_pos.x, start_pos.y)
    } else {
        Vector2Df::zero()
    };

    let zero_start = json_track
        .zero_start
        .is_some_and(|zero_start: bool| zero_start);

    if let Some(json_rider) = json_track.riders {
        for json_rider in json_rider {
            let start_position = Vector2Df::new(json_rider.start_pos.x, json_rider.start_pos.y);
            let start_velocity = Vector2Df::new(json_rider.start_vel.x, json_rider.start_vel.y);

            let mut rider = RiderBuilder::new(RemountVersion::None);
            rider.start_offset(start_position + rider_global_offset);
            rider.start_velocity(start_velocity);

            if let Some(remount) = &json_rider.remountable {
                let (remount_bool, remount_version) = match remount {
                    FaultyBool::BoolRep(x) => (*x, RemountVersion::ComV1),
                    FaultyBool::IntRep(x) => (*x != 0, RemountVersion::ComV2),
                };
                if remount_bool {
                    rider.remount_version(remount_version);
                } else {
                    rider.remount_version(RemountVersion::None);
                }
            }
            track.riders().push(rider);
        }
    } else {
        let start_velocity = if zero_start {
            Vector2Df::zero()
        } else {
            Vector2Df::new(0.4, 0.0)
        };
        let mut rider = RiderBuilder::new(RemountVersion::LRA);
        rider.start_offset(rider_global_offset);
        rider.start_velocity(start_velocity);
        track.riders().push(rider);
    }

    if let Some(label) = json_track.label {
        track.title(label);
    }

    if let Some(creator) = json_track.creator {
        track.artist(creator);
    }

    if let Some(description) = json_track.description {
        track.description(description);
    }

    if let Some(duration) = json_track.duration {
        track.duration(duration);
    }

    if let Some(gravity_well_size) = json_track.gravity_well_size {
        for line in track.standard_lines() {
            line.height(gravity_well_size);
        }
    }

    let _start_zoom = json_track
        .start_zoom
        .map(f64::from)
        .unwrap_or(from_lra_zoom(4.0));

    let _start_gravity = from_lra_gravity(Vector2Df::new(
        f64::from(json_track.start_gravity_x.unwrap_or(0.0)),
        f64::from(json_track.start_gravity_y.unwrap_or(1.0)),
    ));

    let _start_background_color = RGBColor::new(
        u8::try_from(json_track.start_bg_color_red.unwrap_or(244))?,
        u8::try_from(json_track.start_bg_color_green.unwrap_or(245))?,
        u8::try_from(json_track.start_bg_color_blue.unwrap_or(249))?,
    );

    let _start_line_color = RGBColor::new(
        u8::try_from(json_track.start_line_color_red.unwrap_or(0))?,
        u8::try_from(json_track.start_line_color_green.unwrap_or(0))?,
        u8::try_from(json_track.start_line_color_blue.unwrap_or(0))?,
    );

    if let Some(line_triggers) = json_track.line_based_triggers {
        for trigger in line_triggers {
            if trigger.zoom {}
        }
    }

    if let Some(time_triggers) = json_track.time_based_triggers {
        for trigger in time_triggers {
            // Closure just avoids moving the value
            let err = || JsonReadError::InvalidTriggerFormat(format!("{:?}", trigger));
            match trigger.trigger_type {
                0 => {
                    // Zoom
                    let _target_zoom = from_lra_zoom(trigger.zoom_target);
                    let _start_frame = trigger.start;
                    let _end_frame = trigger.end;
                }
                1 => {
                    // Background Color
                    let _red = u8::try_from(
                        Option::<u32>::from(trigger.background_red.ok_or_else(err)?)
                            .ok_or_else(err)?,
                    )?;
                    let _green = u8::try_from(
                        Option::<u32>::from(trigger.background_green.ok_or_else(err)?)
                            .ok_or_else(err)?,
                    )?;
                    let _blue = u8::try_from(
                        Option::<u32>::from(trigger.background_blue.ok_or_else(err)?)
                            .ok_or_else(err)?,
                    )?;
                    let _start_frame = trigger.start;
                    let _end_frame = trigger.end;
                }
                2 => {
                    // Line Color
                    let _red = u8::try_from(
                        Option::<u32>::from(trigger.line_red.ok_or_else(err)?).ok_or_else(err)?,
                    )?;
                    let _green = u8::try_from(
                        Option::<u32>::from(trigger.line_green.ok_or_else(err)?).ok_or_else(err)?,
                    )?;
                    let _blue = u8::try_from(
                        Option::<u32>::from(trigger.line_blue.ok_or_else(err)?).ok_or_else(err)?,
                    )?;
                    let _start_frame = trigger.start;
                    let _end_frame = trigger.end;
                }
                other => Err(JsonReadError::UnsupportedTriggerType(other.to_string()))?,
            }
        }
    }

    Ok(track.build())
}
