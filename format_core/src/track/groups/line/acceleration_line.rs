use vector2d::Vector2Df;

pub struct AccelerationLine {
    id: u32,
    endpoints: (Vector2Df, Vector2Df),
    flipped: bool,
    left_extension: bool,
    right_extension: bool,
    multiplier: Option<f64>,
}

impl AccelerationLine {
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

    pub fn multiplier(&self) -> Option<f64> {
        self.multiplier
    }
}

pub struct AccelerationLineBuilder {
    id: u32,
    endpoints: (Vector2Df, Vector2Df),
    flipped: bool,
    left_extension: bool,
    right_extension: bool,
    multiplier: Option<f64>,
}

impl AccelerationLineBuilder {
    pub fn new(id: u32, endpoints: (Vector2Df, Vector2Df)) -> Self {
        AccelerationLineBuilder {
            id,
            endpoints,
            flipped: false,
            left_extension: false,
            right_extension: false,
            multiplier: None,
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

    pub fn multiplier(&mut self, multiplier: f64) -> &mut Self {
        self.multiplier = Some(multiplier);
        self
    }

    pub fn build(&self) -> AccelerationLine {
        AccelerationLine {
            id: self.id,
            endpoints: self.endpoints,
            flipped: self.flipped,
            left_extension: self.left_extension,
            right_extension: self.right_extension,
            multiplier: self.multiplier,
        }
    }
}
