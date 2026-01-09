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

    pub fn endpoints(&self) -> Line {
        self.endpoints
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

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn multiplier(&self) -> f64 {
        self.multiplier
    }
}

pub struct StandardLineBuilder {
    endpoints: Line,
    flipped: bool,
    left_extension: bool,
    right_extension: bool,
    height: f64,
    multiplier: f64,
}

impl StandardLineBuilder {
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

    pub fn endpoints(&mut self, endpoints: Line) -> &mut Self {
        self.endpoints = endpoints;
        self
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

    pub fn height(&mut self, height: f64) -> &mut Self {
        self.height = height;
        self
    }

    pub fn multiplier(&mut self, multiplier: f64) -> &mut Self {
        self.multiplier = multiplier;
        self
    }

    pub fn build(self) -> StandardLine {
        StandardLine {
            endpoints: self.endpoints,
            flipped: self.flipped,
            left_extension: self.left_extension,
            right_extension: self.right_extension,
            height: self.height,
            multiplier: self.multiplier,
        }
    }
}

impl From<StandardLine> for StandardLineBuilder {
    fn from(standard_line: StandardLine) -> Self {
        StandardLineBuilder {
            endpoints: standard_line.endpoints,
            flipped: standard_line.flipped,
            left_extension: standard_line.left_extension,
            right_extension: standard_line.right_extension,
            height: standard_line.height,
            multiplier: standard_line.multiplier,
        }
    }
}
