use std::{collections::HashMap, f64::INFINITY};

use vector2d::Vector2Df;

use crate::entity::{
    bone::EntityBone,
    registry::{EntityPointId, EntityPointTemplateId, EntityRegistry},
};

pub(crate) struct EntityBoneTemplate {
    connected_points: (EntityPointTemplateId, EntityPointTemplateId),
    bias: Option<f64>,
    initial_length_factor: Option<f64>,
    repel_only: bool,
    endurance: Option<f64>,
    adjustment_strength: Option<f64>,
    endurance_remount_factor: Option<f64>,
    adjustment_strength_remount_factor: Option<f64>,
}

impl EntityBoneTemplate {
    pub fn new(points: (EntityPointTemplateId, EntityPointTemplateId)) -> Self {
        Self {
            connected_points: points,
            bias: None,
            initial_length_factor: None,
            repel_only: false,
            endurance: None,
            adjustment_strength: None,
            endurance_remount_factor: None,
            adjustment_strength_remount_factor: None,
        }
    }

    pub fn bias(&mut self, bias: f64) -> &mut Self {
        self.bias = Some(bias);
        self
    }

    pub fn initial_length_factor(&mut self, rest_length_factor: f64) -> &mut Self {
        self.initial_length_factor = Some(rest_length_factor);
        self
    }

    pub fn repel(&mut self) -> &mut Self {
        self.repel_only = true;
        self
    }

    pub fn endurance(&mut self, endurance: f64) -> &mut Self {
        self.endurance = Some(endurance);
        self
    }

    pub fn adjustment_strength(&mut self, strength: f64) -> &mut Self {
        self.adjustment_strength = Some(strength);
        self
    }

    pub fn endurance_remount_factor(&mut self, factor: f64) -> &mut Self {
        self.endurance_remount_factor = Some(factor);
        self
    }

    pub fn adjustment_strength_remount_factor(&mut self, factor: f64) -> &mut Self {
        self.adjustment_strength_remount_factor = Some(factor);
        self
    }

    pub fn build(
        &self,
        registry: &EntityRegistry,
        mapping: &HashMap<EntityPointTemplateId, EntityPointId>,
    ) -> EntityBone {
        let point_ids = (
            mapping[&self.connected_points.0],
            mapping[&self.connected_points.1],
        );
        let points = (
            registry.get_point(point_ids.0),
            registry.get_point(point_ids.1),
        );
        EntityBone {
            connected_points: point_ids,
            initial_length: Vector2Df::distance(points.0.position(), points.1.position()),
            bias: self.bias.unwrap_or(0.5),
            initial_length_factor: self.initial_length_factor.unwrap_or(1.0),
            repel_only: self.repel_only,
            endurance: self.endurance.unwrap_or(INFINITY),
            adjustment_strength: self.adjustment_strength.unwrap_or(1.0),
            endurance_remount_factor: self.endurance_remount_factor.unwrap_or(1.0),
            adjustment_strength_remount_factor: self.endurance_remount_factor.unwrap_or(1.0),
        }
    }
}
