
use rand::rngs::SmallRng;

use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::illumination::{Illumination};
use crate::color::Color;
use crate::fidelity_consts::{SAMPLE_COUNT};
use crate::utils::{ObjectVec};

// misc
const BACKGROUND_ILLUMINATION: Illumination = Illumination { color: Color(0.0, 0.0, 0.0), intensity: 0.0 };
//const GLOBAL_LIGHT_DIRECTION: Vec3 = Vec3{ x: 1.0, y: 1.0, z: -1.0 };


/**
 * Cast a single ray, from a pixel or from a bounce
 */
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


pub fn get_sample_rays<F: Fn(&mut Intersection, &Ray, f32) -> bool>(intersection: &mut Intersection, predicate: F, rng: &mut SmallRng, range: f32) -> [Ray;SAMPLE_COUNT] {
    let mut rays = [Ray::new();SAMPLE_COUNT];
    
    let mut i = 0;
    while i < SAMPLE_COUNT {
        let ray = Ray::random_direction(intersection.position, rng);
        /*
        let ray = Ray {
            origin: intersection.position,
            direction: DIRECTIONS[rng.gen_range(0, PRECOMPUTED_SAMPLE_DIRECTION_COUNT)]
        };*/

        // HACK: Figure out a way to *generate* rays that are already within our desired area
        if predicate(intersection, &ray, range) {
            rays[i] = ray;
            i += 1;
        }
    }
    

    /*
    double a = random() * TWO_PI
    double r = R * sqrt(random())

    // If you need it in Cartesian coordinates
    double x = r * cos(a)
    double y = r * sin(a)
    */
    /*
    for i in 0..SAMPLE_COUNT {
        let angle = rng.gen_range(0.0, TWO_PI);
        let radius = range * (rng.gen_range(0.0, 1.0) as f32).sqrt();
    
        let intersection_normal_angles = intersection.normal.angles();
    
        let alpha = intersection_normal_angles.0 + radius * angle.cos();
        let beta = intersection_normal_angles.1 + radius * angle.sin();
    
        rays[i] = Ray {
            origin: intersection.position.clone(),
            direction: Vec3::from_angles(alpha, beta)
        };
    }*/

    return rays;
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