
use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Intersection {
    pub distance: f32,
    pub position: Vec3,
    pub normal: Vec3,
}

pub trait Object {
    fn intersection(&self, ray: &Ray) -> Option<Intersection>;
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
