#![allow(unused_doc_comments)]

extern crate rand;
use rand::{Rng,thread_rng};
use rand::rngs::ThreadRng;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use rand::RngCore;
extern crate image;
use image::{ImageBuffer, Rgba};
extern crate crossbeam;
#[macro_use]
extern crate lazy_static;

use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use std::io::Write;
use std::time::{Duration,Instant};

mod color;
mod fidelity_consts;
mod frame;
mod illumination_memoization;
mod illumination;
mod intersection;
mod material;
mod matrix;
mod object;
mod plane;
mod ray;
mod sphere;
mod texture_checkered;
mod texture_solid;
mod texture;
mod utils;
mod vec3;
mod timing;

use color::Color;
use fidelity_consts::{RESOLUTION,SAMPLE_COUNT,MAX_DEPTH};
use frame::Frame;
use illumination_memoization::{find_memoized_illumination,memoize_illumination,print_memoization};
use illumination::{Illumination,integrate};
use intersection::Intersection;
use material::Material;
use utils::clamp;
use vec3::Vec3;
use ray::Ray;
use object::Object;
use sphere::Sphere;
use plane::Plane;
use texture_checkered::TextureCheckered;
use texture_solid::TextureSolid;
use timing::{start,stop,finish};


// fidelity/tuning
const CELLS: usize = 64; // must be the square of an integer

// misc
const BACKGROUND_ILLUMINATION: Illumination = Illumination { color: Color(0.0, 0.0, 0.0), intensity: 0.0 };
//const GLOBAL_LIGHT_DIRECTION: Vec3 = Vec3{ x: 1.0, y: 1.0, z: -1.0 };


fn main() {
    let ray_frame = ray_trace();
    write_image(&ray_frame);
}

// Do the thing!
fn ray_trace<'a>() -> Frame {
    
    println!("Tracing scene...");

    let start_time = Instant::now();
    
    // Create list of objects
    let objs = construct_scene();

    // Create frame
    let mut frame = Frame::new(RESOLUTION,RESOLUTION);
    let mut cells_done = 0;

    // Create thread wrappers
    let objs_arc: Arc<&Vec<Box<dyn Object + Sync + Send>>> = Arc::new(&objs);
    let frame_mutex_arc: Arc<Mutex<&mut Frame>> = Arc::new(Mutex::new(&mut frame));
    let cells_done_mutex_arc = Arc::new(Mutex::new(&mut cells_done));

    // ray_trace_cell(&mut frame, &objs, 0, 0, RESOLUTION, RESOLUTION);

    start("raytrace");

    crossbeam::scope(move |scope| {
        let row_column_count = (CELLS as f32).sqrt().round() as usize;
        let cell_size = RESOLUTION / row_column_count;

        print!("0.00%");
        std::io::stdout().flush().ok().expect("");

        for x_cell in 0..row_column_count {
            for y_cell in 0..row_column_count {
                let objs_arc_clone = objs_arc.clone();
                let frame_mutex_arc_clone = frame_mutex_arc.clone();
                let cells_done_mutex_arc_clone = cells_done_mutex_arc.clone();
                
                scope.spawn(move |_| {
                    ray_trace_cell(
                        frame_mutex_arc_clone, 
                        objs_arc_clone, 
                        x_cell * cell_size, 
                        y_cell * cell_size, 
                        (x_cell + 1) * cell_size, 
                        (y_cell + 1) * cell_size
                    );

                    let mut cells_done = cells_done_mutex_arc_clone.lock().unwrap();
                    **cells_done = (**cells_done) + 1;

                    print!("\r{}%           ", format!("{:.*}", 2, (**cells_done as f32 / CELLS as f32) * 100.0));
                    std::io::stdout().flush().ok().expect("");
                });
            }
        }
    }).unwrap();

    finish("raytrace");
    finish("cast ray");
    finish("cast ray -> find nearest");
    finish("cast ray -> other");
    finish("cast ray -> other -> rand gen");

    print_memoization();

    println!("Total time: {}s", Instant::now().duration_since(start_time).as_millis() as f32 / 1000.0);
    println!("done");

    return frame;
}

