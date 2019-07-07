
use crate::color::Color;
use crate::ray::Ray;

pub trait Object {
    fn intersection(&self, ray: &Ray) -> Option<f32>;
    fn material_type(&self) -> MaterialType;
    fn color(&self) -> Color;
}

#[derive(Debug, Copy, Clone)]
pub enum MaterialType {
    Diffuse,
    Reflective,
    Transparent,
    Light
}
