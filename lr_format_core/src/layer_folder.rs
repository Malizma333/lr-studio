#[derive(Debug, PartialEq)]
pub struct LayerFolder {
    id: u32,
    name: Option<String>,
    visible: Option<bool>,
    editable: Option<bool>,
}

impl LayerFolder {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn visible(&self) -> Option<bool> {
        self.visible
    }

    pub fn editable(&self) -> Option<bool> {
        self.editable
    }
}

pub struct LayerFolderBuilder {
    id: u32,
    name: Option<String>,
    visible: Option<bool>,
    editable: Option<bool>,
}

impl LayerFolderBuilder {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            name: None,
            visible: None,
            editable: None,
        }
    }

    pub fn id(&mut self, id: u32) -> &mut Self {
        self.id = id;
        self
    }

    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }

    pub fn visible(&mut self, visible: bool) -> &mut Self {
        self.visible = Some(visible);
        self
    }

    pub fn editable(&mut self, editable: bool) -> &mut Self {
        self.editable = Some(editable);
        self
    }

    pub fn build(self) -> LayerFolder {
        LayerFolder {
            id: self.id,
            name: self.name,
            visible: self.visible,
            editable: self.editable,
        }
    }
}

impl From<LayerFolder> for LayerFolderBuilder {
    fn from(layer_folder: LayerFolder) -> Self {
        LayerFolderBuilder {
            id: layer_folder.id,
            name: layer_folder.name,
            visible: layer_folder.visible,
            editable: layer_folder.editable,
        }
    }
}
