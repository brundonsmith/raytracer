
use crate::ray::Ray;

pub trait Object {
    fn intersects(&self, ray: &Ray) -> Option<f32>;
}
