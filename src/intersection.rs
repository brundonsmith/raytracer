use std::f32::consts::PI;

use crate::utils::clamp;
use crate::vec3::Vec3;

pub struct Intersection {
    pub distance: f32,
    pub position: Vec3,
    pub normal: Vec3,
    pub direction: Vec3,
    pub reflected_direction: Vec3,
}

impl Intersection {

    /**
     * The angle between the intersecting ray and the normal at that point
     */
    pub fn incident_angle(&self) -> f32 {
        (&self.direction * -1.0).angle_to(&self.normal)
    }

    /**
     * The "directness" of the incident, between 0 and 1
     */
    pub fn incident_amount(&self) -> f32 {
        clamp(self.incident_angle() / (PI / 2.0), 0.0, 1.0)
    }
}
