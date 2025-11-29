use crate::track::Vec2;

pub struct SceneryLine {
    id: u32,
    endpoints: (Vec2, Vec2),
    width: Option<f64>,
}

impl SceneryLine {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn x1(&self) -> f64 {
        self.endpoints.0.x()
    }

    pub fn y1(&self) -> f64 {
        self.endpoints.0.y()
    }

    pub fn x2(&self) -> f64 {
        self.endpoints.1.x()
    }

    pub fn y2(&self) -> f64 {
        self.endpoints.1.y()
    }

    pub fn width(&self) -> Option<f64> {
        self.width
    }
}

pub struct SceneryLineBuilder {
    id: u32,
    endpoints: (Vec2, Vec2),
    width: Option<f64>,
}

impl SceneryLineBuilder {
    pub fn new(id: u32, endpoints: (Vec2, Vec2)) -> Self {
        SceneryLineBuilder {
            id,
            endpoints,
            width: None,
        }
    }

    pub fn width(&mut self, width: f64) -> &mut Self {
        self.width = Some(width);
        self
    }

    pub fn build(&self) -> SceneryLine {
        SceneryLine {
            id: self.id,
            endpoints: self.endpoints,
            width: self.width,
        }
    }
}
