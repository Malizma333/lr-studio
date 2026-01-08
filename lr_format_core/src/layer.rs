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

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn visible(&self) -> Option<bool> {
        self.visible
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = Some(visible);
    }

    pub fn editable(&self) -> Option<bool> {
        self.editable
    }

    pub fn set_editable(&mut self, editable: bool) {
        self.editable = Some(editable);
    }

    pub fn color(&self) -> Option<RGBColor> {
        self.color
    }

    pub fn set_color(&mut self, color: RGBColor) {
        self.color = Some(color);
    }

    pub fn folder_id(&self) -> Option<u32> {
        self.folder_id
    }

    pub fn set_folder_id(&mut self, parent_id: u32) {
        self.folder_id = Some(parent_id);
    }
}
