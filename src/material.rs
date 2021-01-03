use std::f32::consts::PI;

use rand::rngs::SmallRng;
//use flamer::flame;

use crate::{fidelity_consts::{SAMPLE_COUNT_ANGULAR, SAMPLE_COUNT_RADIAL}, illumination::{Illumination,integrate}, matrix::Matrix, utils::TWO_PI};
use crate::texture::Texture;
use crate::color::Color;
use crate::vec3::Vec3;
use crate::intersection::Intersection;
use crate::cast::{cast_ray};
use crate::fidelity_consts::{SAMPLE_COUNT,PREVIEW_MODE};
use crate::ray::Ray;
use crate::utils::{PI_OVER_TWO};
use crate::object::{ObjectEnum};


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
    pub fn shade(&self, intersection: &mut Intersection, uv: (f32,f32), objs: &Vec<ObjectEnum>, rng: &mut SmallRng, bounces_remaining: u8) -> Illumination {
        match &self.texture_emission_intensity {
            Some(tex) => Illumination {
                color: self.texture_emission_color.as_ref().map(|col| col.color_at(uv))
                        .unwrap_or(Color(1.0, 1.0, 1.0)),
                intensity: tex.color_at(uv).0
            },
            None => {
                if bounces_remaining == 0 {
                    BACKGROUND_ILLUMINATION
                } else if PREVIEW_MODE {
                    let base_color = self.texture_albedo.as_ref().map(|texture| texture.color_at(uv)).unwrap_or(Color(1.0, 1.0, 1.0));
                    let adjustment = 1.0 - (intersection.normal.angle(&PREVIEW_DIRECTION) / PI);
                    Illumination {
                        color: base_color * adjustment,
                        intensity: 1.0
                    }
                } else {
                    let diffuse_illumination: Option<Illumination> = self.texture_albedo.as_ref().map(|texture| {
                        let surface_color = texture.color_at(uv);
                        let sample_rays = get_sample_rays(intersection.position, &intersection.normal, rng, PI_OVER_TWO);

                        let mut samples = [Illumination::new();SAMPLE_COUNT];
                        for i in 0..SAMPLE_COUNT {
                            samples[i] = cast_ray(&sample_rays[i], objs, rng, bounces_remaining - 1);
                        }

                        let illumination = integrate(&samples);
                        
                        Illumination {
                            color: surface_color * illumination.color,
                            intensity: illumination.intensity
                        }
                    });

                    let specular_illumination: Option<Illumination> = self.texture_specular.as_ref().map(|texture| {
                        let specularity = texture.color_at(uv).0;

                        if specularity > 0.99 {
                            // if reflection is nearly perfect, just cast a single sample ray to avoid work
                            return cast_ray(&Ray {
                                origin: intersection.position,
                                direction: *intersection.reflected_direction()
                            }, objs, rng, bounces_remaining - 1);
                        } else {
                            let reflected = intersection.reflected_direction().clone();
                            let sample_rays = get_sample_rays(intersection.position, &reflected, rng, (1.0 - specularity) * PI_OVER_TWO);

                            let mut samples = [Illumination::new();SAMPLE_COUNT];
                            for i in 0..SAMPLE_COUNT {
                                samples[i] = cast_ray(&sample_rays[i], objs, rng, bounces_remaining - 1);
                            }
                            
                            let illumination = integrate(&samples);

                            return illumination;
                        }
                    });

                    match diffuse_illumination {
                        Some(diffuse) => {
                            match specular_illumination {
                                Some(specular_illumination) => 
                                    Illumination::combined(&diffuse, &specular_illumination),
                                None => diffuse
                            }
                        },
                        None => {
                            match specular_illumination {
                                Some(specular_illumination) => specular_illumination,
                                None => BACKGROUND_ILLUMINATION
                            }
                        }
                    }
                }
            }
        }
    }
}

fn get_sample_rays(position: Vec3, direction: &Vec3, rng: &mut SmallRng, range: f32) -> [Ray;SAMPLE_COUNT] {
    let mut rays = [Ray::new();SAMPLE_COUNT];
    
    let mut i = 0;
    while i < SAMPLE_COUNT {
        let ray = Ray::random_direction(position, rng);

        // HACK: Figure out a way to *generate* rays that are already within our desired area
        if ray.direction.angle(direction) < range {
            rays[i] = ray;
            i += 1;
        }
    }

    return rays;
}