fn construct_scene() -> Vec<Box<dyn Object + Sync + Send>> {
    let mut objs: Vec<Box<dyn Object + Sync + Send>> = Vec::new();

    // lights
    /*
    objs.push(Box::new(Sphere::new(
        Vec3 {
            x: (rand::random::<u8>() % 10) as f32 - 5.0,
            y: (rand::random::<u8>() % 10) as f32 - 5.0,
            z: (rand::random::<u8>() % 10) as f32 - 15.0,
        },
        1.0,
        Material {
            texture_albedo: None,
            texture_specular: None,
            texture_emission: Some(Box::new(TextureSolid { color: Color(1.0, 0.0, 1.0) })),
        }
    )));*/
    objs.push(Box::new(Sphere::new(
        Vec3 { x: -3.0, y: -3.0, z: -12.0 },
        1.0,
        Material {
            texture_albedo: None,
            texture_specular: None,
            texture_emission: Some(Box::new(TextureSolid { color: Color(0.0, 1.0, 1.0) })),
        }
    )));

    // floor
    objs.push(Box::new(Plane::new(
        Vec3 { x: 0.0, y: -5.0, z: 0.0, },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        Material {
            texture_albedo: Some(Box::new(TextureSolid::new())),
            texture_specular: None,
            texture_emission: None,
        }
    )));

    // left wall
    objs.push(Box::new(Plane::new(
        Vec3 { x: -5.0, y: 0.0, z: 0.0, },
        Vec3 { x: 1.0, y: 0.0, z: 0.0 },
        Material {
            texture_albedo: Some(Box::new(TextureSolid::new())),
            texture_specular: None,
            texture_emission: None,
        }
    )));

    // right wall
    objs.push(Box::new(Plane::new(
        Vec3 { x: 5.0, y: 0.0, z: 0.0, },
        Vec3 { x: -1.0, y: 0.0, z: 0.0 },
        Material {
            texture_albedo: Some(Box::new(TextureSolid::new())),
            texture_specular: None,
            texture_emission: None,
        }
    )));

    // back wall
    objs.push(Box::new(Plane::new(
        Vec3 { x: 0.0, y: 0.0, z: -15.0, },
        Vec3 { x: 0.0, y: 0.0, z: 1.0 },
        Material {
            texture_albedo: Some(Box::new(TextureSolid::new())),
            texture_specular: None,
            texture_emission: None,
        }
    )));

    // spheres
    /*
    for _ in 0..10 {
        objs.push(Box::new(Sphere::new(
            Vec3 {
                x: (rand::random::<u8>() % 10) as f32 - 5.0,
                y: (rand::random::<u8>() % 10) as f32 - 5.0,
                z: (rand::random::<u8>() % 10) as f32 - 15.0,
            },
            1.0,
            Material {
                texture_albedo: Some(Box::new(TextureCheckered::new())),
                texture_specular: None,
                texture_emission: None,
            }
        )))
    }*/

    return objs;
}

/**
 * Raytrace one square sub-portion of the image (exists to facilitate threading)
 */
fn ray_trace_cell(frame_mutex: Arc<Mutex<&mut Frame>>, objs: Arc<&Vec<Box<dyn Object + Sync + Send>>>, min_x: usize, min_y: usize, max_x: usize, max_y: usize) {
    let mut rng = thread_rng();
    
    // Cast ray from each pixel
    for x in min_x..max_x {
        for y in min_y..max_y {
            let frame = frame_mutex.lock().unwrap();
            let ray = frame.pixel_to_ray(&(x, y));
            std::mem::drop(frame);

            let illumination = cast_ray(&ray, &objs, &mut rng, MAX_DEPTH);

            let mut frame = frame_mutex.lock().unwrap();
            frame.buffer[x as usize][y as usize] = illumination.color * clamp(illumination.intensity, 0.0, 1.2);
            std::mem::drop(frame);
        }
    }
}

/**
 * Cast a single ray, from a pixel or from a bounce
 */
