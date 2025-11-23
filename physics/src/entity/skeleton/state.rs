use vector2d::Vector2Df;

use crate::entity::skeleton::MountPhase;

pub struct EntitySkeletonState {
    mount_phase: MountPhase,
    sled_intact: bool,
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
            mount_phase: mount_phase_clone,
            sled_intact: self.sled_intact,
        }
    }
}

impl EntitySkeletonState {
    pub fn mount_phase(&self) -> MountPhase {
        self.mount_phase
    }

    pub fn sled_intact(&self) -> bool {
        self.sled_intact
    }

    pub fn points(&self) -> Vec<(Vector2Df, Vector2Df)> {
        todo!("Get points from somewhere?")
    }
}
