use crate::{
    entity_registry::{EntityState, EntityTemplate, EntityTemplateId, MountPhase, RemountVersion},
    line_registry::LineRegistry,
};
use vector2d::Vector2Df;

struct InitialProps {
    offset: Vector2Df,
    velocity: Vector2Df,
}

pub(crate) struct Entity {
    cached_states: Vec<EntityState>,
    initial_state: EntityState,
    initial_props: InitialProps,
    associated_template_id: EntityTemplateId,
}

impl Entity {
    pub(super) fn new(template_id: EntityTemplateId, template: &EntityTemplate) -> Self {
        let mut entity = Self {
            cached_states: Vec::new(),
            initial_props: InitialProps {
                offset: Vector2Df::zero(),
                velocity: Vector2Df::zero(),
            },
            initial_state: EntityState::new(template, Vector2Df::zero(), Vector2Df::zero()),
            associated_template_id: template_id,
        };
        entity.regenerate_initial_state(template);
        entity
    }

    pub(super) fn cached_states(&self) -> &Vec<EntityState> {
        &self.cached_states
    }

    pub(super) fn push_to_cache(&mut self, state: EntityState) {
        self.cached_states.push(state)
    }

    pub(super) fn truncate_cache(&mut self, size: u32) {
        self.cached_states.truncate(size as usize);
    }

    pub(super) fn initial_state(&self) -> &EntityState {
        &self.initial_state
    }

    pub(super) fn initial_offset(&self) -> Vector2Df {
        self.initial_props.offset
    }

    pub(super) fn set_initial_offset(&mut self, offset: Vector2Df, template: &EntityTemplate) {
        self.initial_props.offset = offset;
        self.regenerate_initial_state(template);
    }

    pub(super) fn initial_velocity(&self) -> Vector2Df {
        self.initial_props.velocity
    }

    pub(super) fn set_initial_velocity(&mut self, velocity: Vector2Df, template: &EntityTemplate) {
        self.initial_props.velocity = velocity;
        self.regenerate_initial_state(template);
    }

    pub(super) fn template_id(&self) -> EntityTemplateId {
        self.associated_template_id
    }

    fn regenerate_initial_state(&mut self, template: &EntityTemplate) {
        self.initial_state =
            EntityState::new(template, self.initial_offset(), self.initial_velocity());
        self.cached_states.clear();
    }

