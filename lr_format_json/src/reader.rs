use color::RGBColor;
use geometry::{Line, Point};
use lr_format_core::{
    GridVersion, Layer, LayerFolder, RemountVersion, Rider, SceneryLine, StandardLine, Track,
};
use vector2d::Vector2Df;

use crate::{FaultyBool, JsonReadError, JsonTrack, json_array_line::LRAJsonArrayLine};

pub fn read(bytes: &[u8]) -> Result<Track, JsonReadError> {
    let json_string = str::from_utf8(bytes)?;
    let json_track: JsonTrack = serde_json::from_str(&json_string)?;

    let grid_version = match json_track.version.as_str() {
        "6.0" => GridVersion::V6_0,
        "6.1" => GridVersion::V6_1,
        "6.2" => GridVersion::V6_2,
        other => Err(JsonReadError::UnsupportedGridVersion(other.to_string()))?,
    };

    let mut track = Track::new(grid_version);

    let mut ordered_standard_lines = Vec::new();

    if let Some(line_list) = json_track.lines {
        for line in line_list {
            let is_standard_line = match line.line_type {
                0 | 1 => true,
                2 => false,
                other => Err(JsonReadError::UnsupportedLineType(other.to_string()))?,
            };

            let endpoints = Line::new(Point::new(line.x1, line.y1), Point::new(line.x2, line.y2));

            let (left_extension, right_extension) = if !is_standard_line {
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

            let multiplier = if line.line_type == 1 {
                if let Some(multiplier) = line.multiplier {
                    multiplier
                } else {
                    1.0
                }
            } else {
                0.0
            };

            let width = if line.line_type == 2 {
                if let Some(width) = line.width {
                    width
                } else {
                    1.0
                }
            } else {
                1.0
            };

            if is_standard_line {
                let mut standard_line = StandardLine::new(endpoints);
                standard_line.set_flipped(flipped);
                standard_line.set_left_extension(left_extension);
                standard_line.set_right_extension(right_extension);
                standard_line.set_multiplier(multiplier);
                ordered_standard_lines.push((line.id, standard_line));
            } else {
                let mut scenery_line = SceneryLine::new(endpoints);
                scenery_line.set_width(width);
                track.scenery_lines_mut().push(scenery_line);
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
                    let mut standard_line = StandardLine::new(endpoints);
                    standard_line.set_flipped(flipped);
                    standard_line.set_left_extension(left_extension);
                    standard_line.set_right_extension(right_extension);
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
                    let mut standard_line = StandardLine::new(endpoints);
                    standard_line.set_flipped(flipped);
                    standard_line.set_left_extension(left_extension);
                    standard_line.set_right_extension(right_extension);
                    standard_line.set_multiplier(f64::from(multiplier));
                    ordered_standard_lines.push((id, standard_line));
                }
                LRAJsonArrayLine::Scenery(_id, x1, y1, x2, y2) => {
                    let endpoints = Line::new(Point::new(x1, y1), Point::new(x2, y2));
                    let scenery_line = SceneryLine::new(endpoints);
                    track.scenery_lines_mut().push(scenery_line);
                }
            }
        }
    }

    ordered_standard_lines.sort_by_key(|t| t.0);

    for (_id, line) in ordered_standard_lines {
        track.standard_lines_mut().push(line);
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

                let mut layer = Layer::new(json_layer.id);
                layer.set_name(name);
                layer.set_visible(json_layer.visible);

                if let Some(color) = color {
                    layer.set_color(color);
                }

                if let Some(editable) = json_layer.editable {
                    layer.set_editable(editable);
                }

                if let Some(folder_id) = &json_layer.folder_id {
                    let folder_id = Option::<u32>::from(*folder_id);
                    if let Some(folder_id) = folder_id {
                        layer.set_folder_id(folder_id);
                    }
                }

                track.layers_mut().push(layer);
            } else {
                let mut layer_folder = LayerFolder::new(json_layer.id);
                layer_folder.set_name(json_layer.name.to_string());
                layer_folder.set_visible(json_layer.visible);

                if let Some(editable) = json_layer.editable {
                    layer_folder.set_editable(editable);
                }

                track.layer_folders_mut().push(layer_folder);
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

            let mut rider = Rider::new(RemountVersion::None);
            rider.set_start_offset(start_position + rider_global_offset);
            rider.set_start_velocity(start_velocity);

            if let Some(remount) = &json_rider.remountable {
                let (remount_bool, remount_version) = match remount {
                    FaultyBool::BoolRep(x) => (*x, RemountVersion::ComV1),
                    FaultyBool::IntRep(x) => (*x != 0, RemountVersion::ComV2),
                };
                if remount_bool {
                    rider.set_remount_version(remount_version);
                } else {
                    rider.set_remount_version(RemountVersion::None);
                }
            }
            track.riders_mut().push(rider);
        }
    } else {
        let start_velocity = if zero_start {
            Vector2Df::zero()
        } else {
            Vector2Df::new(0.4, 0.0)
        };
        let mut rider = Rider::new(RemountVersion::LRA);
        rider.set_start_offset(rider_global_offset);
        rider.set_start_velocity(start_velocity);
        track.riders_mut().push(rider);
    }

    if let Some(label) = json_track.label {
        track.set_title(label);
    }

    if let Some(creator) = json_track.creator {
        track.set_artist(creator);
    }

    if let Some(description) = json_track.description {
        track.set_description(description);
    }

    if let Some(duration) = json_track.duration {
        track.set_duration(duration);
    }

    if let Some(gravity_well_size) = json_track.gravity_well_size {
        for line in track.standard_lines_mut() {
            line.set_height(gravity_well_size);
        }
    }

    /*
       if let Some(x_gravity) = json_track.x_gravity
           && let Some(y_gravity) = json_track.y_gravity
       {
           track
               .metadata()
               .start_gravity(Vector2Df::new(f64::from(x_gravity), f64::from(y_gravity)));
       }

       if let Some(start_zoom) = json_track.start_zoom {
           track.metadata().start_zoom(from_lra_zoom(start_zoom));
       }

       if let Some(init_red) = json_track.line_color_red
           && let Some(init_green) = json_track.line_color_green
           && let Some(init_blue) = json_track.line_color_blue
       {
           track.metadata().start_line_color(RGBColor::new(
               u8::try_from(init_red)?,
               u8::try_from(init_green)?,
               u8::try_from(init_blue)?,
           ));
       }

       if let Some(init_red) = json_track.background_color_red
           && let Some(init_green) = json_track.background_color_green
           && let Some(init_blue) = json_track.background_color_blue
       {
           track.metadata().start_background_color(RGBColor::new(
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
                   track
                       .legacy_camera_zoom_group()
                       .add_trigger(zoom_event, line_hit);
               }
           }
       }

       if let Some(time_triggers) = json_track.time_based_triggers {
           for trigger in time_triggers {
               // Closure just avoids moving the value
               let err = || JsonReadError::InvalidTriggerFormat(format!("{:?}", trigger));
               match trigger.trigger_type {
                   0 => {
                       // Zoom
                       let target_zoom = from_lra_zoom(trigger.zoom_target);
                       let start_frame = trigger.start;
                       let end_frame = trigger.end;
                       let zoom_event = CameraZoomEvent::new(target_zoom);
                       let frame_bounds = FrameBoundsTrigger::new(start_frame, end_frame);
                       track
                           .camera_zoom_group()
                           .add_trigger(zoom_event, frame_bounds);
                   }
                   1 => {
                       // Background Color
                       let red = u8::try_from(
                           Option::<u32>::from(trigger.background_red.ok_or_else(err)?)
                               .ok_or_else(err)?,
                       )?;
                       let green = u8::try_from(
                           Option::<u32>::from(trigger.background_green.ok_or_else(err)?)
                               .ok_or_else(err)?,
                       )?;
                       let blue = u8::try_from(
                           Option::<u32>::from(trigger.background_blue.ok_or_else(err)?)
                               .ok_or_else(err)?,
                       )?;
                       let start_frame = trigger.start;
                       let end_frame = trigger.end;
                       let bg_color_event = BackgroundColorEvent::new(RGBColor::new(red, green, blue));
                       let frame_bounds = FrameBoundsTrigger::new(start_frame, end_frame);
                       track
                           .background_color_group()
                           .add_trigger(bg_color_event, frame_bounds);
                   }
                   2 => {
                       // Line Color
                       let red = u8::try_from(
                           Option::<u32>::from(trigger.line_red.ok_or_else(err)?).ok_or_else(err)?,
                       )?;
                       let green = u8::try_from(
                           Option::<u32>::from(trigger.line_green.ok_or_else(err)?).ok_or_else(err)?,
                       )?;
                       let blue = u8::try_from(
                           Option::<u32>::from(trigger.line_blue.ok_or_else(err)?).ok_or_else(err)?,
                       )?;
                       let start_frame = trigger.start;
                       let end_frame = trigger.end;
                       let line_color_event = LineColorEvent::new(RGBColor::new(red, green, blue));
                       let frame_bounds = FrameBoundsTrigger::new(start_frame, end_frame);
                       track
                           .line_color_group()
                           .add_trigger(line_color_event, frame_bounds);
                   }
                   other => Err(JsonReadError::UnsupportedTriggerType(other.to_string()))?,
               }
           }
       }
    */

    Ok(track)
}
