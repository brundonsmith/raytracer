
use rand::rngs::SmallRng;

use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::intersection::Intersection;
use crate::illumination::Illumination;

use crate::plane::Plane;
use crate::sphere::Sphere;
use crate::mesh::Mesh;

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


    fn shade(&self, ray: &Ray, objs: &Vec<ObjectEnum>, rng: &mut SmallRng, bounces_remaining: u8) -> Illumination;
}

pub enum ObjectEnum {
    Plane(Plane),
    Sphere(Sphere),
    Mesh(Mesh)
}

impl Object for ObjectEnum {
    fn intersection(&self, ray: &Ray) -> Option<Intersection> {
        match self {
            ObjectEnum::Plane(data) => data.intersection(ray),
            ObjectEnum::Sphere(data) => data.intersection(ray),
            ObjectEnum::Mesh(data) => data.intersection(ray),
        }
    }
    fn texture_coordinate(&self, point: &Vec3) -> (f32,f32) {
        match self {
            ObjectEnum::Plane(data) => data.texture_coordinate(point),
            ObjectEnum::Sphere(data) => data.texture_coordinate(point),
            ObjectEnum::Mesh(data) => data.texture_coordinate(point),
        }
    }
    fn shade(&self, ray: &Ray, objs: &Vec<ObjectEnum>, rng: &mut SmallRng, bounces_remaining: u8) -> Illumination {
        match self {
            ObjectEnum::Plane(data) => data.shade(ray, objs, rng, bounces_remaining),
            ObjectEnum::Sphere(data) => data.shade(ray, objs, rng, bounces_remaining),
            ObjectEnum::Mesh(data) => data.shade(ray, objs, rng, bounces_remaining),
        }
    }

}