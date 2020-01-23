use std::f32::consts::PI;

use rand::rngs::SmallRng;
//use flamer::flame;

use crate::illumination::{Illumination,integrate};
use crate::texture::Texture;
use crate::color::Color;
use crate::vec3::Vec3;
use crate::intersection::Intersection;
use crate::cast::{cast_ray};
use crate::fidelity_consts::{SAMPLE_COUNT,PREVIEW_MODE};
use crate::ray::Ray;
use crate::utils::{ObjectVec,PI_OVER_TWO};

const BACKGROUND_ILLUMINATION: Illumination = Illumination { color: Color(0.0, 0.0, 0.0), intensity: 0.0 };

pub struct Material {
    pub texture_albedo: Option<Texture>,
    pub texture_specular: Option<Texture>,
    pub texture_normal: Option<Texture>,
    pub texture_emission_color: Option<Texture>,
    pub texture_emission_intensity: Option<Texture>,
}

const PREVIEW_DIRECTION: Vec3 = Vec3 { x: 1.0, y: 1.0, z: 1.0 };

impl Material {

    pub fn new() -> Self {
        Self {
            texture_albedo: None,
            texture_specular: None,
            texture_normal: None,
            texture_emission_color: None,
            texture_emission_intensity: None,
        }
    }

//    #[flame("Material")]
    pub fn shade(&self, intersection: &mut Intersection, uv: (f32,f32), objs: &ObjectVec, rng: &mut SmallRng, depth: u8) -> Illumination {
        match &self.texture_emission_intensity {
            Some(tex) => Illumination {
                color: self.texture_emission_color.as_ref().map(|col| col.color_at(uv))
                        .unwrap_or(Color(1.0, 1.0, 1.0)),
                intensity: tex.color_at(uv).0
            },
            None => {
                if PREVIEW_MODE {
                    let base_color = self.texture_albedo.as_ref().map(|texture| texture.color_at(uv)).unwrap_or(Color(1.0, 1.0, 1.0));
                    let adjustment = 1.0 - (intersection.normal.angle(&PREVIEW_DIRECTION) / PI);
                    Illumination {
                        color: base_color * adjustment,
                        intensity: 1.0
                    }
                } else {
                    let diffuse_illumination: Option<Illumination> = self.texture_albedo.as_ref().map(|texture| {
                        let surface_color = texture.color_at(uv);
                        let sample_rays = get_sample_rays(intersection, valid_diffuse_sample, rng, PI_OVER_TWO);

                        let mut samples = [Illumination::new();SAMPLE_COUNT];
                        for i in 0..SAMPLE_COUNT {
                            samples[i] = cast_ray(&sample_rays[i], objs, rng, depth - 1);
                        }

                        let illumination = integrate(&samples);
                        
                        Illumination {
                            color: surface_color * illumination.color,
                            intensity: illumination.intensity
                        }
                    });

                    let specular_illumination: Option<(Illumination,f32)> = self.texture_specular.as_ref().map(|texture| {
                        let specularity = texture.color_at(uv).0;

                        if specularity > 0.99 {
                            // if reflection is nearly perfect, just cast a single sample ray to avoid work
                            return (cast_ray(&Ray {
                                origin: intersection.position,
                                direction: intersection.reflected_direction().clone()
                            }, objs, rng, depth - 1), specularity);
                        } else {
                            let sample_rays = get_sample_rays(intersection, valid_specular_sample, rng, (1.0 - specularity) * PI_OVER_TWO);

                            let mut samples = [Illumination::new();SAMPLE_COUNT];
                            for i in 0..SAMPLE_COUNT {
                                samples[i] = cast_ray(&sample_rays[i], objs, rng, depth - 1);
                            }
                            
                            let illumination = integrate(&samples);

                            return (illumination,specularity);
                        }
                    });

                    match diffuse_illumination {
                        Some(diffuse) => {
                            match specular_illumination {
                                Some((illumination, specularity)) => 
                                    Illumination::combined(&diffuse, &illumination, specularity),
                                None => diffuse
                            }
                        },
                        None => {
                            match specular_illumination {
                                Some((illumination, _specularity)) => illumination,
                                None => BACKGROUND_ILLUMINATION
                            }
                        }
                    }
                }
            }
        }
    }
}

fn valid_diffuse_sample(intersection: &mut Intersection, sample_ray: &Ray, range: f32) -> bool {
    sample_ray.direction.angle(&intersection.normal) < range
}

fn valid_specular_sample(intersection: &mut Intersection, sample_ray: &Ray, range: f32) -> bool {
    sample_ray.direction.angle(&intersection.reflected_direction()) < range
}

fn get_sample_rays<F: Fn(&mut Intersection, &Ray, f32) -> bool>(intersection: &mut Intersection, predicate: F, rng: &mut SmallRng, range: f32) -> [Ray;SAMPLE_COUNT] {
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