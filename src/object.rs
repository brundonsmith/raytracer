
use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::texture::Texture;

pub struct Intersection {
    pub distance: f32,
    pub position: Vec3,
    pub normal: Vec3,
}

pub trait Object<T: Texture> {
    fn intersection(&self, ray: &Ray) -> Option<Intersection>;
    fn material_type(&self) -> MaterialType;
    fn color(&self) -> Color;
    fn texture(&self) -> &T;
    fn texture_coordinate(&self, point: &Vec3) -> (f32,f32);
}

#[derive(Debug, Copy, Clone)]
pub enum MaterialType {
    Diffuse,
    Reflective,
    Transparent,
    Light
}
