mod bone;
mod entity;
mod entity_state;
mod entity_template;
mod joint;
mod mount_phase;
mod point;
mod remount_version;

use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    hash::Hash,
};

pub use bone::{EntityBone, EntityBoneBuilder};
pub(crate) use entity::Entity;
pub(crate) use entity_state::EntityPointState;
pub use entity_state::EntityState;
pub use entity_template::{
    EntityBoneId, EntityJointId, EntityPointId, EntityTemplate, EntityTemplateBuilder,
};
pub use joint::{EntityJoint, EntityJointBuilder};
pub use mount_phase::MountPhase;
pub use point::{EntityPoint, EntityPointBuilder};
pub use remount_version::RemountVersion;
use vector2d::Vector2Df;

use crate::{PhysicsMoment, line_registry::LineRegistry};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityTemplateId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityId(usize);

pub(crate) struct EntityRegistry {
    // TODO Should these be slot maps? How do we verify order of insertion impacting order of remount processing?
    entity_templates: HashMap<EntityTemplateId, EntityTemplate>,
    entities: BTreeMap<EntityId, Entity>,
    latest_synced_frame: u32,
}

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

    pub(crate) fn create_entity(&mut self, template_id: EntityTemplateId) -> EntityId {
        self.clear_cache();
        let template = self.entity_templates.get_mut(&template_id).unwrap();
        let entity = Entity::new(template_id, template);
        let id = EntityId(self.entities.len());
        self.entities.insert(id, entity);
        id
    }

    pub(crate) fn get_entity_initial_offset(&self, entity_id: EntityId) -> Vector2Df {
        self.entities.get(&entity_id).unwrap().initial_offset()
    }

    pub(crate) fn set_entity_initial_offset(&mut self, entity_id: EntityId, offset: Vector2Df) {
        self.clear_cache();
        let entity = self.entities.get_mut(&entity_id).unwrap();
        let template = self.entity_templates.get(&entity.template_id()).unwrap();
        entity.set_initial_offset(offset, template)
    }

    pub(crate) fn get_entity_initial_velocity(&self, entity_id: EntityId) -> Vector2Df {
        self.entities.get(&entity_id).unwrap().initial_velocity()
    }

    pub(crate) fn set_entity_initial_velocity(&mut self, entity_id: EntityId, velocity: Vector2Df) {
        self.clear_cache();
        let entity = self.entities.get_mut(&entity_id).unwrap();
        let template = self.entity_templates.get(&entity.template_id()).unwrap();
        entity.set_initial_velocity(velocity, template)
    }

    pub(crate) fn remove_entity(&mut self, entity_id: EntityId) {
        self.clear_cache();
        self.entities.remove(&entity_id);
    }

    pub(crate) fn clear_cache(&mut self) {
        self.latest_synced_frame = 0;
        for entity in self.entities.values_mut() {
            entity.truncate_cache(0);
        }
    }

    // This is a pretty delicate function that manages entity states and cache
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
            let mut state_index = 0;
            let mut dismounts = VecDeque::new();

            for entity in self.entities.values() {
                let template = self.entity_templates.get(&entity.template_id()).unwrap();

                let state = &mut entity_states[state_index];

                let dismounted = entity.process_frame(state, template, line_registry);

                dismounts.push_back(dismounted);

                state_index += 1;
            }

            state_index = 0;

            for entity in self.entities.values() {
                let template = self.entity_templates.get(&entity.template_id()).unwrap();

                let mut state = entity_states[state_index].clone();

                let dismounted = dismounts.pop_front().is_some_and(|d| d);

                // TODO entity_states is all skeletons and may not match template
                if !dismounted {
                    entity.process_mount_phase(&mut state, template, &mut entity_states);
                }

                entity_states[state_index] = state;
                state_index += 1;
            }

            state_index = 0;

            for entity in self.entities.values_mut() {
                let state = entity_states[state_index].clone();
                entity.push_to_cache(state);
                state_index += 1;
            }

            self.latest_synced_frame += 1;
        }

        entity_states
    }
}
