use vector2d::Vector2Df;

pub struct StandardLine {
    id: u32,
    endpoints: (Vector2Df, Vector2Df),
    flipped: bool,
    left_extension: bool,
    right_extension: bool,
}

impl StandardLine {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn x1(&self) -> f64 {
        self.endpoints.0.x
    }

    pub fn y1(&self) -> f64 {
        self.endpoints.0.y
    }

    pub fn x2(&self) -> f64 {
        self.endpoints.1.x
    }

    pub fn y2(&self) -> f64 {
        self.endpoints.1.y
    }

    pub fn flipped(&self) -> bool {
        self.flipped
    }

    pub fn left_extension(&self) -> bool {
        self.left_extension
    }

    pub fn right_extension(&self) -> bool {
        self.right_extension
    }
}

pub struct StandardLineBuilder {
    id: u32,
    endpoints: (Vector2Df, Vector2Df),
    flipped: bool,
    left_extension: bool,
    right_extension: bool,
}

impl StandardLineBuilder {
    pub fn new(id: u32, endpoints: (Vector2Df, Vector2Df)) -> Self {
        StandardLineBuilder {
            id,
            endpoints,
            flipped: false,
            left_extension: false,
            right_extension: false,
        }
    }

    pub fn flipped(&mut self, flipped: bool) -> &mut Self {
        self.flipped = flipped;
        self
    }

    pub fn left_extension(&mut self, left_extension: bool) -> &mut Self {
        self.left_extension = left_extension;
        self
    }

    pub fn right_extension(&mut self, right_extension: bool) -> &mut Self {
        self.right_extension = right_extension;
        self
    }

    pub fn build(&self) -> StandardLine {
        StandardLine {
            id: self.id,
            endpoints: self.endpoints,
            flipped: self.flipped,
            left_extension: self.left_extension,
            right_extension: self.right_extension,
        }
    }
}
