
use rand::Rng;

use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::object::Object;
use crate::illumination::{Illumination};
use crate::color::Color;
use crate::fidelity_consts::{SAMPLE_COUNT};

// misc
const BACKGROUND_ILLUMINATION: Illumination = Illumination { color: Color(0.0, 0.0, 0.0), intensity: 0.0 };
//const GLOBAL_LIGHT_DIRECTION: Vec3 = Vec3{ x: 1.0, y: 1.0, z: -1.0 };


/**
 * Cast a single ray, from a pixel or from a bounce
 */
pub fn cast_ray<R: Rng>(ray: &Ray, objs: &Vec<Box<dyn Object + Sync + Send>>, rng: &mut R, depth: u8) -> Illumination {
    if depth <= 0 { return BACKGROUND_ILLUMINATION; }

    let mut nearest_intersection: Option<Intersection> = None;
    let mut nearest_object_index: Option<usize> = None;


    // Find nearest object intersection
    for index in 0..objs.len() {
        match objs[index].intersection(&ray) {
            Some(intersection) => {
                if intersection.distance < nearest_intersection.as_ref().map(|int| int.distance).unwrap_or(std::f32::INFINITY) {
                    nearest_intersection = Some(intersection);
                    nearest_object_index = Some(index);
                }
            },
            _ => ()
        }
    }

    // Compute total illumination at this intersection
    let nearest_illumination: Illumination = nearest_object_index
        .map(|object_index| {
            let nearest_object = &objs[object_index];
            let mut intersection = nearest_intersection.unwrap(); // if we have nearest_object_index, we have nearest_intersection
            let uv = nearest_object.texture_coordinate(&intersection.position);
            nearest_object.get_material().shade(&mut intersection, uv, objs, rng, depth)
        })
        .unwrap_or(BACKGROUND_ILLUMINATION);

    return nearest_illumination;
}


pub fn get_sample_rays<F: Fn(&mut Intersection, &Ray, f32) -> bool, R: Rng>(intersection: &mut Intersection, predicate: F, rng: &mut R, range: f32) -> [Ray;SAMPLE_COUNT] {
    let mut rays = [Ray::new();SAMPLE_COUNT];

    let mut i = 0;
    while i < SAMPLE_COUNT {        
        let ray = Ray::random_direction(intersection.position, rng);

        // HACK: Figure out a way to *generate* rays that are already within our desired area
        if predicate(intersection, &ray, range) {
            rays[i] = ray;
            i += 1;
        }
    }

    return rays;
}
