mod bone;
mod entity;
mod entity_state;
mod entity_template;
mod joint;
mod mount_phase;
mod point;
mod point_state;
mod remount_version;

use std::{
    collections::{BTreeMap, HashMap},
    error, fmt,
    hash::Hash,
    iter::zip,
};

pub use bone::{EntityBone, EntityBoneBuilder};
pub(crate) use entity::Entity;
pub use entity_state::EntityState;
pub use entity_template::{
    EntityBoneId, EntityJointId, EntityPointId, EntityTemplate, EntityTemplateBuilder,
};
pub use joint::{EntityJoint, EntityJointBuilder};
pub use mount_phase::MountPhase;
pub use point::{EntityPoint, EntityPointBuilder};
pub(crate) use point_state::EntityPointState;
pub use remount_version::RemountVersion;
use vector2d::Vector2Df;

use crate::{PhysicsMoment, line_registry::LineRegistry};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityTemplateId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityId(usize);

pub(crate) struct EntityRegistry {
    entity_templates: HashMap<EntityTemplateId, EntityTemplate>,
    entities: BTreeMap<EntityId, Entity>,
    latest_synced_frame: u32,
}

#[derive(Debug)]
pub enum Error {
    EntityNotFound(EntityId),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::EntityNotFound(id) => {
                write!(f, "Entity with id not found: {}", id.0)
            }
        }
    }
}

const EXPECT_TEMPLATE_MSG: &str = "Entity should point to valid template";

impl EntityRegistry {
    pub(crate) fn new() -> Self {
        Self {
            entity_templates: HashMap::new(),
            entities: BTreeMap::new(),
            latest_synced_frame: 0,
        }
    }

    pub(crate) fn add_entity_template(&mut self, template: EntityTemplate) -> EntityTemplateId {
        let id = EntityTemplateId(self.entity_templates.len());
        self.entity_templates.insert(id, template);
        id
    }

    pub(crate) fn create_entity(&mut self, template_id: EntityTemplateId) -> Option<EntityId> {
        self.clear_cache();
        let template = self.entity_templates.get(&template_id);
        template.map(|template| {
            let entity = Entity::new(template_id, template);
            let id = EntityId(self.entities.len());
            self.entities.insert(id, entity);
            id
        })
    }

    pub(crate) fn get_entity_initial_offset(&self, entity_id: EntityId) -> Option<Vector2Df> {
        self.entities
            .get(&entity_id)
            .map(|entity| entity.initial_offset())
    }

    pub(crate) fn set_entity_initial_offset(
        &mut self,
        entity_id: EntityId,
        offset: Vector2Df,
    ) -> Result<(), Error> {
        self.clear_cache();
        let entity = self
            .entities
            .get_mut(&entity_id)
            .ok_or(Error::EntityNotFound(entity_id))?;
        let template = self
            .entity_templates
            .get(&entity.template_id())
            .expect(EXPECT_TEMPLATE_MSG);
        entity.set_initial_offset(offset, template);
        Ok(())
    }

    pub(crate) fn get_entity_initial_velocity(&self, entity_id: EntityId) -> Option<Vector2Df> {
        self.entities
            .get(&entity_id)
            .map(|entity| entity.initial_velocity())
    }

    pub(crate) fn set_entity_initial_velocity(
        &mut self,
        entity_id: EntityId,
        velocity: Vector2Df,
    ) -> Result<(), Error> {
        self.clear_cache();
        let entity = self
            .entities
            .get_mut(&entity_id)
            .ok_or(Error::EntityNotFound(entity_id))?;
        let template = self
            .entity_templates
            .get(&entity.template_id())
            .expect(EXPECT_TEMPLATE_MSG);
        entity.set_initial_velocity(velocity, template);
        Ok(())
    }

    pub(crate) fn remove_entity(&mut self, entity_id: EntityId) -> Result<(), Error> {
        self.clear_cache();
        let removed_entity = self.entities.remove(&entity_id);
        if removed_entity.is_none() {
            Err(Error::EntityNotFound(entity_id))
        } else {
            Ok(())
        }
    }

    pub(crate) fn clear_cache(&mut self) {
        self.latest_synced_frame = 0;
        for entity in self.entities.values_mut() {
            entity.truncate_cache(0);
        }
    }

    // This is a pretty delicate (lots of expects and unwraps) function that manages entity states and cache
    pub(crate) fn compute_frame(
        &mut self,
        frame: u32,
        _moment: PhysicsMoment,
        line_registry: &LineRegistry,
    ) -> Vec<EntityState> {
        let mut entity_states = Vec::new();

        for entity in self.entities.values_mut() {
            entity.truncate_cache(self.latest_synced_frame);
            let state = entity
                .cached_states()
                .last()
                .unwrap_or(entity.initial_state())
                .clone();
            entity_states.push(state);
        }

        while self.latest_synced_frame < frame {
            let mut dismounts = Vec::new();

            for (entity, state) in zip(self.entities.values(), &mut entity_states) {
                let template = self
                    .entity_templates
                    .get(&entity.template_id())
                    .expect(EXPECT_TEMPLATE_MSG);

                let dismounted = state.process_frame(template, line_registry);

                dismounts.push(dismounted);
            }

            for ((entity_index, entity), dismounted) in
                zip(self.entities.values().enumerate(), dismounts)
            {
                let template = self
                    .entity_templates
                    .get(&entity.template_id())
                    .expect(EXPECT_TEMPLATE_MSG);

                let mut state = entity_states
                    .get(entity_index)
                    .expect("Index should be within bounds of entity state array")
                    .clone();

                // TODO this doesn't check same template
                state.process_mount_phase(template, &mut entity_states, &dismounted);

                *entity_states
                    .get_mut(entity_index)
                    .expect("Index should be within bounds of entity state array") = state;
            }

            for (entity, state) in zip(self.entities.values_mut(), entity_states.clone()) {
                entity.push_to_cache(state);
            }

            self.latest_synced_frame += 1;
        }

        entity_states
    }
}
