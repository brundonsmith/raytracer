
use std::f32::consts::PI;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::object::Object;
use crate::intersection::Intersection;
use crate::material::Material;

pub struct Sphere {
    position: Vec3,
    radius: f32,
    material: Material,
    radius_squared: f32,
}

impl Sphere {

    pub fn new(position: Vec3, radius: f32, material: Material) -> Self {
        Self {
            position,
            radius,
            material,
            radius_squared: radius * radius,
        }
    }

    /*
    pub fn surface_point(&self, latitude: f32, longitude: f32) -> Vec3 {
        let lat_cos = latitude.cos();
        let lon_sin = longitude.sin();
        let lat_sin = latitude.sin();

        &self.position + &(&Vec3 {
            x: lat_cos * lon_sin,
            y: lat_cos,
            z: lat_sin * lon_sin
        } * self.radius)
    }
    */
}

impl Object for Sphere {

    fn intersection(&self, ray: &Ray) -> Option<Intersection> {
        
        // analytic solution
        let l: Vec3 = &ray.origin - &self.position;
        let a: f32 = ray.direction.dot(&ray.direction);
        let b: f32 = 2.0 * ray.direction.dot(&l);
        let c: f32 = l.dot(&l) - self.radius_squared;

        return match solve_quadratic(a, b, c) {
            Some((mut t0, t1)) => {

                if t0 < 0.0 {
                    t0 = t1; // if t0 is negative, let's use t1 instead
                    
                    if t0 < 0.0 {
                        return None; // both t0 and t1 are negative
                    }
                }

                let distance = t0;
                let position = &ray.origin + &(&ray.direction * distance);
                let mut normal = &position - &self.position;
                normal.normalize();
                let direction = ray.direction;

                return Some(Intersection::new(
                    distance,
                    &position + &(&normal * 0.01), // offset to avoid floating-point error
                    normal,
                    direction,
                ));
            },
            None => None
        };
    }

    fn texture_coordinate(&self, point: &Vec3) -> (f32,f32) {
        let relative_point = point - &self.position;

        let longitude = (relative_point.z / relative_point.x).atan();
        let continuous_longitude = longitude 
            + if relative_point.x < 0.0 { PI } else { 0.0 }
            + if relative_point.x >= 0.0 && relative_point.z < 0.0 { 2.0 * PI } else { 0.0 };

        let latitude = (relative_point.y / self.radius).acos();

        let u = continuous_longitude / (2.0 * PI);
        let v = 1.0 - latitude / PI;

        // Some extra tiling (4x4)
        return (
            (u * 4.0) - (u * 4.0).floor(), 
            (v * 4.0) - (v * 4.0).floor()
        );
    }

    fn get_material(&self) -> &Material {
        &self.material
    }
}


fn solve_quadratic(a: f32, b: f32, c: f32) -> Option<(f32,f32)> {
    let discr = b * b - 4.0 * a * c;

    let mut t0: f32;
    let mut t1: f32;

    if discr < 0.0 {
        return None;
    } else if discr == 0.0 {//discr.abs() < 0.0001 {
        t0 = -0.5 * b / a;
        t1 = t0;
    } else {
        let q = 
            if b > 0.0 {
                -0.5 * (b + discr.sqrt())
            } else {
                -0.5 * (b - discr.sqrt())
            };

        t0 = q / a;
        t1 = c / q;
    }

    if t0 > t1 {
        std::mem::swap(&mut t0, &mut t1);
    }

    return Some((t0, t1));
}
