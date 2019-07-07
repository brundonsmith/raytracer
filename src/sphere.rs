
use crate::color::Color;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::object::{Object,MaterialType};

pub struct Sphere {
    pub position: Vec3,
    pub radius: f32,
    pub material_type: MaterialType,
    pub color: Color
}

impl Sphere {

    pub fn new(position: Vec3, radius: f32, material_type: MaterialType, color: Color) -> Self {
        Self {
            position,
            radius,
            material_type,
            color
        }
    }

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

    pub fn radius_squared(&self) -> f32 {
        self.radius * self.radius
    }
}

impl Object for Sphere {

    fn intersection(&self, ray: &Ray) -> Option<f32> {
        
        // analytic solution
        let L: Vec3 = &ray.origin - &self.position;
        let a: f32 = &ray.direction * &ray.direction;
        let b: f32 = 2.0 * (&ray.direction * &L);
        let c: f32 = &L * &L - self.radius_squared();

        return match solve_quadratic(a, b, c) {
            Some((mut t0, mut t1)) => {

                if t0 > t1 {
                    std::mem::swap(&mut t0, &mut t1);
                }

                if t0 < 0.0 {
                    t0 = t1; // if t0 is negative, let's use t1 instead
                    
                    if t0 < 0.0 {
                        return None; // both t0 and t1 are negative
                    }
                }

                return Some(t0);
            },
            None => None
        };
    }

    fn material_type(&self) -> MaterialType {
        self.material_type
    }

    fn color(&self) -> Color {
        self.color
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
