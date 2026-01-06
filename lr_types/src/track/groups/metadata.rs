use crate::track::GridVersion;
use color::RGBColor;
use vector2d::Vector2Df;

// way to store all data that previous versions have in their preserved form
// solver for converting between the data in those versions
// have a new version that's a summary of that data
// since this is an application, just read from old formats and parse them into the overall format
// the purpose of specs is to be able to create accurate writers, not old code

#[derive(Debug, PartialEq)]
pub struct Metadata {
    // Shared Properties
    grid_version: GridVersion,

    // Linerider.com Properties
    title: Option<String>,
    artist: Option<String>,
    description: Option<String>,
    duration: Option<u32>,

    // LRA+ Properties
    gravity_well_size: Option<f64>,
    audio_filename: Option<String>,
    audio_offset: Option<f64>, // Offset (in seconds) until the song starts
    start_gravity: Option<Vector2Df>,
    start_zoom: Option<f64>,
    start_line_color: Option<RGBColor>,
    start_background_color: Option<RGBColor>,
}

impl Metadata {
    pub fn grid_version(&self) -> GridVersion {
        self.grid_version
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

    pub fn gravity_well_size(&self) -> Option<f64> {
        self.gravity_well_size
    }

    pub fn audio_filename(&self) -> &Option<String> {
        &self.audio_filename
    }

    pub fn audio_offset_until_start(&self) -> Option<f64> {
        self.audio_offset
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
}

pub struct MetadataBuilder {
    // Shared Properties
    grid_version: GridVersion,
    // Linerider.com Properties
    title: Option<String>,
    artist: Option<String>,
    description: Option<String>,
    duration: Option<u32>,

    // LRA+ Properties
    gravity_well_size: Option<f64>,
    audio_filename: Option<String>,
    audio_offset: Option<f64>,
    start_gravity: Option<Vector2Df>,
    start_zoom: Option<f64>,
    start_line_color: Option<RGBColor>,
    start_background_color: Option<RGBColor>,
}

impl MetadataBuilder {
    pub fn new(grid_version: GridVersion) -> Self {
        MetadataBuilder {
            grid_version,
            title: None,
            artist: None,
            description: None,
            duration: None,
            gravity_well_size: None,
            audio_filename: None,
            audio_offset: None,
            start_gravity: None,
            start_zoom: None,
            start_line_color: None,
            start_background_color: None,
        }
    }

    pub fn grid_version(&mut self, grid_version: GridVersion) -> &mut Self {
        self.grid_version = grid_version;
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

    pub fn gravity_well_size(&mut self, gravity_well_size: f64) -> &mut Self {
        self.gravity_well_size = Some(gravity_well_size);
        self
    }

    pub fn audio_filename(&mut self, audio_filename: String) -> &mut Self {
        self.audio_filename = Some(audio_filename);
        self
    }

    pub fn audio_offset(&mut self, audio_offset: f64) -> &mut Self {
        self.audio_offset = Some(audio_offset);
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

    pub(crate) fn build(&self) -> Metadata {
        Metadata {
            grid_version: self.grid_version,
            title: self.title.clone(),
            artist: self.artist.clone(),
            description: self.description.clone(),
            duration: self.duration,
            gravity_well_size: self.gravity_well_size,
            audio_filename: self.audio_filename.clone(),
            audio_offset: self.audio_offset,
            start_gravity: self.start_gravity,
            start_zoom: self.start_zoom,
            start_line_color: self.start_line_color,
            start_background_color: self.start_background_color,
        }
    }
}
