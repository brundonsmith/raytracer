#![allow(unused_doc_comments)]

extern crate rand;
use rand::Rng;
extern crate image;
use image::{ImageBuffer, Rgb};
extern crate crossbeam;

use std::f32::consts::PI;
use std::sync::{Arc, Mutex};

mod color;
mod illumination;
mod intersection;
mod material;
mod matrix;
mod object;
mod ray;
mod sphere;
mod texture_blank;
mod texture_checkered;
mod texture;
mod utils;
mod vec3;

use color::Color;
use illumination::{Illumination,integrate};
use material::{Material,ShadingType};
use vec3::Vec3;
use ray::Ray;
use sphere::Sphere;
use object::{Object};
use texture_checkered::TextureCheckered;
use texture_blank::TextureBlank;

const RESOLUTION: usize = 256;
const SAMPLE_COUNT: usize = 256;
const MAX_DEPTH: u8 = 2;
const THREADS: usize = 4;

const CAMERA_POSITION: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
const FOCAL_LENGTH: f32 = 2.0;
const CAMERA_WIDTH: f32 = 2.0;
const CAMERA_HEIGHT: f32 = 2.0;
const CAMERA_TOP_LEFT: Vec3 = Vec3 { 
    x: CAMERA_POSITION.x - CAMERA_WIDTH / 2.0, 
    y: CAMERA_POSITION.y + CAMERA_HEIGHT / 2.0, 
    z: CAMERA_POSITION.z - FOCAL_LENGTH
};

const LIGHT_DIRECTION: Vec3 = Vec3{ x: 1.0, y: 1.0, z: -1.0 };

const BACKGROUND_COLOR: Color = Color(0.0, 0.0, 0.0);

struct Frame {
    pub buffer: Vec<Vec<Color>>,
    units_per_pixel_x: f32,
    units_per_pixel_y: f32,
}

impl Frame {

    pub fn new(width: usize, height: usize) -> Self {
        Frame { 
            buffer: vec![vec![Color(0.0, 0.0, 0.0); height]; width],
            units_per_pixel_x: CAMERA_WIDTH / width as f32,
            units_per_pixel_y: CAMERA_HEIGHT / height as f32
        }
    }

    pub fn units_per_pixel_x(&self) -> f32 { self.units_per_pixel_x }
    pub fn units_per_pixel_y(&self) -> f32 { self.units_per_pixel_y }
    
    /**
     * Find the world position of a pixel in this frame.
     */
    pub fn pixel_to_world(&self, pixel: &(usize,usize)) -> Vec3 {
        Vec3 { 
            x: CAMERA_TOP_LEFT.x + (0.5 + pixel.0 as f32) * self.units_per_pixel_x(),
            y: CAMERA_TOP_LEFT.y - (0.5 + pixel.1 as f32) * self.units_per_pixel_y(),
            z: CAMERA_TOP_LEFT.z
        }
    }

    /**
     * Initialize a ray projecting out from one pixel in this frame 
     * (incorporating not only origin position, but direction from 
     * the focal point).
     */
    pub fn pixel_to_ray(&self, pixel: &(usize,usize)) -> Ray {
        let pixel_position = self.pixel_to_world(&pixel);
        let mut direction = &pixel_position - &CAMERA_POSITION;
        direction.normalize();

        Ray {
            //ray_type: RayType::Primary,
            origin: pixel_position,
            direction: direction
        }
    }
}


fn ray_trace<'a>() -> Frame {
    
    println!("Tracing scene...");
    
    // Create list of objects
    let mut objs: Vec<Box<dyn Object + Sync + Send>> = Vec::new();
    
    // light
    objs.push(Box::new(Sphere {
        position: Vec3 {
            x: (rand::random::<u8>() % 10) as f32 - 5.0,
            y: (rand::random::<u8>() % 10) as f32 - 5.0,
            z: (rand::random::<u8>() % 10) as f32 - 15.0,
        },
        radius: 1.0,
        material: Material {
            shading_type: ShadingType::Light,
            texture: Box::new(TextureBlank::new(Color(1.0, 0.0, 0.0))),
        }
    }));
    
    // spheres
    for _ in 0..10 {
        objs.push(Box::new(Sphere {
            position: Vec3 {
                x: (rand::random::<u8>() % 10) as f32 - 5.0,
                y: (rand::random::<u8>() % 10) as f32 - 5.0,
                z: (rand::random::<u8>() % 10) as f32 - 15.0,
            },
            radius: 1.0,
            material: Material {
                shading_type: ShadingType::Diffuse,
                texture: Box::new(TextureCheckered::new()),
            }
        }))
    }

    // Create frame
    let mut ray_frame = Frame::new(RESOLUTION,RESOLUTION);
    let mutex_ray_frame: Arc<Mutex<&mut Frame>> = Arc::new(Mutex::new(&mut ray_frame));

    ray_trace_cell(&mut ray_frame, &objs, 0, 0, RESOLUTION, RESOLUTION);

    /*


    crossbeam::scope(move |scope| {
        let cell_size = RESOLUTION / THREADS;
        for x_cell in 0..cell_size {
            for y_cell in 0..cell_size {
                let x_min = x_cell * cell_size;
                let y_min = y_cell * cell_size;
                let x_max = x_min + cell_size;
                let y_max = y_min + cell_size;
                
                let rau_frame_ref = &ray_frame;
                let ray_frame_mutex_ref = Arc::clone(&mutex_ray_frame);
                
                scope.spawn(move |_| {
                    ray_trace_cell()
                });
            }
        }
    }).unwrap();
    */

    println!("done");

    return ray_frame;
}

