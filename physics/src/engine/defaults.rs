use vector2d::Vector2Df;

use crate::entity::registry::EntitySkeletonId;

pub(super) fn default_get_gravity_at_time(_frame: u32) -> Vector2Df {
    Vector2Df::down()
}

pub(super) fn default_get_skeleton_frozen_at_time(
    _skeleton: EntitySkeletonId,
    _frame: u32,
) -> bool {
    false
}
