use std::f32::consts::PI;
use std::ops::{Add,Div};

use crate::object::Object;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::intersection::Intersection;

const TWO_PI: f32 = PI * 2.0;
const PI_OVER_TWO: f32 = PI / 2.0;


pub fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    if val < min { min }
    else if val > max { max } 
    else { val }
}

pub fn avg(a: f32, b: f32) -> f32 {
    (a + b) / 2.0
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

pub fn avg_all<'a,T: Add<Output=T> + Div<usize, Output=T>,I: Iterator<Item = T>>(elements: I) -> Option<T> {
    let mut count = 0;
    let mut sum: Option<T> = None;

    for el in elements {
        count += 1;

        if sum.is_none() {
            sum = Some(el);
        } else {
            sum = Some(sum.unwrap() + el);
        }
    }

    return sum.map(|s| s / count);
}

pub type ObjectVec = Vec<Box<dyn Object + Sync + Send>>;