fn ray_trace_cell(frame: &mut Frame, objs: &Vec<Box<dyn Object + Sync + Send>>, min_x: usize, min_y: usize, max_x: usize, max_y: usize) {
    // Cast ray from each pixel
    for x in min_x..max_x {
        for y in min_y..max_y {

            // HACK: Too lazy to guarantee this right now
            if x < RESOLUTION && y < RESOLUTION {
                let ray = frame.pixel_to_ray(&(x, y));
                let illumination = cast_ray(&ray, &objs, MAX_DEPTH);

                frame.buffer[x as usize][y as usize] = Color(
                    illumination.intensity,
                    illumination.intensity,
                    illumination.intensity
                );

                print!("\r{}% done           ", ((y + x * RESOLUTION) as f32 / (max_x * max_y) as f32) * 100.0);
            }
        }
    }
}

fn cast_ray(ray: &Ray, objs: &Vec<Box<dyn Object + Sync + Send>>, depth: u8) -> Illumination {
    let mut nearest_distance = std::f32::INFINITY;
    let mut nearest_illumination = Illumination { color: Color(0.0, 0.0, 0.0), intensity: 0.0 };
    let mut rng = rand::thread_rng();

    if depth <= 0 { return nearest_illumination; }

    //let light_position = objs.iter().find(|x| x.get_material().shading_type == ShadingType::Light).unwrap().get_position();

    /**
     * Find the nearest object intersection for this ray, and then shade it.
     */
    for obj in objs {
        match obj.intersection(&ray) {
            Some(intersection) => {
                // TODO: Optimize by doing shading only once, after looping
                if intersection.distance < nearest_distance {
                    nearest_distance = intersection.distance;

                    if obj.get_material().shading_type == ShadingType::Light {
                        //println!("Found light!");
                    }

                    nearest_illumination = match obj.get_material().shading_type {
                        ShadingType::Diffuse => {
                            //println!("Diffuse");
                            
                            let mut samples: Vec<Illumination> = Vec::with_capacity(SAMPLE_COUNT);
                            while samples.len() < SAMPLE_COUNT {
                                let ray = Ray {
                                    origin: intersection.position,
                                    direction: Vec3::from_angles(
                                        rng.gen_range(0.0, 1.0) * PI * 2.0, 
                                        rng.gen_range(0.0, 1.0) * PI * 2.0
                                    )
                                };

                                // HACK: Figure out a way to *generate* rays that are already within our desired area
                                if ray.direction.angle_to(&intersection.normal) < (PI / 2.0) {
                                    //println!("Sampled");
                                    
                                    // recurse
                                    samples.push(cast_ray(&ray, objs, depth - 1));
                                }
                            }

                            let mut illumination = integrate(samples.iter());
                            //illumination.intensity *= 0.5; // HACK: Roughness. Need to represent this somehow.

                            return illumination;
                        },
                        ShadingType::Light => Illumination { color: Color(1.0, 1.0, 1.0), intensity: 30.0 },
                        _ => Illumination { color: Color(1.0, 0.0, 1.0), intensity: 1.0 }, // magenta; material "error"
                    }
                    
                    //let light_direction = &intersection.position - light_position;
                    //let incident = clamp((&light_direction * -1.0).angle_to(&intersection.normal) / (PI / 2.0), 0.0, 1.0);
                    /*
                    let incident = clamp((&LIGHT_DIRECTION * -1.0).angle_to(&intersection.normal) / (PI / 2.0), 0.0, 1.0);
                    
                    nearest_illumination = Illumination { color: obj.get_material().shade(
                        incident,
                        Illumination { color: Color(1.0, 1.0, 1.0), intensity: 1.0 }, 
                        obj.texture_coordinate(&intersection.position)
                    ), intensity: 1.0 };*/
                }
            },
            _ => ()
        }
    }

    return nearest_illumination;
}

/*
let incident_angle = intersection.incident_angle();
let normal_shade = incident_angle / (std::f32::consts::PI / 2.0);

const MAX_DISTANCE: f32 = 60.0; // objects this far away are black
let distance_shade = 1.0 - intersection.distance / MAX_DISTANCE;

let uv = obj.texture_coordinate(&intersection.position);
let checker_color = obj.get_material().texture.color_at(uv.0, uv.1);
// HACK
let checker_shade = checker_color.0 as f32 / 255.0;

nearest_illumination = Color(255,255,255) * (checker_shade * distance_shade * normal_shade);
*/


fn main() {
    let ray_frame = ray_trace();
    write_image(&ray_frame);
}

/**
 * Write a frame to a PNG file
 */
fn write_image(ray_frame: &Frame) {
    println!("Writing to png...");

    let mut image: ImageBuffer::<Rgb<u8>,Vec<u8>> = ImageBuffer::new(RESOLUTION as u32, RESOLUTION as u32);

    for x in 0..RESOLUTION {
        for y in 0..RESOLUTION {
            let color = ray_frame.buffer[x][y];
            image.get_pixel_mut(x as u32, y as u32).data = color.to_u8();
        }
    }

    image.save("output.png").unwrap();

    println!("done");
}
