
use rand::rngs::SmallRng;
//use flamer::flame;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::object::{Object,ObjectEnum};
use crate::intersection::Intersection;
use crate::material::Material;
use crate::utils::{plane_intersection,PI_OVER_TWO,adjusted_for_normal,color_to_normal};
use crate::illumination::Illumination;

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
            normal: normal.normalized(),
            material,
            projected_bias: projected_bias.normalized(),
            rotated_projected_bias: projected_bias.rotated_around(&normal, -1.0 * PI_OVER_TWO).normalized()
        }
    }

    pub fn projection(&self, point: &Vec3) -> Vec3 {
        projection(&self.position, &self.normal, point)
    }
}

impl Object for Plane {

//    #[flame("Plane")]
    fn intersection(&self, ray: &Ray) -> Option<Intersection> {
        plane_intersection(&self.position, &self.normal, ray)
            .map(|mut intersection| {
                
                self.material.texture_normal.as_ref().map(|texture_normal| {
                    let normal_color = texture_normal.color_at(self.texture_coordinate(&intersection.position));
                    intersection.normal = adjusted_for_normal(&intersection.normal, &color_to_normal(&normal_color));
                });
                
                intersection
            })
    }

//    #[flame("Plane")]
    fn texture_coordinate(&self, point: &Vec3) -> (f32,f32) {
        let point_projected_on_plane = self.projection(point);

        let proj_y = point_projected_on_plane.projected_on(&self.projected_bias);
        let difference_vec_y = &point_projected_on_plane - &proj_y;
        let u = difference_vec_y.x.signum() * difference_vec_y.len();

        let proj_x = point_projected_on_plane.projected_on(&self.rotated_projected_bias);
        let difference_vec_x = &point_projected_on_plane - &proj_x;
        let v = difference_vec_x.y.signum() * difference_vec_x.len();

        (u - u.floor(), v - v.floor())
    }

//    #[flame("Plane")]
    fn shade(&self, ray: &Ray, objs: &Vec<ObjectEnum>, rng: &mut SmallRng, bounces_remaining: u8) -> Illumination {
        let mut intersection = self.intersection(ray).unwrap();
        let uv = self.texture_coordinate(&intersection.position);

        self.material.shade(
            &mut intersection,
            uv,
            objs,
            rng,
            bounces_remaining
        )
    }
}

fn projection(origin: &Vec3, normal: &Vec3, point: &Vec3) -> Vec3 {
    point - &point.projected_on(&(origin + normal))
}