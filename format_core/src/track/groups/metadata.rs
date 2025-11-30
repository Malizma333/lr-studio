use vector2d::Vector2Df;

use crate::track::{GridVersion, RGBColor};

#[derive(Debug)]
pub struct Metadata {
    // Shared Properties
    grid_version: GridVersion,
    start_position: Option<Vector2Df>,

    // Linerider.com Properties
    title: Option<String>,
    artist: Option<String>,
    description: Option<String>,
    duration: Option<u32>,
    script: Option<String>,

    // LRA+ Properties
    gravity_well_size: Option<f64>,
    audio_filename: Option<String>,
    audio_offset_until_start: Option<f64>,
    start_gravity: Option<Vector2Df>,
    start_zoom: Option<f64>,
    start_line_color: Option<RGBColor>,
    start_background_color: Option<RGBColor>,
    lra_remount: Option<bool>,
    legacy_lra_fakie: Option<bool>,
    zero_friction_riders: bool,
    zero_velocity_start_riders: bool,
    remount_riders: Option<bool>,

    // Flash Properties
    start_line: Option<u32>,
}

impl Metadata {
    pub fn grid_version(&self) -> GridVersion {
        self.grid_version
    }

    pub fn start_position(&self) -> Option<Vector2Df> {
        self.start_position
    }

    pub fn title(&self) -> &Option<String> {
        &self.title
    }

    pub fn artist(&self) -> &Option<String> {
        &self.artist
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn duration(&self) -> Option<u32> {
        self.duration
    }

    pub fn script(&self) -> &Option<String> {
        &self.script
    }

    pub fn gravity_well_size(&self) -> Option<f64> {
        self.gravity_well_size
    }

    pub fn audio_filename(&self) -> &Option<String> {
        &self.audio_filename
    }

    pub fn audio_offset_until_start(&self) -> Option<f64> {
        self.audio_offset_until_start
    }

    pub fn start_gravity(&self) -> Option<Vector2Df> {
        self.start_gravity
    }

    pub fn start_zoom(&self) -> Option<f64> {
        self.start_zoom
    }

    pub fn start_line_color(&self) -> Option<RGBColor> {
        self.start_line_color
    }

    pub fn start_background_color(&self) -> Option<RGBColor> {
        self.start_background_color
    }

    pub fn lra_remount(&self) -> Option<bool> {
        self.lra_remount
    }

    pub fn legacy_lra_fakie(&self) -> Option<bool> {
        self.legacy_lra_fakie
    }

    pub fn zero_friction_riders(&self) -> bool {
        self.zero_friction_riders
    }

    pub fn zero_velocity_start_riders(&self) -> bool {
        self.zero_velocity_start_riders
    }

    pub fn remount_riders(&self) -> Option<bool> {
        self.remount_riders
    }

    pub fn start_line(&self) -> Option<u32> {
        self.start_line
    }
}

pub struct MetadataBuilder {
    // Shared Properties
    grid_version: GridVersion,
    start_position: Option<Vector2Df>,
    // Linerider.com Properties
    title: Option<String>,
    artist: Option<String>,
    description: Option<String>,
    duration: Option<u32>,
    script: Option<String>,

    // LRA+ Properties
    gravity_well_size: Option<f64>,
    audio_filename: Option<String>,
    audio_offset_until_start: Option<f64>,
    start_gravity: Option<Vector2Df>,
    start_zoom: Option<f64>,
    start_line_color: Option<RGBColor>,
    start_background_color: Option<RGBColor>,
    lra_remount: Option<bool>,
    legacy_lra_fakie: Option<bool>,
    zero_friction_riders: bool,
    zero_velocity_start_riders: bool,
    remount_riders: Option<bool>,

    // Flash Properties
    start_line: Option<u32>,
}

impl MetadataBuilder {
    pub fn new(grid_version: GridVersion) -> Self {
        MetadataBuilder {
            grid_version,
            start_position: None,
            title: None,
            artist: None,
            description: None,
            duration: None,
            script: None,
            gravity_well_size: None,
            audio_filename: None,
            audio_offset_until_start: None,
            start_gravity: None,
            start_zoom: None,
            start_line_color: None,
            start_background_color: None,
            lra_remount: None,
            legacy_lra_fakie: None,
            zero_friction_riders: false,
            zero_velocity_start_riders: false,
            remount_riders: None,
            start_line: None,
        }
    }

