
use rand::rngs::SmallRng;
//use flamer::flame;

use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::illumination::{Illumination};
use crate::color::Color;
use crate::object::{Object,ObjectEnum};

// misc
const BACKGROUND_ILLUMINATION: Illumination = Illumination { color: Color(0.0, 0.0, 0.0), intensity: 0.0 };
//const GLOBAL_LIGHT_DIRECTION: Vec3 = Vec3{ x: 1.0, y: 1.0, z: -1.0 };


/**
 * Cast a single ray, from a pixel or from a bounce
 */
//#[flame]
pub fn cast_ray(ray: &Ray, objs: &Vec<ObjectEnum>, rng: &mut SmallRng, bounces_remaining: u8) -> Illumination {
    let mut nearest_intersection: Option<Intersection> = None;
    let mut nearest_object: Option<&ObjectEnum> = None;


    // Find nearest object intersection
    for index in 0..objs.len() {
        if let Some(intersection) = objs[index].intersection(&ray) {
            if intersection.distance < nearest_intersection.as_ref().map(|int| int.distance).unwrap_or(std::f32::INFINITY) {
                nearest_intersection = Some(intersection);
                nearest_object = Some(&objs[index]);
            }
        }
    }

    // Compute total illumination at this intersection
    let nearest_illumination: Illumination = nearest_object
        .map(|obj| obj.shade(ray, objs, rng, bounces_remaining))
        .unwrap_or(BACKGROUND_ILLUMINATION);

    return nearest_illumination;
}
