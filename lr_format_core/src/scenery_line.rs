use geometry::Line;

#[derive(PartialEq, Debug)]
pub struct SceneryLine {
    endpoints: Line,
    width: Option<f64>,
}

impl SceneryLine {
    pub fn new(endpoints: Line) -> Self {
        Self {
            endpoints,
            width: None,
        }
    }

    pub fn x0(&self) -> f64 {
        self.endpoints.p0().x()
    }

    pub fn set_x0(&mut self, x: f64) {
        self.endpoints.p0_mut().set_x(x);
    }

    pub fn y0(&self) -> f64 {
        self.endpoints.p0().y()
    }

    pub fn set_y0(&mut self, y: f64) {
        self.endpoints.p0_mut().set_y(y);
    }

    pub fn x1(&self) -> f64 {
        self.endpoints.p1().x()
    }

    pub fn set_x1(&mut self, x: f64) {
        self.endpoints.p1_mut().set_x(x);
    }

    pub fn y1(&self) -> f64 {
        self.endpoints.p1().y()
    }

    pub fn set_y1(&mut self, y: f64) {
        self.endpoints.p1_mut().set_y(y);
    }

    pub fn width(&self) -> Option<f64> {
        self.width
    }

    pub fn set_width(&mut self, width: f64) {
        self.width = Some(width);
    }
}
