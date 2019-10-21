#![allow(unused_doc_comments)]

extern crate rand;
use rand::Rng;
extern crate image;
use image::{ImageBuffer, Rgb};
extern crate crossbeam;

use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use std::io::Write;

mod color;
mod frame;
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

use color::Color;
use frame::Frame;
use illumination::{Illumination,integrate};
use intersection::Intersection;
use material::Material;
use vec3::Vec3;
use ray::Ray;
use object::Object;
use sphere::Sphere;
use plane::Plane;
use texture_checkered::TextureCheckered;
use texture_solid::TextureSolid;


// fidelity/tuning
const RESOLUTION: usize = 128;
const SAMPLE_COUNT: usize = 128;
const MAX_DEPTH: u8 = 2;
const CELLS: usize = 16; // must be the square of an integer

// misc
const BACKGROUND_ILLUMINATION: Illumination = Illumination { color: Color(0.0, 0.0, 0.0), intensity: 0.0 };
//const GLOBAL_LIGHT_DIRECTION: Vec3 = Vec3{ x: 1.0, y: 1.0, z: -1.0 };


fn ray_trace<'a>() -> Frame {
    
    println!("Tracing scene...");
    
    // Create list of objects
    let mut objs: Vec<Box<dyn Object + Sync + Send>> = Vec::new();
    
    // lights
    objs.push(Box::new(Sphere {
        position: Vec3 {
            x: (rand::random::<u8>() % 10) as f32 - 5.0,
            y: (rand::random::<u8>() % 10) as f32 - 5.0,
            z: (rand::random::<u8>() % 10) as f32 - 15.0,
        },
        radius: 1.0,
        material: Material {
            texture_albedo: None,
            texture_specular: None,
            texture_emission: Some(Box::new(TextureSolid { color: Color(1.0, 0.0, 1.0) })),
        }
    }));
    objs.push(Box::new(Sphere {
        position: Vec3 {
            x: (rand::random::<u8>() % 10) as f32 - 5.0,
            y: (rand::random::<u8>() % 10) as f32 - 5.0,
            z: (rand::random::<u8>() % 10) as f32 - 15.0,
        },
        radius: 1.0,
        material: Material {
            texture_albedo: None,
            texture_specular: None,
            texture_emission: Some(Box::new(TextureSolid { color: Color(0.0, 1.0, 1.0) })),
        }
    }));

    // floor
    objs.push(Box::new(Plane {
        position: Vec3 { x: 0.0, y: -5.0, z: 0.0, },
        normal: Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        material: Material {
            texture_albedo: Some(Box::new(TextureSolid::new())),
            texture_specular: None,
            texture_emission: None,
        }
    }));
    
    // left wall
    objs.push(Box::new(Plane {
        position: Vec3 { x: -5.0, y: 0.0, z: 0.0, },
        normal: Vec3 { x: 1.0, y: 0.0, z: 0.0 },
        material: Material {
            texture_albedo: Some(Box::new(TextureSolid::new())),
            texture_specular: None,
            texture_emission: None,
        }
    }));

    // right wall
    objs.push(Box::new(Plane {
        position: Vec3 { x: 5.0, y: 0.0, z: 0.0, },
        normal: Vec3 { x: -1.0, y: 0.0, z: 0.0 },
        material: Material {
            texture_albedo: Some(Box::new(TextureSolid::new())),
            texture_specular: None,
            texture_emission: None,
        }
    }));

    // back wall
    objs.push(Box::new(Plane {
        position: Vec3 { x: 0.0, y: 0.0, z: -15.0, },
        normal: Vec3 { x: 0.0, y: 0.0, z: 1.0 },
        material: Material {
            texture_albedo: Some(Box::new(TextureSolid::new())),
            texture_specular: None,
            texture_emission: None,
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
                texture_albedo: Some(Box::new(TextureCheckered::new())),
                texture_specular: None,
                texture_emission: None,
            }
        }))
    }

    // Create frame
    let mut frame = Frame::new(RESOLUTION,RESOLUTION);
    let mut cells_done = 0;

    // Create thread wrappers
    let objs_arc: Arc<&Vec<Box<dyn Object + Sync + Send>>> = Arc::new(&objs);
    let frame_mutex_arc: Arc<Mutex<&mut Frame>> = Arc::new(Mutex::new(&mut frame));
    let cells_done_mutex_arc = Arc::new(Mutex::new(&mut cells_done));

    let start = std::time::SystemTime::now();

    // ray_trace_cell(&mut frame, &objs, 0, 0, RESOLUTION, RESOLUTION);


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
    
    let end = std::time::SystemTime::now();
    let total = end.duration_since(start).unwrap();
    println!("Took {}s", total.as_millis() as f32 / 1000.0);

    println!("done");

    return frame;
}

