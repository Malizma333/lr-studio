mod groups;
mod line_type;
mod primitives;

pub use groups::{layer, line, metadata, rider, trigger};
pub use line_type::LineType;
pub use primitives::{
    BackgroundColorEvent, CameraZoomEvent, FrameBoundsTrigger, GridVersion, LineColorEvent,
    LineHitTrigger, RGBColor, RemountVersion,
};

use crate::track::{
    layer::layer_group::{LayerGroup, LayerGroupBuilder},
    line::line_group::{LineGroup, LineGroupBuilder},
    metadata::{Metadata, MetadataBuilder},
    rider::rider_group::{RiderGroup, RiderGroupBuilder},
    trigger::{
        background_color_group::{BackgroundColorGroup, BackgroundColorGroupBuilder},
        camera_zoom_group::{CameraZoomGroup, CameraZoomGroupBuilder},
        legacy_camera_zoom_group::{LegacyCameraZoomGroup, LegacyCameraZoomGroupBuilder},
        line_color_group::{LineColorGroup, LineColorGroupBuilder},
    },
};

pub struct Track {
    metadata: Metadata,
    line_group: LineGroup,
    layer_group: Option<LayerGroup>,
    rider_group: Option<RiderGroup>,
    background_color_group: Option<BackgroundColorGroup>,
    line_color_group: Option<LineColorGroup>,
    camera_zoom_group: Option<CameraZoomGroup>,
    legacy_camera_zoom_group: Option<LegacyCameraZoomGroup>,
}

impl Track {
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn line_group(&self) -> &LineGroup {
        &self.line_group
    }

    pub fn layer_group(&self) -> &Option<LayerGroup> {
        &self.layer_group
    }

    pub fn rider_group(&self) -> &Option<RiderGroup> {
        &self.rider_group
    }

    pub fn background_color_group(&self) -> &Option<BackgroundColorGroup> {
        &self.background_color_group
    }

    pub fn line_color_group(&self) -> &Option<LineColorGroup> {
        &self.line_color_group
    }

    pub fn camera_zoom_group(&self) -> &Option<CameraZoomGroup> {
        &self.camera_zoom_group
    }

    pub fn legacy_camera_zoom_group(&self) -> &Option<LegacyCameraZoomGroup> {
        &self.legacy_camera_zoom_group
    }
}

pub struct TrackBuilder {
    metadata: MetadataBuilder,
    line_group: LineGroupBuilder,
    layer_group: LayerGroupBuilder,
    rider_group: RiderGroupBuilder,
    background_color_group: BackgroundColorGroupBuilder,
    line_color_group: LineColorGroupBuilder,
    camera_zoom_group: CameraZoomGroupBuilder,
    legacy_camera_zoom_group: LegacyCameraZoomGroupBuilder,
}

impl TrackBuilder {
    pub fn new(grid_version: GridVersion) -> Self {
        TrackBuilder {
            metadata: MetadataBuilder::new(grid_version),
            line_group: LineGroupBuilder::new(),
            layer_group: LayerGroupBuilder::new(),
            rider_group: RiderGroupBuilder::new(),
            background_color_group: BackgroundColorGroupBuilder::new(),
            line_color_group: LineColorGroupBuilder::new(),
            camera_zoom_group: CameraZoomGroupBuilder::new(),
            legacy_camera_zoom_group: LegacyCameraZoomGroupBuilder::new(),
        }
    }

    pub fn metadata(&mut self) -> &mut MetadataBuilder {
        &mut self.metadata
    }

    pub fn line_group(&mut self) -> &mut LineGroupBuilder {
        &mut self.line_group
    }

    pub fn layer_group(&mut self) -> &mut LayerGroupBuilder {
        &mut self.layer_group
    }

    pub fn rider_group(&mut self) -> &mut RiderGroupBuilder {
        &mut self.rider_group
    }

    pub fn background_color_group(&mut self) -> &mut BackgroundColorGroupBuilder {
        &mut self.background_color_group
    }

    pub fn line_color_group(&mut self) -> &mut LineColorGroupBuilder {
        &mut self.line_color_group
    }

    pub fn camera_zoom_group(&mut self) -> &mut CameraZoomGroupBuilder {
        &mut self.camera_zoom_group
    }

    pub fn legacy_camera_zoom_group(&mut self) -> &mut LegacyCameraZoomGroupBuilder {
        &mut self.legacy_camera_zoom_group
    }

    pub fn build(&self) -> Track {
        Track {
            metadata: self.metadata.build(),
            line_group: self.line_group.build(),
            layer_group: self.layer_group.build(),
            rider_group: self.rider_group.build(),
            background_color_group: self.background_color_group.build(),
            line_color_group: self.line_color_group.build(),
            camera_zoom_group: self.camera_zoom_group.build(),
            legacy_camera_zoom_group: self.legacy_camera_zoom_group.build(),
        }
    }
}
