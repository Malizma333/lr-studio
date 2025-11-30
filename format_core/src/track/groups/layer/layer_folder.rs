pub struct LayerFolder {
    id: u32,
    index: usize,
    name: Option<String>,
    visible: Option<bool>,
    editable: Option<bool>,
    size: u32,
}

impl LayerFolder {
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

    pub fn size(&self) -> u32 {
        self.size
    }
}

pub struct LayerFolderBuilder {
    id: u32,
    index: usize,
    name: Option<String>,
    visible: Option<bool>,
    editable: Option<bool>,
    size: u32,
}

impl LayerFolderBuilder {
    pub fn new(id: u32, index: usize, size: u32) -> Self {
        LayerFolderBuilder {
            id,
            index,
            name: None,
            visible: None,
            editable: None,
            size,
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

    pub fn size(&mut self, size: u32) -> &mut Self {
        self.size = size;
        self
    }

    pub fn build(&self) -> LayerFolder {
        LayerFolder {
            id: self.id,
            index: self.index,
            name: self.name.clone(),
            editable: self.editable,
            visible: self.visible,
            size: self.size,
        }
    }
}
