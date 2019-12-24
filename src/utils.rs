use std::f32::consts::PI;
use std::ops::{Add,Div};

use crate::object::Object;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::intersection::Intersection;

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
    t * (b - a).abs() + f32::min(a, b)
}

pub fn plane_intersection(position: &Vec3, normal: &Vec3, ray: &Ray) -> Option<Intersection> {
    let numerator = (position - &ray.origin).dot(&normal);
    let denominator = ray.direction.dot(&normal);
    let distance = numerator / denominator;

    if distance > 0.0 {
        let point = &ray.origin + &(&ray.direction * distance);

        return Some(Intersection::new(
            distance,
            &point + &(normal * 0.01), // offset to avoid floating-point error
            *normal,
            ray.direction,
        ));
    } else {
        return None;
    };
}

pub type ObjectVec = Vec<Box<dyn Object + Sync + Send>>;