    // This is the main physics loop that transforms an entity state
    // Returns whether the rider dismounted
    pub(super) fn process_frame(
        &self,
        state: &mut EntityState,
        template: &EntityTemplate,
        line_registry: &LineRegistry,
    ) -> bool {
        let mut dismounted = false;

        for (point_id, point) in template.points() {
            const GRAVITY_MULTIPLIER: f64 = 0.175;
            let gravity = Vector2Df::down() * GRAVITY_MULTIPLIER;

            let point_state = state.point_state_mut(point_id);
            let computed_velocity = point_state
                .position()
                .vector_from(point_state.computed_previous_position());
            let new_velocity =
                computed_velocity * (1.0 - point.air_friction()) + gravity.flipped_vertical();
            let new_position = point_state.position().translated_by(new_velocity);
            point_state.update(
                Some(new_position),
                Some(new_velocity),
                Some(point_state.position()),
            );
        }

        let initial_mount_phase = state.mount_phase();

        const MAX_ITERATION: u8 = 6;
        for _ in 0..MAX_ITERATION {
            for bone in template.bones().values() {
                if !bone.is_flutter() {
                    let point_states = (
                        state.point_state(&bone.point_ids().0),
                        state.point_state(&bone.point_ids().1),
                    );

                    let mount_phase = match template.remount_version() {
                        RemountVersion::LRA => initial_mount_phase,
                        _ => state.mount_phase(),
                    };

                    if !bone.is_breakable() {
                        let adjusted = bone.get_adjusted(point_states, mount_phase.is_remounting());
                        state.point_state_mut(&bone.point_ids().0).update(
                            Some(adjusted.0),
                            None,
                            None,
                        );
                        state.point_state_mut(&bone.point_ids().1).update(
                            Some(adjusted.1),
                            None,
                            None,
                        );
                    } else if (mount_phase.is_remounting() || mount_phase.is_mounted())
                        && !dismounted
                    {
                        if bone.get_intact(point_states, mount_phase.is_remounting()) {
                            let adjusted =
                                bone.get_adjusted(point_states, mount_phase.is_remounting());
                            state.point_state_mut(&bone.point_ids().0).update(
                                Some(adjusted.0),
                                None,
                                None,
                            );
                            state.point_state_mut(&bone.point_ids().1).update(
                                Some(adjusted.1),
                                None,
                                None,
                            );
                        } else {
                            dismounted = true;

                            let next_mount_phase = match template.remount_version() {
                                RemountVersion::None => MountPhase::Dismounted {
                                    frames_until_remounting: 0,
                                },
                                _ => {
                                    if mount_phase.is_mounted() {
                                        MountPhase::Dismounting {
                                            frames_until_dismounted: template.dismounted_timer(),
                                        }
                                    } else if mount_phase.is_remounting() {
                                        MountPhase::Dismounted {
                                            frames_until_remounting: template.remounting_timer(),
                                        }
                                    } else {
                                        mount_phase
                                    }
                                }
                            };

                            state.skeleton_state_mut().set_mount_phase(next_mount_phase);
                        }
                    }
                }
            }

            for (point_id, point) in template.points() {
                if point.is_contact() {
                    let point_state = state.point_state_mut(&point_id);
                    for line in line_registry.lines_near_point(point_state.position()) {
                        if let Some((new_position, new_computed_previous_position)) =
                            line.check_interaction(point, point_state)
                        {
                            point_state.update(
                                Some(new_position),
                                None,
                                Some(new_computed_previous_position),
                            );
                        }
                    }
                }
            }
        }

        for bone in template.bones().values() {
            if bone.is_flutter() {
                let point_states = (
                    state.point_state(&bone.point_ids().0),
                    state.point_state(&bone.point_ids().1),
                );
                let mount_phase = state.skeleton_state().mount_phase();
                let adjusted = bone.get_adjusted(point_states, mount_phase.is_remounting());
                state
                    .point_state_mut(&bone.point_ids().0)
                    .update(Some(adjusted.0), None, None);
                state
                    .point_state_mut(&bone.point_ids().1)
                    .update(Some(adjusted.1), None, None);
            }
        }

        let mount_phase = state.skeleton_state().mount_phase();

        if mount_phase.is_mounted() || mount_phase.is_remounting() {
            for joint in template.joints().values() {
                if joint.is_mount() && template.get_joint_should_break(&state, joint) && !dismounted
                {
                    dismounted = true;

                    let next_mount_phase = match template.remount_version() {
                        RemountVersion::None => MountPhase::Dismounted {
                            frames_until_remounting: 0,
                        },
                        _ => {
                            if mount_phase.is_mounted() {
                                MountPhase::Dismounting {
                                    frames_until_dismounted: template.dismounted_timer(),
                                }
                            } else {
                                MountPhase::Dismounted {
                                    frames_until_remounting: template.remounting_timer(),
                                }
                            }
                        }
                    };

                    state.skeleton_state_mut().set_mount_phase(next_mount_phase);

                    if let RemountVersion::LRA = template.remount_version() {
                        state.skeleton_state_mut().set_sled_intact(false)
                    }
                }
            }
        }

        let mount_phase = state.skeleton_state().mount_phase();
        let sled_intact = state.skeleton_state().sled_intact();

        let sled_break_version = match template.remount_version() {
            RemountVersion::None | RemountVersion::ComV2 => true,
            _ => false,
        };

        if mount_phase.is_mounted() || mount_phase.is_remounting() || sled_break_version {
            for joint in template.joints().values() {
                if !joint.is_mount()
                    && template.get_joint_should_break(&state, joint)
                    && sled_intact
                {
                    state.skeleton_state_mut().set_sled_intact(false);
                }
            }
        }

        dismounted
    }

