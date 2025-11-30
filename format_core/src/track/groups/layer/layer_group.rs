use std::vec;

use crate::track::layer::{
    layer_base::{Layer, LayerBuilder},
    layer_folder::{LayerFolder, LayerFolderBuilder},
};

pub struct LayerGroup {
    layers: Vec<Layer>,
    layer_folders: Option<Vec<LayerFolder>>,
}

impl LayerGroup {
    pub fn layers(&self) -> &Vec<Layer> {
        &self.layers
    }

    pub fn layer_folders(&self) -> &Option<Vec<LayerFolder>> {
        &self.layer_folders
    }
}

pub struct LayerGroupBuilder {
    layers: Vec<LayerBuilder>,
    layer_folders: Option<Vec<LayerFolderBuilder>>,
}

impl LayerGroupBuilder {
    pub fn new() -> Self {
        LayerGroupBuilder {
            layers: Vec::new(),
            layer_folders: None,
        }
    }

    pub fn add_layer(&mut self, id: u32, index: usize) -> &mut LayerBuilder {
        self.layers.push(LayerBuilder::new(id, index));
        self.layers.last_mut().unwrap()
    }

    pub fn layers(&mut self) -> &mut Vec<LayerBuilder> {
        &mut self.layers
    }

    pub fn add_layer_folder(
        &mut self,
        id: u32,
        index: usize,
        size: u32,
    ) -> &mut LayerFolderBuilder {
        let layer_folders = self.layer_folders.get_or_insert_with(Vec::new);
        layer_folders.push(LayerFolderBuilder::new(id, index, size));
        layer_folders.last_mut().unwrap()
    }

    pub fn layer_folders(&mut self) -> &mut Option<Vec<LayerFolderBuilder>> {
        &mut self.layer_folders
    }

    pub fn build(&self) -> Option<LayerGroup> {
        let mut layers: Vec<Layer> = vec![];
        let mut layer_folders: Option<Vec<LayerFolder>> = None;

        for layer_builder in &self.layers {
            let layer = layer_builder.build();
            layers.push(layer);
        }

        if let Some(layer_folder_builders) = &self.layer_folders {
            let mut some_layer_folders = vec![];
            for layer_folder_builder in layer_folder_builders {
                let layer_folder = layer_folder_builder.build();
                some_layer_folders.push(layer_folder);
            }
            layer_folders = Some(some_layer_folders);
        }

        if layers.len() == 0
            && layer_folders
                .as_ref()
                .is_none_or(|layer_folders| layer_folders.len() == 0)
        {
            None
        } else {
            Some(LayerGroup {
                layers,
                layer_folders,
            })
        }
    }
}
