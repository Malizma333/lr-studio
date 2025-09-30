use crate::engine::entity_registry::EntityRegistryIndex;

const REMOUNT_STRENGTH_FACTOR: f64 = 0.1;
const LRA_REMOUNT_STRENGTH_FACTOR: f64 = 0.5;

enum MountPhase {
    Mounted,
    Dismounting {
        frames_until_dismounted: u32,
    },
    Dismounted {
        // Some still eligible to remount, None means skeleton no longer intact
        frames_until_can_remount: Option<u32>,
    },
    Remounting {
        frames_until_remounted: u32,
    },
}

pub struct EntitySkeletonState {
    // TODO it should be clear that this should only have unstable indices
    mount_bones: Vec<EntityRegistryIndex>,
    mount_joints: Vec<EntityRegistryIndex>,
    mount_phase: MountPhase,
    other_skeleton: Option<EntityRegistryIndex>,
}

impl Clone for EntitySkeletonState {
    fn clone(&self) -> Self {
        let mount_phase_clone = match self.mount_phase {
            MountPhase::Mounted => MountPhase::Mounted,
            MountPhase::Dismounting {
                frames_until_dismounted,
            } => MountPhase::Dismounting {
                frames_until_dismounted,
            },
            MountPhase::Dismounted {
                frames_until_can_remount,
            } => MountPhase::Dismounted {
                frames_until_can_remount,
            },
            MountPhase::Remounting {
                frames_until_remounted,
            } => MountPhase::Remounting {
                frames_until_remounted,
            },
        };

        EntitySkeletonState {
            // TODO: these reference invalid bones + joints
            mount_bones: self.mount_bones.clone(),
            mount_joints: self.mount_joints.clone(),
            mount_phase: mount_phase_clone,
            other_skeleton: self.other_skeleton,
        }
    }
}

pub struct EntitySkeleton {
    points: Vec<EntityRegistryIndex>,
    bones: Vec<EntityRegistryIndex>,
    joints: Vec<EntityRegistryIndex>,
    remount_enabled: bool,
    dismounted_timer: u32,
    remounting_timer: u32,
    remounted_timer: u32,
    state: EntitySkeletonState,
}

impl EntitySkeleton {
    pub fn is_remounting(&self) -> bool {
        matches!(
            self.state.mount_phase,
            MountPhase::Remounting {
                frames_until_remounted: _
            }
        )
    }

    pub fn is_mounted(&self) -> bool {
        matches!(self.state.mount_phase, MountPhase::Mounted) || self.is_remounting()
    }

    pub fn dismount(&mut self) {
        todo!()
    }

    pub fn points(&self) -> &Vec<EntityRegistryIndex> {
        &self.points
    }

    pub fn bones(&self) -> &Vec<EntityRegistryIndex> {
        &self.bones
    }

    pub fn joints(&self) -> &Vec<EntityRegistryIndex> {
        &self.joints
    }

    pub fn mount_bones(&self) -> &Vec<EntityRegistryIndex> {
        &self.state.mount_bones
    }

    pub fn mount_joints(&self) -> &Vec<EntityRegistryIndex> {
        &self.state.mount_joints
    }
}