    pub fn grid_version(&mut self, grid_version: GridVersion) -> &mut Self {
        self.grid_version = grid_version;
        self
    }

    pub fn start_position(&mut self, start_position: Vector2Df) -> &mut Self {
        self.start_position = Some(start_position);
        self
    }

    pub fn title(&mut self, title: String) -> &mut Self {
        self.title = Some(title);
        self
    }

    pub fn artist(&mut self, artist: String) -> &mut Self {
        self.artist = Some(artist);
        self
    }

    pub fn description(&mut self, description: String) -> &mut Self {
        self.description = Some(description);
        self
    }

    pub fn duration(&mut self, duration: u32) -> &mut Self {
        self.duration = Some(duration);
        self
    }

    pub fn script(&mut self, script: String) -> &mut Self {
        self.script = Some(script);
        self
    }

    pub fn gravity_well_size(&mut self, gravity_well_size: f64) -> &mut Self {
        self.gravity_well_size = Some(gravity_well_size);
        self
    }

    pub fn audio_filename(&mut self, audio_filename: String) -> &mut Self {
        self.audio_filename = Some(audio_filename);
        self
    }

    pub fn audio_offset_until_start(&mut self, audio_offset_until_start: f64) -> &mut Self {
        self.audio_offset_until_start = Some(audio_offset_until_start);
        self
    }

    pub fn start_gravity(&mut self, start_gravity: Vector2Df) -> &mut Self {
        self.start_gravity = Some(start_gravity);
        self
    }

    pub fn start_zoom(&mut self, start_zoom: f64) -> &mut Self {
        self.start_zoom = Some(start_zoom);
        self
    }

    pub fn start_line_color(&mut self, start_line_color: RGBColor) -> &mut Self {
        self.start_line_color = Some(start_line_color);
        self
    }

    pub fn start_background_color(&mut self, start_background_color: RGBColor) -> &mut Self {
        self.start_background_color = Some(start_background_color);
        self
    }

    pub fn lra_remount(&mut self, lra_remount: bool) -> &mut Self {
        self.lra_remount = Some(lra_remount);
        self
    }

    pub fn legacy_lra_fakie(&mut self, legacy_lra_fakie: bool) -> &mut Self {
        self.legacy_lra_fakie = Some(legacy_lra_fakie);
        self
    }

    pub fn zero_friction_riders(&mut self, zero_friction_riders: bool) -> &mut Self {
        self.zero_friction_riders = zero_friction_riders;
        self
    }

    pub fn zero_velocity_start_riders(&mut self, zero_velocity_start_riders: bool) -> &mut Self {
        self.zero_velocity_start_riders = zero_velocity_start_riders;
        self
    }

    pub fn remount_riders(&mut self, remount_riders: bool) -> &mut Self {
        self.remount_riders = Some(remount_riders);
        self
    }

    pub fn start_line(&mut self, start_line: u32) -> &mut Self {
        self.start_line = Some(start_line);
        self
    }

    pub fn build(&self) -> Metadata {
        Metadata {
            grid_version: self.grid_version,
            start_position: self.start_position,
            title: self.title.clone(),
            artist: self.artist.clone(),
            description: self.description.clone(),
            duration: self.duration,
            script: self.script.clone(),
            gravity_well_size: self.gravity_well_size,
            audio_filename: self.audio_filename.clone(),
            audio_offset_until_start: self.audio_offset_until_start,
            start_gravity: self.start_gravity,
            start_zoom: self.start_zoom,
            start_line_color: self.start_line_color,
            start_background_color: self.start_background_color,
            lra_remount: self.lra_remount,
            legacy_lra_fakie: self.legacy_lra_fakie,
            zero_friction_riders: self.zero_friction_riders,
            zero_velocity_start_riders: self.zero_velocity_start_riders,
            remount_riders: self.remount_riders,
            start_line: self.start_line,
        }
    }
}