fn cast_ray(ray: &Ray, objs: &Vec<Box<dyn Object + Sync + Send>>, rng: &mut ThreadRng, depth: u8) -> Illumination {
    start("cast ray");
    if depth <= 0 { return BACKGROUND_ILLUMINATION; }

    let mut nearest_intersection: Option<Intersection> = None;
    let mut nearest_object_index: Option<usize> = None;

    start("cast ray -> find nearest");

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
    stop("cast ray -> find nearest");

    start("cast ray -> other");

    // Compute total illumination at this intersection
    let nearest_illumination: Illumination = nearest_object_index
        .map(|object_index| {
            let nearest_object = &objs[object_index];
            let intersection = nearest_intersection.unwrap(); // if we have nearest_object_index, we have nearest_intersection

            find_memoized_illumination(object_index, &intersection.position)
                .unwrap_or({

                    // HACK: This is a weird relationship between Material and cast_ray; assumption is made that 
                    // if a texture exists, the corresponding illumination will be passed to shade(). Can
                    // probably be improved somehow.
                    let diffuse_illumination: Option<Illumination> = if nearest_object.get_material().texture_albedo.is_some() {
                        let mut samples = [Illumination::new();SAMPLE_COUNT];

                        let mut i = 0;
                        while i < SAMPLE_COUNT {
                            start("cast ray -> other -> rand gen");
                            
                            let ray = Ray {
                                origin: intersection.position,
                                direction: Vec3::from_angles(
                                    rng.gen_range(0.0, PI * 2.0),
                                    rng.gen_range(0.0, PI * 2.0),
                                )
                            };
                            stop("cast ray -> other -> rand gen");

                            // HACK: Figure out a way to *generate* rays that are already within our desired area
                            if valid_diffuse_sample(&intersection, &ray) {
                                // recurse
                                stop("cast ray -> other");
                                samples[i] = cast_ray(&ray, objs, rng, depth - 1);
                                start("cast ray -> other");

                                i += 1;
                            }
                        }

                        let illumination = integrate(&samples);

                        Some(illumination)
                    } else {
                        None
                    };

                    let specular_illumination: Option<Illumination> = None;/*if nearest_object.get_material().texture_specular.is_some() {
                        let mut samples = [Illumination::new();SAMPLE_COUNT];

                        let mut i = 0;
                        while i < SAMPLE_COUNT {
                            start("cast ray -> other -> rand gen");
                            
                            let ray = Ray {
                                origin: intersection.position,
                                direction: Vec3::from_angles(
                                    rng.gen_range(0.0, PI * 2.0),
                                    rng.gen_range(0.0, PI * 2.0),
                                )
                            };
                            stop("cast ray -> other -> rand gen");

                            // HACK: Figure out a way to *generate* rays that are already within our desired area
                            if valid_specular_sample(&intersection, &ray) {
                                // recurse
                                stop("cast ray -> other");
                                samples[i] = cast_ray(&ray, objs, rng, depth - 1);
                                start("cast ray -> other");

                                i += 1;
                            }
                        }

                        let illumination = integrate(&samples);

                        Some(illumination)
                    } else {
                        None
                    };*/

                    let uv = nearest_object.texture_coordinate(&intersection.position);

                    let pos = intersection.position.clone();

                    let illumination = nearest_object.get_material().shade(
                        &intersection, 
                        uv,
                        &diffuse_illumination, 
                        &specular_illumination
                    );

                    memoize_illumination(object_index, pos, illumination.clone());

                    illumination
                })
        })
        .unwrap_or(BACKGROUND_ILLUMINATION);

    stop("cast ray -> other");
    stop("cast ray");
    return nearest_illumination;
}

fn valid_diffuse_sample(intersection: &Intersection, sample_ray: &Ray) -> bool {
    //                                            angle < PI / 2.0
    sample_ray.direction.angle_to(&intersection.normal) * 2.0 < PI
}

/*
fn valid_specular_sample(intersection: &Intersection, sample_ray: &Ray) -> bool {
    // HACK: Factor in an actual "smoothness" value instead of PI / 4.0
    sample_ray.direction.angle_to(&intersection.normal) * 4.0 < PI
}*/

/**
 * Write a frame to a PNG file
 */
fn write_image(ray_frame: &Frame) {
    println!("Writing to png...");

    let mut image: ImageBuffer::<Rgba<u8>,Vec<u8>> = ImageBuffer::new(RESOLUTION as u32, RESOLUTION as u32);

    for x in 0..RESOLUTION {
        for y in 0..RESOLUTION {
            let color = ray_frame.buffer[x][y];
            image.get_pixel_mut(x as u32, y as u32).data = color.to_u8();
        }
    }

    image.save("output.png").unwrap();

    println!("done");
}
