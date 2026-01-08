#[derive(Debug, PartialEq)]
pub struct LayerFolder {
    id: u32,
    name: Option<String>,
    visible: Option<bool>,
    editable: Option<bool>,
}

impl LayerFolder {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            name: None,
            visible: None,
            editable: None,
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
}
