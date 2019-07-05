
use crate::ray::Ray;

pub trait Object {
    fn intersection(&self, ray: &Ray) -> Option<f32>;
}
