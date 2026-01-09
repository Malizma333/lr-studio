use color::RGBColor;

#[derive(Debug, PartialEq)]
pub struct Layer {
    id: u32,
    name: Option<String>,
    visible: Option<bool>,
    editable: Option<bool>,
    color: Option<RGBColor>,
    folder_id: Option<u32>,
}

impl Layer {
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

    pub fn color(&self) -> Option<RGBColor> {
        self.color
    }

    pub fn folder_id(&self) -> Option<u32> {
        self.folder_id
    }
}

pub struct LayerBuilder {
    id: u32,
    name: Option<String>,
    visible: Option<bool>,
    editable: Option<bool>,
    color: Option<RGBColor>,
    folder_id: Option<u32>,
}

impl LayerBuilder {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            name: None,
            visible: None,
            editable: None,
            color: None,
            folder_id: None,
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

    pub fn color(&mut self, color: RGBColor) -> &mut Self {
        self.color = Some(color);
        self
    }

    pub fn folder_id(&mut self, parent_id: u32) -> &mut Self {
        self.folder_id = Some(parent_id);
        self
    }

    pub fn build(self) -> Layer {
        Layer {
            id: self.id,
            name: self.name,
            visible: self.visible,
            editable: self.editable,
            color: self.color,
            folder_id: self.folder_id,
        }
    }
}

impl From<Layer> for LayerBuilder {
    fn from(layer: Layer) -> Self {
        LayerBuilder {
            id: layer.id,
            name: layer.name,
            visible: layer.visible,
            editable: layer.editable,
            color: layer.color,
            folder_id: layer.folder_id,
        }
    }
}
