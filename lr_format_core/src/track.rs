use crate::{GridVersion, Layer, LayerFolder, Rider, SceneryLine, StandardLine};

#[derive(Debug, PartialEq)]
pub struct Track {
    grid_version: GridVersion,
    title: Option<String>,
    artist: Option<String>,
    description: Option<String>,
    duration: Option<u32>,
    audio_filename: Option<String>,
    audio_offset: Option<f64>, // Offset (in seconds) until the song starts
    standard_lines: Vec<StandardLine>,
    scenery_lines: Vec<SceneryLine>,
    layers: Option<Vec<Layer>>,
    layer_folders: Option<Vec<LayerFolder>>,
    riders: Option<Vec<Rider>>,
    // background_color_group: Option<BackgroundColorGroup>,
    // line_color_group: Option<LineColorGroup>,
    // camera_zoom_group: Option<CameraZoomGroup>,
    // legacy_camera_zoom_group: Option<LegacyCameraZoomGroup>,
}

impl Track {
    pub fn new(grid_version: GridVersion) -> Self {
        Self {
            grid_version,
            standard_lines: Vec::new(),
            scenery_lines: Vec::new(),
            title: None,
            artist: None,
            description: None,
            duration: None,
            audio_filename: None,
            audio_offset: None,
            layers: None,
            layer_folders: None,
            riders: None,
        }
    }

    pub fn grid_version(&self) -> GridVersion {
        self.grid_version
    }

    pub fn set_grid_version(&mut self, grid_version: GridVersion) {
        self.grid_version = grid_version;
    }

    pub fn title(&self) -> &Option<String> {
        &self.title
    }

    pub fn set_title(&mut self, title: String) {
        self.title = Some(title);
    }

    pub fn artist(&self) -> &Option<String> {
        &self.artist
    }

    pub fn set_artist(&mut self, artist: String) {
        self.artist = Some(artist);
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn duration(&self) -> Option<u32> {
        self.duration
    }

    pub fn set_duration(&mut self, duration: u32) {
        self.duration = Some(duration);
    }

    pub fn audio_filename(&self) -> &Option<String> {
        &self.audio_filename
    }

    pub fn set_audio_filename(&mut self, audio_filename: String) {
        self.audio_filename = Some(audio_filename);
    }

    pub fn audio_offset_until_start(&self) -> Option<f64> {
        self.audio_offset
    }

    pub fn set_audio_offset_until_start(&mut self, audio_offset_until_start: f64) {
        self.audio_offset = Some(audio_offset_until_start);
    }

    pub fn standard_lines(&self) -> &Vec<StandardLine> {
        &self.standard_lines
    }

    pub fn standard_lines_mut(&mut self) -> &mut Vec<StandardLine> {
        &mut self.standard_lines
    }

    pub fn scenery_lines(&self) -> &Vec<SceneryLine> {
        &self.scenery_lines
    }

    pub fn scenery_lines_mut(&mut self) -> &mut Vec<SceneryLine> {
        &mut self.scenery_lines
    }

    pub fn layers(&self) -> &Option<Vec<Layer>> {
        &self.layers
    }

    pub fn layers_mut(&mut self) -> &mut Vec<Layer> {
        self.layers.get_or_insert_with(Vec::new)
    }

    pub fn layer_folders(&self) -> &Option<Vec<LayerFolder>> {
        &self.layer_folders
    }

    pub fn layer_folders_mut(&mut self) -> &mut Vec<LayerFolder> {
        self.layer_folders.get_or_insert_with(Vec::new)
    }

    pub fn riders(&self) -> &Option<Vec<Rider>> {
        &self.riders
    }

    pub fn riders_mut(&mut self) -> &mut Vec<Rider> {
        self.riders.get_or_insert_with(Vec::new)
    }
}
