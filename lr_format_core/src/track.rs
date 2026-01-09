use crate::{
    GridVersion, Layer, LayerBuilder, LayerFolder, LayerFolderBuilder, Rider, RiderBuilder,
    SceneryLine, SceneryLineBuilder, StandardLine, StandardLineBuilder,
};

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
    layers: Vec<Layer>,
    layer_folders: Vec<LayerFolder>,
    riders: Vec<Rider>,
}

impl Track {
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

    pub fn audio_filename(&self) -> &Option<String> {
        &self.audio_filename
    }

    pub fn audio_offset_until_start(&self) -> Option<f64> {
        self.audio_offset
    }

    pub fn standard_lines(&self) -> &Vec<StandardLine> {
        &self.standard_lines
    }

    pub fn scenery_lines(&self) -> &Vec<SceneryLine> {
        &self.scenery_lines
    }

    pub fn layers(&self) -> &Vec<Layer> {
        &self.layers
    }

    pub fn layer_folders(&self) -> &Vec<LayerFolder> {
        &self.layer_folders
    }

    pub fn riders(&self) -> &Vec<Rider> {
        &self.riders
    }
}

pub struct TrackBuilder {
    grid_version: GridVersion,
    title: Option<String>,
    artist: Option<String>,
    description: Option<String>,
    duration: Option<u32>,
    audio_filename: Option<String>,
    audio_offset: Option<f64>, // Offset (in seconds) until the song starts
    standard_lines: Vec<StandardLineBuilder>,
    scenery_lines: Vec<SceneryLineBuilder>,
    layers: Vec<LayerBuilder>,
    layer_folders: Vec<LayerFolderBuilder>,
    riders: Vec<RiderBuilder>,
}

impl TrackBuilder {
    pub fn new(grid_version: GridVersion) -> Self {
        Self {
            grid_version,
            title: None,
            artist: None,
            description: None,
            duration: None,
            audio_filename: None,
            audio_offset: None,
            standard_lines: Vec::new(),
            scenery_lines: Vec::new(),
            layers: Vec::new(),
            layer_folders: Vec::new(),
            riders: Vec::new(),
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

    pub fn audio_filename(&mut self, audio_filename: String) -> &mut Self {
        self.audio_filename = Some(audio_filename);
        self
    }

    pub fn audio_offset_until_start(&mut self, audio_offset_until_start: f64) -> &mut Self {
        self.audio_offset = Some(audio_offset_until_start);
        self
    }

    pub fn standard_lines(&mut self) -> &mut Vec<StandardLineBuilder> {
        &mut self.standard_lines
    }

    pub fn scenery_lines(&mut self) -> &mut Vec<SceneryLineBuilder> {
        &mut self.scenery_lines
    }

    pub fn layers(&mut self) -> &mut Vec<LayerBuilder> {
        &mut self.layers
    }

    pub fn layer_folders(&mut self) -> &mut Vec<LayerFolderBuilder> {
        &mut self.layer_folders
    }

    pub fn riders(&mut self) -> &mut Vec<RiderBuilder> {
        &mut self.riders
    }

    pub fn build(self) -> Track {
        Track {
            grid_version: self.grid_version,
            title: self.title,
            artist: self.artist,
            description: self.description,
            duration: self.duration,
            audio_filename: self.audio_filename,
            audio_offset: self.audio_offset,
            standard_lines: self.standard_lines.into_iter().map(|x| x.build()).collect(),
            scenery_lines: self.scenery_lines.into_iter().map(|x| x.build()).collect(),
            layers: self.layers.into_iter().map(|x| x.build()).collect(),
            layer_folders: self.layer_folders.into_iter().map(|x| x.build()).collect(),
            riders: self.riders.into_iter().map(|x| x.build()).collect(),
        }
    }
}

impl From<Track> for TrackBuilder {
    fn from(track: Track) -> Self {
        TrackBuilder {
            grid_version: track.grid_version,
            title: track.title,
            artist: track.artist,
            description: track.description,
            duration: track.duration,
            audio_filename: track.audio_filename,
            audio_offset: track.audio_offset,
            standard_lines: track.standard_lines.into_iter().map(|x| x.into()).collect(),
            scenery_lines: track.scenery_lines.into_iter().map(|x| x.into()).collect(),
            layers: track.layers.into_iter().map(|x| x.into()).collect(),
            layer_folders: track.layer_folders.into_iter().map(|x| x.into()).collect(),
            riders: track.riders.into_iter().map(|x| x.into()).collect(),
        }
    }
}
