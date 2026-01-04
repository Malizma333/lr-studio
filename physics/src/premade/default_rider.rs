use format_core::track::RemountVersion;
use vector2d::Vector2Df;

use crate::{Engine, entity::registry::EntitySkeletonTemplateId};

// Known bug: Default riders of different remount versions are not able to
// cross-remount with each other because they come from different templates,
// even though they normally would in linerider.com. This is such a niche case
// that it's probably not worth fixing.
pub fn build_default_rider(
    engine: &mut Engine,
    version: RemountVersion,
) -> EntitySkeletonTemplateId {
    let repel_length_factor = 0.5;
    let scarf_friction = 0.1;
    let mount_endurance = 0.057;
    let remount_endurance_factor = 2.0;
    let remount_strength_factor = match version {
        RemountVersion::None => 0.0,
        RemountVersion::ComV1 => 0.1,
        RemountVersion::ComV2 => 0.1,
        RemountVersion::LRA => 0.5,
    };
    // Adjustment strength when remounting affects all bones in LRA
    let unbreakable_remount_strength_factor = match version {
        RemountVersion::LRA => 0.5,
        _ => 1.0,
    };

    // Remount version also affects physics processing order, which is why it's needed internally
    let mut skeleton = engine.build_skeleton().remount_version(version);

    skeleton = match version {
        RemountVersion::None => skeleton,
        _ => skeleton
            .dismounted_timer(30)
            .remounting_timer(3)
            .mounted_timer(3),
    };

    let (skeleton, peg) = skeleton
        .point(Vector2Df::new(0.0, 0.0))
        .contact()
        .contact_friction(0.8)
        .build();
    let (skeleton, tail) = skeleton.point(Vector2Df::new(0.0, 5.0)).contact().build();
    let (skeleton, nose) = skeleton.point(Vector2Df::new(15.0, 5.0)).contact().build();
    let (skeleton, string) = skeleton.point(Vector2Df::new(17.5, 0.0)).contact().build();
    let (skeleton, butt) = skeleton
        .point(Vector2Df::new(5.0, 0.0))
        .contact()
        .contact_friction(0.8)
        .build();
    let (skeleton, shoulder) = skeleton
        .point(Vector2Df::new(5.0, -5.5))
        .contact()
        .contact_friction(0.8)
        .build();
    let (skeleton, right_hand) = skeleton
        .point(Vector2Df::new(11.5, -5.0))
        .contact()
        .contact_friction(0.1)
        .build();
    let (skeleton, left_hand) = skeleton
        .point(Vector2Df::new(11.5, -5.0))
        .contact()
        .contact_friction(0.1)
        .build();
    let (skeleton, left_foot) = skeleton.point(Vector2Df::new(10.0, 5.0)).contact().build();
    let (skeleton, right_foot) = skeleton.point(Vector2Df::new(10.0, 5.0)).contact().build();
    let (skeleton, scarf0) = skeleton
        .point(Vector2Df::new(3.0, -5.5))
        .air_friction(scarf_friction)
        .build();
    let (skeleton, scarf1) = skeleton
        .point(Vector2Df::new(1.0, -5.5))
        .air_friction(scarf_friction)
        .build();
    let (skeleton, scarf2) = skeleton
        .point(Vector2Df::new(-1.0, -5.5))
        .air_friction(scarf_friction)
        .build();
    let (skeleton, scarf3) = skeleton
        .point(Vector2Df::new(-3.0, -5.5))
        .air_friction(scarf_friction)
        .build();
    let (skeleton, scarf4) = skeleton
        .point(Vector2Df::new(-5.0, -5.5))
        .air_friction(scarf_friction)
        .build();
    let (skeleton, scarf5) = skeleton
        .point(Vector2Df::new(-7.0, -5.5))
        .air_friction(scarf_friction)
        .build();
    let (skeleton, scarf6) = skeleton
        .point(Vector2Df::new(-9.0, -5.5))
        .air_friction(scarf_friction)
        .build();

    let (skeleton, sled_back) = skeleton
        .bone(peg, tail)
        .adjustment_strength_remount_factor(unbreakable_remount_strength_factor)
        .build();
    let (skeleton, _) = skeleton
        .bone(tail, nose)
        .adjustment_strength_remount_factor(unbreakable_remount_strength_factor)
        .build();
    let (skeleton, _) = skeleton
        .bone(nose, string)
        .adjustment_strength_remount_factor(unbreakable_remount_strength_factor)
        .build();
    let (skeleton, sled_front) = skeleton
        .bone(string, peg)
        .adjustment_strength_remount_factor(unbreakable_remount_strength_factor)
        .build();
    let (skeleton, _) = skeleton
        .bone(peg, nose)
        .adjustment_strength_remount_factor(unbreakable_remount_strength_factor)
        .build();
    let (skeleton, _) = skeleton
        .bone(string, tail)
        .adjustment_strength_remount_factor(unbreakable_remount_strength_factor)
        .build();
    let (skeleton, _) = skeleton
        .bone(peg, butt)
        .endurance(mount_endurance)
        .endurance_remount_factor(remount_endurance_factor)
        .adjustment_strength_remount_factor(remount_strength_factor)
        .build();
    let (skeleton, _) = skeleton
        .bone(tail, butt)
        .endurance(mount_endurance)
        .endurance_remount_factor(remount_endurance_factor)
        .adjustment_strength_remount_factor(remount_strength_factor)
        .build();
    let (skeleton, _) = skeleton
        .bone(nose, butt)
        .endurance(mount_endurance)
        .endurance_remount_factor(remount_endurance_factor)
        .adjustment_strength_remount_factor(remount_strength_factor)
        .build();
    let (skeleton, torso) = skeleton
        .bone(shoulder, butt)
        .adjustment_strength_remount_factor(unbreakable_remount_strength_factor)
        .build();
    let (skeleton, _) = skeleton
        .bone(shoulder, left_hand)
        .adjustment_strength_remount_factor(unbreakable_remount_strength_factor)
        .build();
    let (skeleton, _) = skeleton
        .bone(shoulder, right_hand)
        .adjustment_strength_remount_factor(unbreakable_remount_strength_factor)
        .build();
    let (skeleton, _) = skeleton
        .bone(butt, left_foot)
        .adjustment_strength_remount_factor(unbreakable_remount_strength_factor)
        .build();
    let (skeleton, _) = skeleton
        .bone(butt, right_foot)
        .adjustment_strength_remount_factor(unbreakable_remount_strength_factor)
        .build();
    let (skeleton, _) = skeleton
        .bone(shoulder, right_hand)
        .adjustment_strength_remount_factor(unbreakable_remount_strength_factor)
        .build();
    let (skeleton, _) = skeleton
        .bone(shoulder, peg)
        .endurance(mount_endurance)
        .endurance_remount_factor(remount_endurance_factor)
        .adjustment_strength_remount_factor(remount_strength_factor)
        .build();
    let (skeleton, _) = skeleton
        .bone(left_hand, string)
        .endurance(mount_endurance)
        .endurance_remount_factor(remount_endurance_factor)
        .adjustment_strength_remount_factor(remount_strength_factor)
        .build();
    let (skeleton, _) = skeleton
        .bone(right_hand, string)
        .endurance(mount_endurance)
        .endurance_remount_factor(remount_endurance_factor)
        .adjustment_strength_remount_factor(remount_strength_factor)
        .build();
    let (skeleton, _) = skeleton
        .bone(left_foot, nose)
        .endurance(mount_endurance)
        .endurance_remount_factor(remount_endurance_factor)
        .adjustment_strength_remount_factor(remount_strength_factor)
        .build();
    let (skeleton, _) = skeleton
        .bone(right_foot, nose)
        .endurance(mount_endurance)
        .endurance_remount_factor(remount_endurance_factor)
        .adjustment_strength_remount_factor(remount_strength_factor)
        .build();
    let (skeleton, _) = skeleton
        .bone(shoulder, left_foot)
        .repel()
        .initial_length_factor(repel_length_factor)
        .adjustment_strength_remount_factor(unbreakable_remount_strength_factor)
        .build();
    let (skeleton, _) = skeleton
        .bone(shoulder, right_foot)
        .repel()
        .initial_length_factor(repel_length_factor)
        .adjustment_strength_remount_factor(unbreakable_remount_strength_factor)
        .build();
    let (skeleton, _) = skeleton.bone(shoulder, scarf0).bias(1.0).build();
    let (skeleton, _) = skeleton.bone(scarf0, scarf1).bias(1.0).build();
    let (skeleton, _) = skeleton.bone(scarf1, scarf2).bias(1.0).build();
    let (skeleton, _) = skeleton.bone(scarf2, scarf3).bias(1.0).build();
    let (skeleton, _) = skeleton.bone(scarf3, scarf4).bias(1.0).build();
    let (skeleton, _) = skeleton.bone(scarf4, scarf5).bias(1.0).build();
    let (skeleton, _) = skeleton.bone(scarf5, scarf6).bias(1.0).build();

    let (skeleton, _) = skeleton.joint(sled_back, sled_front).mount().build();
    let (skeleton, _) = skeleton.joint(torso, sled_front).mount().build();
    let (skeleton, _) = skeleton.joint(sled_back, sled_front).build();

    skeleton.build()
}
