use std::f64::INFINITY;

use vector2d::Vector2Df;

use crate::entity::point::{EntityPoint, EntityPointIndex};

fn get_point(index: &EntityPointIndex) -> &EntityPoint {
    todo!("Need to get this from a registry")
}

pub type EntityBoneIndex = usize;

pub struct EntityBone {
    connected_points: (EntityPointIndex, EntityPointIndex),
    bias: f64,
    initial_length: f64,
    rest_length_factor: f64,
    repel_only: bool,
    endurance: f64,
    adjustment_strength: f64,
    remount_endurance_factor: f64,
    remount_adjustment_strength_factor: f64,
}

pub struct EntityBoneBuilder {
    connected_points: Option<(EntityPointIndex, EntityPointIndex)>,
    bias: Option<f64>,
    rest_length_factor: Option<f64>,
    repel_only: bool,
    endurance: Option<f64>,
    adjustment_strength: Option<f64>,
    remount_endurance_factor: Option<f64>,
    remount_adjustment_strength_factor: Option<f64>,
}

#[derive(Debug, Clone)]
pub enum EntityBoneBuilderError {
    MissingPoints,
}

impl EntityBoneBuilder {
    pub fn new() -> EntityBoneBuilder {
        EntityBoneBuilder {
            connected_points: None,
            bias: None,
            rest_length_factor: None,
            repel_only: false,
            endurance: None,
            adjustment_strength: None,
            remount_endurance_factor: None,
            remount_adjustment_strength_factor: None,
        }
    }

    pub fn points(&mut self, p1: EntityPointIndex, p2: EntityPointIndex) -> &mut Self {
        self.connected_points = Some((p1, p2));
        self
    }

    pub fn bias(&mut self, bias: f64) -> &mut Self {
        self.bias = Some(bias);
        self
    }

    pub fn rest_length_factor(&mut self, rest_length_factor: f64) -> &mut Self {
        self.rest_length_factor = Some(rest_length_factor);
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

    pub fn remount_endurance_factor(&mut self, factor: f64) -> &mut Self {
        self.remount_endurance_factor = Some(factor);
        self
    }

    pub fn remount_adjustment_strength_factor(&mut self, factor: f64) -> &mut Self {
        self.remount_adjustment_strength_factor = Some(factor);
        self
    }

    pub fn build(&self) -> Result<EntityBone, EntityBoneBuilderError> {
        if let Some(connected_points) = self.connected_points {
            let bone_vector = get_point(&connected_points.1).position()
                - get_point(&connected_points.0).position();
            Ok(EntityBone {
                connected_points,
                bias: self.bias.unwrap_or(0.5),
                initial_length: bone_vector.length(),
                rest_length_factor: self.rest_length_factor.unwrap_or(1.0),
                repel_only: self.repel_only,
                endurance: self.endurance.unwrap_or(INFINITY),
                adjustment_strength: self.adjustment_strength.unwrap_or(1.0),
                remount_endurance_factor: self.remount_endurance_factor.unwrap_or(1.0),
                remount_adjustment_strength_factor: self.remount_endurance_factor.unwrap_or(1.0),
            })
        } else {
            Err(EntityBoneBuilderError::MissingPoints)
        }
    }
}

impl EntityBone {
    pub fn get_vector(&self) -> Vector2Df {
        get_point(&self.connected_points.1).position()
            - get_point(&self.connected_points.0).position()
    }

    pub fn get_percent_adjustment(&self) -> f64 {
        let bone_vector = self.get_vector();
        let current_length = bone_vector.length();
        let should_repel = current_length < self.initial_length * self.rest_length_factor;

        if current_length == 0.0 || (self.repel_only && !should_repel) {
            0.0
        } else {
            (current_length - self.initial_length * self.rest_length_factor) / current_length
        }
    }

    pub fn get_adjustment(&self, remounting: bool) -> (Vector2Df, Vector2Df) {
        let bone_vector = self.get_vector();
        let percent_adjustment = self.get_percent_adjustment();

        let adjustment_strength = if remounting {
            self.adjustment_strength * self.remount_adjustment_strength_factor
        } else {
            self.adjustment_strength
        };

        (
            -1.0 * bone_vector * adjustment_strength * percent_adjustment * (1.0 - self.bias),
            -1.0 * bone_vector * adjustment_strength * percent_adjustment * self.bias,
        )
    }

    pub fn get_intact(&self, remounting: bool) -> bool {
        let percent_adjustment = self.get_percent_adjustment();

        let endurance = if remounting {
            self.endurance * self.remount_endurance_factor
        } else {
            self.endurance
        };

        return percent_adjustment <= endurance * self.initial_length * self.rest_length_factor;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_percent_adjustment() {
        todo!("length 0");
        todo!("length not 0");
        todo!("repel and should repel");
        todo!("repel and not should repel");
        todo!("large rest length factor");
        todo!("small rest length factor");
    }

    #[test]
    fn get_adjustment() {
        todo!("half bias");
        todo!("full 0 bias");
        todo!("full 1 bias");
        todo!("length 0");
        todo!("repel and not should repel");
        todo!("remounting adjustment factor");
        todo!("large rest length factor");
        todo!("small rest length factor");
    }

    #[test]
    fn get_intact() {
        todo!("length 0");
        todo!("repel and not should repel");
        todo!("infinite endurance");
        todo!("remounting endurance factor");
    }
}
