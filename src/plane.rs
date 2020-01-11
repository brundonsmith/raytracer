
use rand::rngs::SmallRng;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::object::Object;
use crate::intersection::Intersection;
use crate::material::Material;
use crate::utils::{plane_intersection,ObjectVec,PI_OVER_TWO};
use crate::illumination::Illumination;
use crate::matrix::Matrix;

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
            rotated_projected_bias: projected_bias.rotated_around(&normal, -1.0 * PI_OVER_TWO).normalized()
        }
    }

    pub fn projection(&self, point: &Vec3) -> Vec3 {
        projection(&self.position, &self.normal, point)
    }
}

const FORWARD: Vec3 = Vec3 { x: 0.0, y: 0.0, z: -1.0 };

impl Object for Plane {

    fn intersection(&self, ray: &Ray) -> Option<Intersection> {
        plane_intersection(&self.position, &self.normal, ray)
            .map(|mut intersection| {

                self.material.texture_normal.as_ref().map(|texture_normal| {
                    let normal_color = texture_normal.color_at(self.texture_coordinate(&intersection.position));
                    let normal_vec = Vec3 {
                        x: -1.0 * (normal_color.0 * 2.0 - 1.0),
                        y: normal_color.1 * 2.0 - 1.0,
                        z: (normal_color.2 * 2.0 - 1.0) * -1.0,
                    };
                    let transformation = Matrix::from_to_rotation(&FORWARD, &normal_vec);

                    intersection.normal = intersection.normal.transformed(&transformation);
                });
                
                intersection
            })
    }

    fn texture_coordinate(&self, point: &Vec3) -> (f32,f32) {
        let plane_projection = self.projection(point);

        let proj_y = plane_projection.projected_on(&self.projected_bias);
        let difference_vec_y = &plane_projection - &proj_y;
        let u = difference_vec_y.x.signum() * difference_vec_y.len() / 2.0;

        let proj_x = plane_projection.projected_on(&self.rotated_projected_bias);
        let difference_vec_x = &plane_projection - &proj_x;
        let v = difference_vec_x.y.signum() * difference_vec_x.len() / 2.0;

        (u - u.floor(), v - v.floor())
    }

    fn shade(&self, ray: &Ray, objs: &ObjectVec, rng: &mut SmallRng, depth: u8) -> Illumination {
        let mut intersection = self.intersection(ray).unwrap();
        let uv = self.texture_coordinate(&intersection.position);

        self.material.shade(
            &mut intersection,
            uv,
            objs,
            rng,
            depth
        )
    }
}

fn projection(origin: &Vec3, normal: &Vec3, point: &Vec3) -> Vec3 {
    point - &point.projected_on(&(origin + normal))
}