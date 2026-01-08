use geometry::Line;

#[derive(PartialEq, Debug)]
pub struct StandardLine {
    endpoints: Line,
    flipped: bool,
    left_extension: bool,
    right_extension: bool,
    height: f64,
    multiplier: f64,
}

impl StandardLine {
    pub fn new(endpoints: Line) -> Self {
        Self {
            endpoints,
            flipped: false,
            left_extension: false,
            right_extension: false,
            height: 10.0,
            multiplier: 0.0,
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

    pub fn flipped(&self) -> bool {
        self.flipped
    }

    pub fn set_flipped(&mut self, flipped: bool) {
        self.flipped = flipped;
    }

    pub fn left_extension(&self) -> bool {
        self.left_extension
    }

    pub fn set_left_extension(&mut self, left_extension: bool) {
        self.left_extension = left_extension;
    }

    pub fn right_extension(&self) -> bool {
        self.right_extension
    }

    pub fn set_right_extension(&mut self, right_extension: bool) {
        self.right_extension = right_extension;
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn set_height(&mut self, height: f64) {
        self.height = height;
    }

    pub fn multiplier(&self) -> f64 {
        self.multiplier
    }

    pub fn set_multiplier(&mut self, multiplier: f64) {
        self.multiplier = multiplier;
    }
}
