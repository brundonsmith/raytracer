
use rand::Rng;
use rand::rngs::SmallRng;
use std::f32::consts::PI;

const PI_OVER_TWO: f32 = PI / 2.0;

use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {

    pub fn new() -> Self {
        Self {
            origin: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            direction: Vec3 { x: 1.0, y: 0.0, z: 0.0 },
        }
    }

    pub fn random_direction(origin: Vec3, rng: &mut SmallRng) -> Self {
        Self {
            origin,
            direction: Vec3::from_angles(
                rng.gen_range(0.0, PI * 2.0),
                rng.gen_range(-1.0 * PI_OVER_TWO, PI_OVER_TWO),
            )
        }
    }
}