    // This retrieves the next mount phase
    pub(super) fn process_mount_phase(
        &self,
        state: &mut EntityState,
        template: &EntityTemplate,
        other_states: &mut Vec<EntityState>,
    ) {
        let current_mount_phase = state.skeleton_state().mount_phase();
        let sled_intact = state.skeleton_state().sled_intact();

        let next_mount_phase = match template.remount_version() {
            RemountVersion::LRA => {
                if !sled_intact {
                    MountPhase::Dismounted {
                        frames_until_remounting: 0,
                    }
                } else {
                    match current_mount_phase {
                        MountPhase::Dismounting {
                            frames_until_dismounted,
                        } => {
                            if frames_until_dismounted == 0 {
                                MountPhase::Dismounted {
                                    frames_until_remounting: template.remounting_timer(),
                                }
                            } else {
                                MountPhase::Dismounting {
                                    frames_until_dismounted: frames_until_dismounted
                                        .saturating_sub(1),
                                }
                            }
                        }
                        MountPhase::Dismounted {
                            frames_until_remounting,
                        } => {
                            let mut can_swap = false;

                            for other_state in other_states {
                                if template.can_swap_sleds(state, other_state) {
                                    can_swap = true;
                                    break;
                                }
                            }

                            if can_swap {
                                if frames_until_remounting == 0 {
                                    MountPhase::Remounting {
                                        frames_until_mounted: template.mounted_timer(),
                                    }
                                } else {
                                    MountPhase::Dismounted {
                                        frames_until_remounting: frames_until_remounting
                                            .saturating_sub(1),
                                    }
                                }
                            } else {
                                MountPhase::Dismounted {
                                    frames_until_remounting: template.remounting_timer(),
                                }
                            }
                        }
                        MountPhase::Remounting {
                            frames_until_mounted,
                        } => {
                            if template.skeleton_can_enter_phase(&state, false) {
                                if frames_until_mounted == 0 {
                                    MountPhase::Mounted
                                } else {
                                    MountPhase::Remounting {
                                        frames_until_mounted: frames_until_mounted
                                            .saturating_sub(1),
                                    }
                                }
                            } else {
                                MountPhase::Remounting {
                                    frames_until_mounted: template.mounted_timer(),
                                }
                            }
                        }
                        MountPhase::Mounted => MountPhase::Mounted,
                    }
                }
            }
            RemountVersion::ComV1 | RemountVersion::ComV2 => match current_mount_phase {
                MountPhase::Dismounting {
                    frames_until_dismounted,
                } => {
                    let next_timer = frames_until_dismounted.saturating_sub(1);
                    if next_timer == 0 {
                        MountPhase::Dismounted {
                            frames_until_remounting: template.remounting_timer(),
                        }
                    } else {
                        MountPhase::Dismounting {
                            frames_until_dismounted: next_timer,
                        }
                    }
                }
                MountPhase::Dismounted {
                    frames_until_remounting,
                } => {
                    let mut can_swap = false;

                    for other_state in other_states {
                        if template.can_swap_sleds(state, other_state) {
                            can_swap = true;
                            break;
                        }
                    }

                    let next_timer = if can_swap {
                        frames_until_remounting.saturating_sub(1)
                    } else {
                        template.remounting_timer()
                    };

                    if next_timer == 0 {
                        MountPhase::Remounting {
                            frames_until_mounted: template.mounted_timer(),
                        }
                    } else {
                        MountPhase::Dismounted {
                            frames_until_remounting: next_timer,
                        }
                    }
                }
                MountPhase::Remounting {
                    frames_until_mounted,
                } => {
                    let next_timer = if template.skeleton_can_enter_phase(&state, false) {
                        frames_until_mounted.saturating_sub(1)
                    } else {
                        template.mounted_timer()
                    };

                    if next_timer == 0 {
                        MountPhase::Mounted
                    } else {
                        MountPhase::Remounting {
                            frames_until_mounted: next_timer,
                        }
                    }
                }
                MountPhase::Mounted => MountPhase::Mounted,
            },
            RemountVersion::None => current_mount_phase,
        };

        state.skeleton_state_mut().set_mount_phase(next_mount_phase);
    }
}
