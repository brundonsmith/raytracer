use std::f32::consts::PI;

use crate::utils::clamp;
use crate::vec3::Vec3;

pub struct Intersection {
    pub distance: f32,
    pub position: Vec3,
    pub normal: Vec3,
    pub direction: Vec3,
    reflected_direction: Option<Vec3>,
}

impl Intersection {

    pub fn new(distance: f32, position: Vec3, normal: Vec3, direction: Vec3) -> Self {
        Self {
            distance,
            position,
            normal,
            direction,
            reflected_direction: None,
        }
    }

    /**
     * The angle between the intersecting ray and the normal at that point
     */
    pub fn incident_angle(&self) -> f32 {
        (&self.direction * -1.0).angle(&self.normal)
    }

    /**
     * The "directness" of the incident, between 0 and 1
     */
    pub fn incident_amount(&self) -> f32 {
        clamp(self.incident_angle() / (PI / 2.0), 0.0, 1.0)
    }

    pub fn reflected_direction(&mut self) -> &Vec3 {
        if self.reflected_direction.is_none() {
            //R=2(N⋅L)N−L
            self.reflected_direction = Some(&self.direction - &(&self.normal * (2.0 * &(&self.normal * &self.direction))));
        }

        return self.reflected_direction.as_ref().unwrap();
    }
}
