use std::f32::consts::PI;

use crate::object::{ObjectEnum};
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::color::Color;
use crate::matrix::Matrix;

pub const TWO_PI: f32 = PI * 2.0;
pub const PI_OVER_TWO: f32 = PI / 2.0;


pub fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    if val < min { min }
    else if val > max { max } 
    else { val }
}

pub fn avg(a: f32, b: f32) -> f32 {
    (a + b) / 2.0
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

pub fn plane_intersection(position: &Vec3, normal: &Vec3, ray: &Ray) -> Option<Intersection> {
    let numerator = (position - &ray.origin).dot(&normal);
    let denominator = ray.direction.dot(&normal);
    let distance = numerator / denominator;

    if distance > 0.0 {
        let point = &ray.origin + &(&ray.direction * distance);

        return Some(Intersection::new(
            distance,
            &point + &(normal * 0.001), // offset to avoid floating-point error
            *normal,
            ray.direction,
        ));
    } else {
        return None;
    };
}

const FORWARD: Vec3 = Vec3 { x: 0.0, y: 0.0, z: -1.0 };

pub fn color_to_normal(color: &Color) -> Vec3 {
    Vec3 {
        x: color.0 * 2.0 - 1.0,
        y: color.1 * 2.0 - 1.0,
        z: (color.2 * 2.0 - 1.0) * -1.0,
    }
}

pub fn adjusted_for_normal(original_normal: &Vec3, normal_from_map: &Vec3) -> Vec3 {
    let transformation = Matrix::from_to_rotation(&FORWARD, &normal_from_map);
    original_normal.transformed(&transformation)
}
