#![allow(unused_doc_comments)]

extern crate rand;
use rand::{Rng,thread_rng};
use rand::rngs::ThreadRng;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use rand::RngCore;
extern crate image;
use image::{ImageBuffer, Rgb};
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
mod illumination;
mod intersection;
mod material;
mod matrix;
mod object;
mod plane;
mod ray;
mod scenes;
mod sphere;
mod texture_checkered;
mod texture_solid;
mod texture_image;
mod texture;
mod utils;
mod vec3;
mod timing;
mod mesh;
mod obj_parser;

use color::Color;
use fidelity_consts::{RESOLUTION,SAMPLE_COUNT,MAX_DEPTH,CELLS,PRECALCULATED_SAMPLES};
use frame::Frame;
use illumination::{Illumination,integrate};
use intersection::Intersection;
use utils::clamp;
use ray::Ray;
use scenes::{construct_reflect_scene,construct_room_scene,construct_image_texture_test};
use object::Object;
use timing::{start,stop,finish};

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
    let objs = construct_room_scene();

    // Create frame
    let mut frame = Frame::new(RESOLUTION,RESOLUTION);
    let mut cells_done = 0;

    // Create thread wrappers
    let frame_mutex_arc: Arc<Mutex<&mut Frame>> = Arc::new(Mutex::new(&mut frame));
    let objs_arc: Arc<&Vec<Box<dyn Object + Sync + Send>>> = Arc::new(&objs);
    let cells_done_mutex_arc = Arc::new(Mutex::new(&mut cells_done));

    // ray_trace_cell(&mut frame, &objs, 0, 0, RESOLUTION, RESOLUTION);

    //start("raytrace");

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

    println!("Total time: {}s", Instant::now().duration_since(start_time).as_millis() as f32 / 1000.0);
    println!("done");

    return frame;
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
            frame.set(x as usize, y as usize, illumination.color * clamp(illumination.intensity, 0.0, 1.0));
            std::mem::drop(frame);
        }
    }
}

/**
 * Cast a single ray, from a pixel or from a bounce
 */
fn cast_ray(ray: &Ray, objs: &Vec<Box<dyn Object + Sync + Send>>, rng: &mut ThreadRng, depth: u8) -> Illumination {
    //start("cast ray");
    if depth <= 0 { return BACKGROUND_ILLUMINATION; }

    let mut nearest_intersection: Option<Intersection> = None;
    let mut nearest_object_index: Option<usize> = None;

    //start("cast ray -> find nearest");

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
    //stop("cast ray -> find nearest");

    //start("cast ray -> other");

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

    //stop("cast ray -> other");
    //stop("cast ray");
    return nearest_illumination;
}


fn get_sample_rays<F: Fn(&mut Intersection, &Ray, f32) -> bool>(intersection: &mut Intersection, predicate: F, rng: &mut ThreadRng, range: f32) -> [Ray;SAMPLE_COUNT] {
    let mut rays = [Ray::new();SAMPLE_COUNT];

    let mut i = 0;
    while i < SAMPLE_COUNT {
        //start("cast ray -> other -> rand gen");
        
        let ray = Ray::random_direction(intersection.position, rng);
        //stop("cast ray -> other -> rand gen");

        // HACK: Figure out a way to *generate* rays that are already within our desired area
        if predicate(intersection, &ray, range) {
            //stop("cast ray -> other");
            rays[i] = ray;
            //start("cast ray -> other");

            i += 1;
        }
    }

    return rays;
}

fn valid_diffuse_sample(intersection: &mut Intersection, sample_ray: &Ray, range: f32) -> bool {
    //                                            angle < PI / 2.0
    sample_ray.direction.angle(&intersection.normal) < range
}

fn valid_specular_sample(intersection: &mut Intersection, sample_ray: &Ray, range: f32) -> bool {
    // HACK: Factor in an actual "smoothness" value instead of PI / 64.0
    sample_ray.direction.angle(&intersection.reflected_direction()) < range
}

/**
 * Write a frame to a PNG file
 */
fn write_image(ray_frame: &Frame) {
    println!("Writing to png...");

    let mut image: ImageBuffer::<Rgb<u8>,Vec<u8>> = ImageBuffer::new(RESOLUTION as u32, RESOLUTION as u32);

    for x in 0..RESOLUTION {
        for y in 0..RESOLUTION {
            let color = ray_frame.get(x, y);
            image.get_pixel_mut(x as u32, y as u32).0 = color.to_u8();
        }
    }

    image.save("output.png").unwrap();

    println!("done");
}
