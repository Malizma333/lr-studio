mod groups;
mod line_type;
mod primitives;

pub use groups::{layer, line, metadata, rider, trigger};
pub use line_type::LineType;
pub use primitives::{GridVersion, RGBColor, RemountVersion, Vec2};

use crate::track::metadata::{Metadata, MetadataBuilder};

pub struct Track {
    metadata: Metadata,
    // line_group: LineGroup,
    // layer_group: Option<LayerGroup>,
    // rider_group: Option<RiderGroup>,
    // background_color_group: Option<BackgroundColorGroup>,
    // line_color_group: Option<LineColorGroup>,
    // camera_zoom_group: Option<CameraZoomGroup>,
    // legacy_camera_zoom_group: Option<LegacyCameraZoomGroup>,
}

impl Track {
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

struct TrackBuilder {
    metadata: MetadataBuilder,
}

impl TrackBuilder {
    pub fn new(grid_version: GridVersion) -> Self {
        TrackBuilder {
            metadata: MetadataBuilder::new(grid_version),
        }
    }

    pub fn metadata(&mut self) -> &mut MetadataBuilder {
        &mut self.metadata
    }

    pub fn build(&self) -> Track {
        Track {
            metadata: self.metadata.build(),
        }
    }
}
