
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    //ray_type: RayType,
    pub origin: Vec3,
    pub direction: Vec3
}

enum RayType {
    Primary,
    Shadow,
    Diffuse,
    Reflection,
    Refraction
}
