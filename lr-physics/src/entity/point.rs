use geometry::Point;
use vector2d::Vector2Df;

pub struct EntityPointState {
    position: Point,
    velocity: Vector2Df,
    previous_position: Point,
}

impl Clone for EntityPointState {
    fn clone(&self) -> Self {
        Self {
            position: self.position.clone(),
            velocity: self.velocity.clone(),
            previous_position: self.previous_position.clone(),
        }
    }
}

pub struct EntityPoint {
    state: EntityPointState,
    contact: bool,
    contact_friction: f64,
    air_friction: f64,
}

pub struct EntityPointBuilder {
    initial_position: Option<Point>,
    contact: bool,
    contact_friction: Option<f64>,
    air_friction: Option<f64>,
}

#[derive(Debug, Clone)]
pub enum EntityPointBuilderError {
    MissingInitialPosition,
}

impl EntityPointBuilder {
    pub fn new() -> EntityPointBuilder {
        EntityPointBuilder {
            initial_position: None,
            contact: false,
            contact_friction: None,
            air_friction: None,
        }
    }

    pub fn initial_position(&mut self, position: Point) -> &mut Self {
        self.initial_position = Some(position);
        self
    }

    pub fn contact(&mut self) -> &mut Self {
        self.contact = true;
        self
    }

    pub fn contact_friction(&mut self, friction: f64) -> &mut Self {
        self.contact_friction = Some(friction);
        self
    }

    pub fn air_friction(&mut self, friction: f64) -> &mut Self {
        self.air_friction = Some(friction);
        self
    }

    pub fn build(&self) -> Result<EntityPoint, EntityPointBuilderError> {
        if let Some(initial_position) = self.initial_position {
            Ok(EntityPoint {
                state: EntityPointState {
                    position: initial_position,
                    velocity: Vector2Df::zero(),
                    previous_position: initial_position,
                },
                contact: self.contact,
                contact_friction: self.contact_friction.unwrap_or(0.0),
                air_friction: self.air_friction.unwrap_or(0.0),
            })
        } else {
            Err(EntityPointBuilderError::MissingInitialPosition)
        }
    }
}

impl EntityPoint {
    pub fn update(
        &mut self,
        new_position: Point,
        new_velocity: Vector2Df,
        new_previous_position: Point,
    ) {
        self.state.position = new_position;
        self.state.velocity = new_velocity;
        self.state.previous_position = new_previous_position;
    }

    pub fn position(&self) -> Point {
        self.state.position
    }

    pub fn velocity(&self) -> Vector2Df {
        self.state.velocity
    }

    pub fn previous_position(&self) -> Point {
        self.state.previous_position
    }

    pub fn friction(&self) -> f64 {
        self.contact_friction
    }

    pub fn air_friction(&self) -> f64 {
        self.air_friction
    }

    pub fn is_contact(&self) -> bool {
        self.contact
    }
}
