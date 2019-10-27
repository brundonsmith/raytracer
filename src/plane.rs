
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::object::Object;
use crate::intersection::Intersection;
use crate::material::Material;

pub struct Plane {
    pub position: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl Plane {
    pub fn new(position: Vec3, normal: Vec3, material: Material) -> Self {
        Self {
            position,
            normal,
            material,
        }
    }
}

impl Object for Plane {

    fn intersection(&self, ray: &Ray) -> Option<Intersection> {
        let distance = (&(&self.position - &ray.origin) * &self.normal) / (&ray.direction * &self.normal);

        return if distance > 0.0 {
            //println!("Intersected plane at {:?}", &(&ray.origin + &ray.direction) * distance);

            Some(Intersection {
                distance,
                position: &ray.origin + &(&ray.direction * distance),
                normal: self.normal,
                direction: ray.direction,
            })
        } else {
            None
        };
    }

    fn texture_coordinate(&self, point: &Vec3) -> (f32,f32) {
        //let projected = point - (self.normal * (point - self.origin)) * self.normal;
        (0.0, 0.0)
    }

    fn get_position(&self) -> &Vec3 {
        &self.position
    }

    fn get_material(&self) -> &Material {
        &self.material
    }
}