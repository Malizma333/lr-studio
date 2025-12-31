use vector2d::Vector2Df;

pub struct SceneryLine {
    endpoints: (Vector2Df, Vector2Df),
    width: Option<f64>,
}

impl SceneryLine {
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

    pub fn width(&self) -> Option<f64> {
        self.width
    }
}

pub struct SceneryLineBuilder {
    endpoints: (Vector2Df, Vector2Df),
    width: Option<f64>,
}

impl SceneryLineBuilder {
    pub fn new(endpoints: (Vector2Df, Vector2Df)) -> Self {
        SceneryLineBuilder {
            endpoints,
            width: None,
        }
    }

    pub fn width(&mut self, width: f64) -> &mut Self {
        self.width = Some(width);
        self
    }

    pub(crate) fn build(&self) -> SceneryLine {
        SceneryLine {
            endpoints: self.endpoints,
            width: self.width,
        }
    }
}
