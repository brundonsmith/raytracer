
use std::f32::consts::PI;

use rand::rngs::ThreadRng;

use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::object::Object;
use crate::illumination::{Illumination,integrate};
use crate::color::Color;
use crate::fidelity_consts::{SAMPLE_COUNT};

// misc
const BACKGROUND_ILLUMINATION: Illumination = Illumination { color: Color(0.0, 0.0, 0.0), intensity: 0.0 };
//const GLOBAL_LIGHT_DIRECTION: Vec3 = Vec3{ x: 1.0, y: 1.0, z: -1.0 };


/**
 * Cast a single ray, from a pixel or from a bounce
 */
pub fn cast_ray(ray: &Ray, objs: &Vec<Box<dyn Object + Sync + Send>>, rng: &mut ThreadRng, depth: u8) -> Illumination {
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

            // HACK: This is a weird relationship between Material and cast_ray; assumption is made that 
            // if a texture exists, the corresponding illumination will be passed to shade(). Can
            // probably be improved somehow.
            let diffuse_illumination: Option<Illumination> = nearest_object.get_material().texture_albedo.as_ref().map(|_| {
                let sample_rays = get_sample_rays(&mut intersection, valid_diffuse_sample, rng, PI / 2.0);

                let mut samples = [Illumination::new();SAMPLE_COUNT];
                for i in 0..SAMPLE_COUNT {
                    samples[i] = cast_ray(&sample_rays[i], objs, rng, depth - 1);
                }

                let illumination = integrate(&samples);

                illumination
            });

            let uv = nearest_object.texture_coordinate(&intersection.position);

            let specular_illumination: Option<Illumination> = nearest_object.get_material().texture_specular.as_ref().map(|texture| {
                let specularity = texture.color_at(uv).0;
                
                if specularity > 0.99 {
                    return cast_ray(&Ray {
                        origin: intersection.position,
                        direction: intersection.reflected_direction().clone()
                    }, objs, rng, depth - 1);
                } else {
                    let sample_rays = get_sample_rays(&mut intersection, valid_specular_sample, rng, (1.0 - specularity) * PI / 2.0);

                let mut samples = [Illumination::new();SAMPLE_COUNT];
                for i in 0..SAMPLE_COUNT {
                    samples[i] = cast_ray(&sample_rays[i], objs, rng, depth - 1);
                }
                
                let illumination = integrate(&samples);

                return illumination;
                }
            });

            let illumination = nearest_object.get_material().shade(
                &intersection, 
                uv,
                &diffuse_illumination, 
                &specular_illumination
            );


            illumination
        })
        .unwrap_or(BACKGROUND_ILLUMINATION);

    return nearest_illumination;
}


fn get_sample_rays<F: Fn(&mut Intersection, &Ray, f32) -> bool>(intersection: &mut Intersection, predicate: F, rng: &mut ThreadRng, range: f32) -> [Ray;SAMPLE_COUNT] {
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

fn valid_diffuse_sample(intersection: &mut Intersection, sample_ray: &Ray, range: f32) -> bool {
    sample_ray.direction.angle(&intersection.normal) < range
}

fn valid_specular_sample(intersection: &mut Intersection, sample_ray: &Ray, range: f32) -> bool {
    sample_ray.direction.angle(&intersection.reflected_direction()) < range
}