fn ray_trace_cell(frame_mutex: Arc<Mutex<&mut Frame>>, objs: Arc<&Vec<Box<dyn Object + Sync + Send>>>, min_x: usize, min_y: usize, max_x: usize, max_y: usize) {
    // Cast ray from each pixel
    for x in min_x..max_x {
        for y in min_y..max_y {

            // HACK: Too lazy to guarantee this right now
            if x < RESOLUTION && y < RESOLUTION {

                let frame = frame_mutex.lock().unwrap();
                let ray = frame.pixel_to_ray(&(x, y));
                std::mem::drop(frame);

                let illumination = cast_ray(&ray, &objs, MAX_DEPTH);

                let mut frame = frame_mutex.lock().unwrap();
                frame.buffer[x as usize][y as usize] = illumination.color * clamp(illumination.intensity, 0.0, 1.2);
                std::mem::drop(frame);
            }
        }
    }
}

fn cast_ray(ray: &Ray, objs: &Vec<Box<dyn Object + Sync + Send>>, depth: u8) -> Illumination {
    if depth <= 0 { return BACKGROUND_ILLUMINATION; }

    let mut nearest_intersection: Option<Intersection> = None;
    let mut nearest_object_index: Option<usize> = None;
    let mut rng = rand::thread_rng();

    /**
     * Find the nearest object intersection for this ray, and then shade it.
     */
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

    let nearest_illumination = nearest_intersection.map(|intersection| {
        let obj = nearest_object_index.map(|index| &objs[index]).unwrap();
        let uv = obj.texture_coordinate(&intersection.position);

        // HACK: This is a weird relationship between Material and cast_ray; assumption is made that 
        // if a texture exists, the corresponding illumination will be passed to shade(). Can
        // probably be improved somehow.
        let diffuse_illumination: Option<Illumination> = if obj.get_material().texture_albedo.is_some() {
            let mut samples = [Illumination::new();SAMPLE_COUNT];

            let mut i = 0;
            while i < SAMPLE_COUNT {

                let ray = Ray {
                    origin: intersection.position,
                    direction: Vec3::from_angles(
                        rng.gen_range(0.0, 1.0) * PI * 2.0, 
                        rng.gen_range(0.0, 1.0) * PI * 2.0,
                    )
                };

                // HACK: Figure out a way to *generate* rays that are already within our desired area
                if ray.direction.angle_to(&intersection.normal) < (PI / 2.0) {
                    samples[i] = cast_ray(&ray, objs, depth - 1);
                    //start("cast ray -> other");
                    
                    i += 1;
                }
            }

            let illumination = integrate(samples.iter());

            Some(illumination)
        } else {
            None
        };

        let specular_illumination = match &obj.get_material().texture_specular {
            Some(_) => None, // TODO
            None => None,
        };

        obj.get_material().shade(
            intersection, 
            uv, 
            diffuse_illumination, 
            specular_illumination
        )
    });

    return nearest_illumination.unwrap_or(BACKGROUND_ILLUMINATION);
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
