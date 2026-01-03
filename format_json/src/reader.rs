use color::RGBColor;
use format_core::{
    track::{
        BackgroundColorEvent, CameraZoomEvent, FrameBoundsTrigger, LineColorEvent, LineHitTrigger,
        LineType, RemountVersion, Track, TrackBuilder,
    },
    util::from_lra_zoom,
};
use spatial_grid::GridVersion;
use vector2d::Vector2Df;

use crate::{FaultyBool, FaultyU32, JsonReadError, JsonTrack, LRAJsonArrayLine};

pub fn read(data: &Vec<u8>) -> Result<Track, JsonReadError> {
    let json_string = String::from_utf8(data.to_vec())?;
    let json_track: JsonTrack = serde_json::from_str(&json_string)?;

    let grid_version = match json_track.version.as_str() {
        "6.0" => GridVersion::V6_0,
        "6.1" => GridVersion::V6_1,
        "6.2" => GridVersion::V6_2,
        other => Err(JsonReadError::InvalidData {
            name: "grid version",
            value: other.to_string(),
        })?,
    };

    let track_builder = &mut TrackBuilder::new(grid_version);

    if let Some(line_list) = json_track.lines {
        for line in line_list {
            let line_type = match line.line_type {
                0 => LineType::Standard,
                1 => LineType::Acceleration,
                2 => LineType::Scenery,
                other => Err(JsonReadError::InvalidData {
                    name: "line type",
                    value: other.to_string(),
                })?,
            };

            let endpoints = (
                Vector2Df::new(line.x1, line.y1),
                Vector2Df::new(line.x2, line.y2),
            );

            let (left_extension, right_extension) = if line_type == LineType::Scenery {
                (false, false)
            } else if let Some(ext) = line.extended {
                (ext & 1 != 0, ext & 2 != 0)
            } else if let (Some(left_ext), Some(right_ext)) = (line.left_ext, line.right_ext) {
                let left_ext_bool = match left_ext {
                    FaultyBool::BoolRep(x) => x,
                    FaultyBool::IntRep(x) => x == 1,
                };
                let right_ext_bool = match right_ext {
                    FaultyBool::BoolRep(x) => x,
                    FaultyBool::IntRep(x) => x == 1,
                };
                (left_ext_bool, right_ext_bool)
            } else {
                Err(JsonReadError::InvalidData {
                    name: "line extension",
                    value: "None".to_string(),
                })?
            };

            let flipped = match line.flipped {
                None => false,
                Some(FaultyBool::BoolRep(x)) => x,
                Some(FaultyBool::IntRep(x)) => x == 1,
            };

            match line_type {
                LineType::Standard => {
                    track_builder
                        .line_group()
                        .add_standard_line(line.id, endpoints)
                        .flipped(flipped)
                        .left_extension(left_extension)
                        .right_extension(right_extension);
                }
                LineType::Acceleration => {
                    let line_builder = track_builder
                        .line_group()
                        .add_acceleration_line(line.id, endpoints)
                        .flipped(flipped)
                        .left_extension(left_extension)
                        .right_extension(right_extension);
                    if let Some(multiplier) = line.multiplier {
                        line_builder.multiplier(multiplier);
                    }
                }
                LineType::Scenery => {
                    let line_builder = track_builder.line_group().add_scenery_line(endpoints);
                    if let Some(width) = line.width {
                        line_builder.width(width);
                    }
                }
            }
        }
    }

    // Legacy line array
    if let Some(line_list) = json_track.line_array {
        for line in line_list {
            match line {
                LRAJsonArrayLine::Standard(id, x1, y1, x2, y2, extended, flipped) => {
                    let endpoints = (Vector2Df::new(x1, y1), Vector2Df::new(x2, y2));
                    let left_extension = extended & 0x1 != 0;
                    let right_extension = extended & 0x2 != 0;
                    track_builder
                        .line_group()
                        .add_standard_line(id, endpoints)
                        .flipped(flipped)
                        .left_extension(left_extension)
                        .right_extension(right_extension);
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
                    let endpoints = (Vector2Df::new(x1, y1), Vector2Df::new(x2, y2));
                    let left_extension = extended & 0x1 != 0;
                    let right_extension = extended & 0x2 != 0;
                    track_builder
                        .line_group()
                        .add_acceleration_line(id, endpoints)
                        .flipped(flipped)
                        .left_extension(left_extension)
                        .right_extension(right_extension)
                        .multiplier(f64::from(multiplier));
                }
                LRAJsonArrayLine::Scenery(_id, x1, y1, x2, y2) => {
                    let endpoints = (Vector2Df::new(x1, y1), Vector2Df::new(x2, y2));
                    track_builder.line_group().add_scenery_line(endpoints);
                }
            }
        }
    }

    if let Some(layers) = json_track.layers {
        for (index, layer) in layers.iter().enumerate() {
            let layer_is_folder = layer.size.is_some();

            if !layer_is_folder {
                let (layer_color, layer_name) =
                    if layer.name.len() < 7 || !layer.name.starts_with('#') {
                        (None, layer.name.clone())
                    } else {
                        let hex = &layer.name[..7];
                        let r = u8::from_str_radix(&hex[1..3], 16).ok();
                        let g = u8::from_str_radix(&hex[3..5], 16).ok();
                        let b = u8::from_str_radix(&hex[5..7], 16).ok();

                        match (r, g, b) {
                            (Some(r), Some(g), Some(b)) => {
                                (Some(RGBColor::new(r, g, b)), layer.name[7..].to_string())
                            }
                            _ => (None, layer.name.clone()),
                        }
                    };

                let layer_builder = track_builder
                    .layer_group()
                    .add_layer(layer.id, index)
                    .name(layer_name)
                    .visible(layer.visible);

                if let Some(color) = layer_color {
                    layer_builder.color(color);
                }

                if let Some(editable) = layer.editable {
                    layer_builder.editable(editable);
                }

                if let Some(folder_id) = &layer.folder_id {
                    if let FaultyU32::Valid(valid_folder_id) = folder_id {
                        layer_builder.folder_id(*valid_folder_id);
                    }
                    track_builder.layer_group().enable_layer_folders();
                }
            } else {
                let layer_folder_builder = track_builder
                    .layer_group()
                    .add_layer_folder(layer.id, index, 0)
                    .name(layer.name.to_string())
                    .visible(layer.visible);

                if let Some(editable) = layer.editable {
                    layer_folder_builder.editable(editable);
                }

                if let Some(size) = layer.size {
                    layer_folder_builder.size(size);
                }
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

    if let Some(riders) = json_track.riders {
        for (index, rider) in riders.iter().enumerate() {
            let start_position = Vector2Df::new(rider.start_pos.x, rider.start_pos.y);
            let start_velocity = Vector2Df::new(rider.start_vel.x, rider.start_vel.y);

            let rider_builder = track_builder
                .rider_group()
                .add_rider(RemountVersion::None, index as u32)
                .start_position(start_position + rider_global_offset)
                .start_velocity(start_velocity);

            if let Some(angle) = rider.angle {
                rider_builder.start_angle(angle);
            }

            if let Some(remount) = &rider.remountable {
                let (remount_bool, remount_version) = match remount {
                    FaultyBool::BoolRep(x) => (*x, RemountVersion::ComV1),
                    FaultyBool::IntRep(x) => (*x == 1, RemountVersion::ComV2),
                };
                if remount_bool {
                    rider_builder.remount_version(remount_version);
                } else {
                    rider_builder.remount_version(RemountVersion::None);
                }
            }
        }
    } else {
        let start_velocity = if zero_start {
            Vector2Df::zero()
        } else {
            Vector2Df::new(0.4, 0.0)
        };
        track_builder
            .rider_group()
            .add_rider(RemountVersion::LRA, 0)
            .start_angle(0.0)
            .start_position(rider_global_offset)
            .start_velocity(start_velocity);
    }

    if let Some(label) = json_track.label {
        track_builder.metadata().title(label);
    }

    if let Some(creator) = json_track.creator {
        track_builder.metadata().artist(creator);
    }

    if let Some(description) = json_track.description {
        track_builder.metadata().description(description);
    }

    if let Some(duration) = json_track.duration {
        track_builder.metadata().duration(duration);
    }

    if let Some(script) = json_track.script {
        track_builder.metadata().script(script);
    }

    if let Some(gravity_well_size) = json_track.gravity_well_size {
        track_builder
            .metadata()
            .gravity_well_size(gravity_well_size);
    }

    if let Some(x_gravity) = json_track.x_gravity
        && let Some(y_gravity) = json_track.y_gravity
    {
        track_builder
            .metadata()
            .start_gravity(Vector2Df::new(f64::from(x_gravity), f64::from(y_gravity)));
    }

    if let Some(start_zoom) = json_track.start_zoom {
        track_builder
            .metadata()
            .start_zoom(from_lra_zoom(start_zoom));
    }

    if let Some(init_red) = json_track.line_color_red
        && let Some(init_green) = json_track.line_color_green
        && let Some(init_blue) = json_track.line_color_blue
    {
        track_builder.metadata().start_line_color(RGBColor::new(
            u8::try_from(init_red)?,
            u8::try_from(init_green)?,
            u8::try_from(init_blue)?,
        ));
    }

    if let Some(init_red) = json_track.background_color_red
        && let Some(init_green) = json_track.background_color_green
        && let Some(init_blue) = json_track.background_color_blue
    {
        track_builder
            .metadata()
            .start_background_color(RGBColor::new(
                u8::try_from(init_red)?,
                u8::try_from(init_green)?,
                u8::try_from(init_blue)?,
            ));
    }

    if let Some(line_triggers) = json_track.line_based_triggers {
        for trigger in line_triggers {
            if trigger.zoom {
                let line_hit = LineHitTrigger::new(trigger.id, trigger.frames);
                let zoom_event = CameraZoomEvent::new(from_lra_zoom(trigger.target));
                track_builder
                    .legacy_camera_zoom_group()
                    .add_trigger(zoom_event, line_hit);
            }
        }
    }

    if let Some(time_triggers) = json_track.time_based_triggers {
        for trigger in time_triggers {
            match trigger.trigger_type {
                0 => {
                    // Zoom
                    let target_zoom = from_lra_zoom(trigger.zoom_target);
                    let start_frame = trigger.start;
                    let end_frame = trigger.end;
                    let zoom_event = CameraZoomEvent::new(target_zoom);
                    let frame_bounds = FrameBoundsTrigger::new(start_frame, end_frame);
                    track_builder
                        .camera_zoom_group()
                        .add_trigger(zoom_event, frame_bounds);
                }
                1 => {
                    // Background Color
                    let red = extract_u8(&trigger.background_red, "background red")?;
                    let green = extract_u8(&trigger.background_green, "background green")?;
                    let blue = extract_u8(&trigger.background_blue, "background blue")?;
                    let start_frame = trigger.start;
                    let end_frame = trigger.end;
                    let bg_color_event = BackgroundColorEvent::new(RGBColor::new(red, green, blue));
                    let frame_bounds = FrameBoundsTrigger::new(start_frame, end_frame);
                    track_builder
                        .background_color_group()
                        .add_trigger(bg_color_event, frame_bounds);
                }
                2 => {
                    // Line Color
                    let red = extract_u8(&trigger.line_red, "line red")?;
                    let green = extract_u8(&trigger.line_green, "line green")?;
                    let blue = extract_u8(&trigger.line_blue, "line blue")?;
                    let start_frame = trigger.start;
                    let end_frame = trigger.end;
                    let line_color_event = LineColorEvent::new(RGBColor::new(red, green, blue));
                    let frame_bounds = FrameBoundsTrigger::new(start_frame, end_frame);
                    track_builder
                        .line_color_group()
                        .add_trigger(line_color_event, frame_bounds);
                }
                other => Err(JsonReadError::InvalidData {
                    name: "trigger type",
                    value: other.to_string(),
                })?,
            }
        }
    }

    Ok(track_builder.build())
}

fn extract_u8(value: &Option<FaultyU32>, field: &'static str) -> Result<u8, JsonReadError> {
    match value {
        Some(value) => match value {
            FaultyU32::Valid(value) => {
                u8::try_from(*value).map_err(|_err| JsonReadError::InvalidData {
                    name: field,
                    value: value.to_string(),
                })
            }
            FaultyU32::Invalid(value) => Err(JsonReadError::InvalidData {
                name: field,
                value: value.to_string(),
            }),
        },
        None => Err(JsonReadError::InvalidData {
            name: field,
            value: "None".to_string(),
        }),
    }
}
