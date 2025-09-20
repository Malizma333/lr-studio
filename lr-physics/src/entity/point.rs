use geometry::Point;
use vector2d::Vector2Df;

struct BaseEntityPoint {
    position: Point,
    velocity: Vector2Df,
    previous_position: Point,
}

impl BaseEntityPoint {
    fn update(
        &mut self,
        new_position: Point,
        new_velocity: Vector2Df,
        new_previous_position: Point,
    ) {
        self.position = new_position;
        self.velocity = new_velocity;
        self.previous_position = new_previous_position;
    }
}

impl Clone for BaseEntityPoint {
    fn clone(&self) -> Self {
        Self {
            position: self.position.clone(),
            velocity: self.velocity.clone(),
            previous_position: self.previous_position.clone(),
        }
    }
}

pub struct ContactPoint {
    base: BaseEntityPoint,
    friction: f64,
}

pub struct FlutterPoint {
    base: BaseEntityPoint,
    air_friction: f64,
}

impl ContactPoint {
    pub fn new(initial_position: Vector2Df, friction: f64) -> ContactPoint {
        ContactPoint {
            base: BaseEntityPoint {
                position: initial_position,
                velocity: Vector2Df::zero(),
                previous_position: initial_position,
            },
            friction: friction,
        }
    }

    pub fn position(&self) -> Point {
        self.base.position
    }

    pub fn velocity(&self) -> Vector2Df {
        self.base.velocity
    }

    pub fn previous_position(&self) -> Point {
        self.base.previous_position
    }

    pub fn update(
        &mut self,
        new_position: Point,
        new_velocity: Vector2Df,
        new_previous_position: Point,
    ) {
        self.base
            .update(new_position, new_velocity, new_previous_position);
    }

    pub fn friction(&self) -> f64 {
        self.friction
    }
}

impl Clone for ContactPoint {
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
            friction: self.friction.clone(),
        }
    }
}

impl FlutterPoint {
    pub fn new(initial_position: Vector2Df, air_friction: f64) -> FlutterPoint {
        FlutterPoint {
            base: BaseEntityPoint {
                position: initial_position,
                velocity: Vector2Df::zero(),
                previous_position: initial_position,
            },
            air_friction,
        }
    }

    pub fn position(&self) -> Point {
        self.base.position
    }

    pub fn velocity(&self) -> Vector2Df {
        self.base.velocity
    }

    pub fn previous_position(&self) -> Point {
        self.base.previous_position
    }

    pub fn update(
        &mut self,
        new_position: Point,
        new_velocity: Vector2Df,
        new_previous_position: Point,
    ) {
        self.base
            .update(new_position, new_velocity, new_previous_position);
    }

    pub fn air_friction(&self) -> f64 {
        self.air_friction
    }
}

impl Clone for FlutterPoint {
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
            air_friction: self.air_friction.clone(),
        }
    }
}
