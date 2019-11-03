
use std::f32::consts::PI;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::object::Object;
use crate::intersection::Intersection;
use crate::material::Material;

pub struct Plane {
    pub position: Vec3,
    pub normal: Vec3,
    pub material: Material,
    projected_bias: Vec3,
    rotated_projected_bias: Vec3,
}

impl Plane {

    pub fn new(position: Vec3, normal: Vec3, bias: Vec3, material: Material) -> Self {
        let projected_bias = projection(&position, &normal, &bias).normalized();

        Self {
            position,
            normal,
            material,
            projected_bias,
            rotated_projected_bias: projected_bias.rotated_around(&normal, PI / -2.0).normalized()
        }
    }

    pub fn projection(&self, point: &Vec3) -> Vec3 {
        projection(&self.position, &self.normal, point)
    }
}

impl Object for Plane {

    fn intersection(&self, ray: &Ray) -> Option<Intersection> {
        let numerator = (&self.position - &ray.origin).dot(&self.normal);
        let denominator = ray.direction.dot(&self.normal);
        let distance = numerator / denominator;

        if distance > 0.0 {
            let point = &ray.origin + &(&ray.direction * distance);

            return Some(Intersection::new(
                distance,
                &point + &(&self.normal * 0.01), // offset to avoid floating-point error
                self.normal,
                ray.direction,
            ));
        } else {
            return None;
        };
    }

    fn texture_coordinate(&self, point: &Vec3) -> (f32,f32) {
        let plane_projection = self.projection(point);

        let proj_y = plane_projection.projected_on(&self.projected_bias);
        let u = (&plane_projection - &proj_y).len() / 2.0;

        let proj_x = plane_projection.projected_on(&self.rotated_projected_bias);
        let v = (&plane_projection - &proj_x).len() / 2.0;

        (u - u.floor(), v - v.floor())
    }

    fn get_position(&self) -> &Vec3 {
        &self.position
    }

    fn get_material(&self) -> &Material {
        &self.material
    }
}

fn projection(origin: &Vec3, normal: &Vec3, point: &Vec3) -> Vec3 {
    point - &point.projected_on(&(origin + normal))
}