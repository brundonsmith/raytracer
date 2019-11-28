
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::material::Material;
use crate::intersection::Intersection;

pub trait Object {

    /**
     * Get information about the point where a ray intersects this object, 
     * if it does at all.
     */
    fn intersection(&self, ray: &Ray) -> Option<Intersection>;

    /**
     * Get the UV coordinate on this object's texture for a given 
     * world-space coordinate.
     */
    fn texture_coordinate(&self, point: &Vec3) -> (f32,f32);

    fn get_material(&self) -> &Material;
}
