use std::f64::INFINITY;

use crate::entity::{
    bone::template::EntityBoneTemplate,
    registry::{EntityBoneTemplateId, EntityPointTemplateId},
    skeleton::builder::EntitySkeletonBuilder,
};

pub struct EntityBoneBuilder<'a> {
    skeleton: EntitySkeletonBuilder<'a>,
    connected_points: (EntityPointTemplateId, EntityPointTemplateId),
    bias: Option<f64>,
    initial_length_factor: Option<f64>,
    repel_only: bool,
    endurance: Option<f64>,
    adjustment_strength: Option<f64>,
    endurance_remount_factor: Option<f64>,
    adjustment_strength_remount_factor: Option<f64>,
}

impl<'a> EntityBoneBuilder<'a> {
    pub fn new(
        skeleton: EntitySkeletonBuilder<'a>,
        p1: EntityPointTemplateId,
        p2: EntityPointTemplateId,
    ) -> EntityBoneBuilder<'a> {
        Self {
            skeleton,
            connected_points: (p1, p2),
            bias: None,
            initial_length_factor: None,
            repel_only: false,
            endurance: None,
            adjustment_strength: None,
            endurance_remount_factor: None,
            adjustment_strength_remount_factor: None,
        }
    }

    pub fn bias(mut self, bias: f64) -> Self {
        self.bias = Some(bias);
        self
    }

    pub fn initial_length_factor(mut self, rest_length_factor: f64) -> Self {
        self.initial_length_factor = Some(rest_length_factor);
        self
    }

    pub fn repel(mut self) -> Self {
        self.repel_only = true;
        self
    }

    pub fn endurance(mut self, endurance: f64) -> Self {
        self.endurance = Some(endurance);
        self
    }

    pub fn adjustment_strength(mut self, strength: f64) -> Self {
        self.adjustment_strength = Some(strength);
        self
    }

    pub fn endurance_remount_factor(mut self, factor: f64) -> Self {
        self.endurance_remount_factor = Some(factor);
        self
    }

    pub fn adjustment_strength_remount_factor(mut self, factor: f64) -> Self {
        self.adjustment_strength_remount_factor = Some(factor);
        self
    }

    pub fn build(self) -> (EntitySkeletonBuilder<'a>, EntityBoneTemplateId) {
        let p0 = self
            .skeleton
            .registry()
            .get_point_template(self.connected_points.0);
        let p1 = self
            .skeleton
            .registry()
            .get_point_template(self.connected_points.1);
        let is_flutter = !(p0.is_contact() && p1.is_contact());

        let bone_template = EntityBoneTemplate {
            connected_points: self.connected_points,
            is_flutter,
            bias: self.bias.unwrap_or(0.5),
            initial_length_factor: self.initial_length_factor.unwrap_or(1.0),
            repel_only: self.repel_only,
            endurance: self.endurance.unwrap_or(INFINITY),
            adjustment_strength: self.adjustment_strength.unwrap_or(1.0),
            endurance_remount_factor: self.endurance_remount_factor.unwrap_or(1.0),
            adjustment_strength_remount_factor: self
                .adjustment_strength_remount_factor
                .unwrap_or(1.0),
        };
        self.skeleton.add_bone(bone_template)
    }
}
