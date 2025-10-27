pub(super) enum MountPhase {
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

pub(crate) struct EntitySkeletonState {
    pub(super) mount_phase: MountPhase,
    pub(super) intact: bool,
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
            intact: self.intact,
        }
    }
}
