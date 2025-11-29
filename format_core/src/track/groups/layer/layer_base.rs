pub struct Layer {
    id: u32,
    index: usize,
    name: Option<String>,
    visible: Option<bool>,
    editable: Option<bool>,
    folder_id: Option<u32>,
}

impl Layer {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn index(&self) -> usize {
        self.index
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

    pub fn folder_id(&self) -> Option<u32> {
        self.folder_id
    }
}

pub struct LayerBuilder {
    id: u32,
    index: usize,
    name: Option<String>,
    visible: Option<bool>,
    editable: Option<bool>,
    folder_id: Option<u32>,
}

impl LayerBuilder {
    pub fn new(id: u32, index: usize) -> Self {
        LayerBuilder {
            id,
            index,
            name: None,
            visible: None,
            editable: None,
            folder_id: None,
        }
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

    pub fn folder_id(&mut self, id: u32) -> &mut Self {
        self.folder_id = Some(id);
        self
    }

    pub fn build(&self) -> Layer {
        Layer {
            id: self.id,
            index: self.index,
            name: self.name.clone(),
            editable: self.editable,
            visible: self.visible,
            folder_id: self.folder_id,
        }
    }
}
