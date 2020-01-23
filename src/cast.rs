
use rand::rngs::SmallRng;
//use flamer::flame;

use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::illumination::{Illumination};
use crate::color::Color;
use crate::utils::{ObjectVec};

// misc
const BACKGROUND_ILLUMINATION: Illumination = Illumination { color: Color(0.0, 0.0, 0.0), intensity: 0.0 };
//const GLOBAL_LIGHT_DIRECTION: Vec3 = Vec3{ x: 1.0, y: 1.0, z: -1.0 };


/**
 * Cast a single ray, from a pixel or from a bounce
 */
//#[flame]
pub fn cast_ray(ray: &Ray, objs: &ObjectVec, rng: &mut SmallRng, depth: u8) -> Illumination {
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
            nearest_object.shade(ray, objs, rng, depth)
        })
        .unwrap_or(BACKGROUND_ILLUMINATION);

    return nearest_illumination;
}

/*
const PRECOMPUTED_SAMPLE_DIRECTION_COUNT: usize = 1024;

lazy_static! {
    static ref DIRECTIONS: [Vec3;PRECOMPUTED_SAMPLE_DIRECTION_COUNT] = generate_directions();
}

fn generate_directions() -> [Vec3;PRECOMPUTED_SAMPLE_DIRECTION_COUNT] {
    let mut arr = [Vec3::new();PRECOMPUTED_SAMPLE_DIRECTION_COUNT];

    let alpha_increments = (PRECOMPUTED_SAMPLE_DIRECTION_COUNT as f32 * 2.0 / 3.0) as usize;
    let rows = (PRECOMPUTED_SAMPLE_DIRECTION_COUNT as f32 / 3.0) as usize;

    for i in 0..PRECOMPUTED_SAMPLE_DIRECTION_COUNT {
        let alpha_increment = i % beta_increments;
        let beta_increment = i / beta_increments;

        arr[i] = Vec3::from_angles(
            (alpha_increment as f32 / alpha_increments as f32) * TWO_PI,
            (beta_increment as f32 / beta_increments as f32) * PI);
    }

    return arr;
}*/