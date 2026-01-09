use geometry::Line;

#[derive(PartialEq, Debug)]
pub struct SceneryLine {
    endpoints: Line,
    width: Option<f64>,
}

impl SceneryLine {
    pub fn x0(&self) -> f64 {
        self.endpoints.p0().x()
    }

    pub fn y0(&self) -> f64 {
        self.endpoints.p0().y()
    }

    pub fn x1(&self) -> f64 {
        self.endpoints.p1().x()
    }

    pub fn y1(&self) -> f64 {
        self.endpoints.p1().y()
    }

    pub fn width(&self) -> Option<f64> {
        self.width
    }
}

pub struct SceneryLineBuilder {
    endpoints: Line,
    width: Option<f64>,
}

impl SceneryLineBuilder {
    pub fn new(endpoints: Line) -> Self {
        Self {
            endpoints,
            width: None,
        }
    }

    pub fn x0(&mut self, x: f64) -> &mut Self {
        self.endpoints.p0_mut().set_x(x);
        self
    }

    pub fn y0(&mut self, y: f64) -> &mut Self {
        self.endpoints.p0_mut().set_y(y);
        self
    }

    pub fn x1(&mut self, x: f64) -> &mut Self {
        self.endpoints.p1_mut().set_x(x);
        self
    }

    pub fn y1(&mut self, y: f64) -> &mut Self {
        self.endpoints.p1_mut().set_y(y);
        self
    }

    pub fn width(&mut self, width: f64) -> &mut Self {
        self.width = Some(width);
        self
    }

    pub fn build(self) -> SceneryLine {
        SceneryLine {
            endpoints: self.endpoints,
            width: self.width,
        }
    }
}

impl From<SceneryLine> for SceneryLineBuilder {
    fn from(scenery_line: SceneryLine) -> Self {
        SceneryLineBuilder {
            endpoints: scenery_line.endpoints,
            width: scenery_line.width,
        }
    }
}
