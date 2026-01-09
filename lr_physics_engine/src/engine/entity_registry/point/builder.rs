use geometry::Point;

use crate::engine::entity_registry::{
    EntityPointTemplateId, point::template::EntityPointTemplate,
    skeleton::builder::EntitySkeletonBuilder,
};

pub struct EntityPointBuilder<'a> {
    skeleton: EntitySkeletonBuilder<'a>,
    initial_position: Point,
    is_contact: bool,
    contact_friction: Option<f64>,
    air_friction: Option<f64>,
}

impl<'a> EntityPointBuilder<'a> {
    pub(crate) fn new(
        skeleton: EntitySkeletonBuilder<'a>,
        initial_position: Point,
    ) -> EntityPointBuilder<'a> {
        Self {
            skeleton,
            initial_position,
            is_contact: false,
            contact_friction: None,
            air_friction: None,
        }
    }

    pub fn is_contact(mut self) -> Self {
        self.is_contact = true;
        self
    }

    pub fn contact_friction(mut self, friction: f64) -> Self {
        self.contact_friction = Some(friction);
        self
    }

    pub fn air_friction(mut self, friction: f64) -> Self {
        self.air_friction = Some(friction);
        self
    }

    pub fn build(self) -> (EntitySkeletonBuilder<'a>, EntityPointTemplateId) {
        let template = EntityPointTemplate {
            initial_position: self.initial_position,
            is_contact: self.is_contact,
            contact_friction: self.contact_friction.unwrap_or(0.0),
            air_friction: self.air_friction.unwrap_or(0.0),
        };
        self.skeleton.add_point(template)
    }
}
