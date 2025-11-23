pub(crate) mod builder;
pub(crate) mod entity;
pub(crate) mod snapshot;
pub(crate) mod state;
pub(crate) mod template;

pub enum MountPhase {
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

impl Clone for MountPhase {
    fn clone(&self) -> Self {
        match self {
            MountPhase::Mounted => MountPhase::Mounted,
            MountPhase::Remounting {
                frames_until_remounted,
            } => MountPhase::Remounting {
                frames_until_remounted: *frames_until_remounted,
            },
            MountPhase::Dismounted {
                frames_until_can_remount,
            } => MountPhase::Dismounted {
                frames_until_can_remount: *frames_until_can_remount,
            },
            MountPhase::Dismounting {
                frames_until_dismounted,
            } => MountPhase::Dismounting {
                frames_until_dismounted: *frames_until_dismounted,
            },
        }
    }
}

impl Copy for MountPhase {}
