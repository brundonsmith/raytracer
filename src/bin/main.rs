#![allow(unused_doc_comments)]

extern crate rand;
use rand::prelude::*;
use rand::{thread_rng};
use rand::rngs::SmallRng;
extern crate image;
use image::{ImageBuffer, Rgb};
extern crate crossbeam;

use std::sync::{Arc, Mutex};
use std::io::Write;
use std::time::{Instant};

use raytracer::fidelity_consts::{RESOLUTION,MAX_DEPTH,CELLS};
use raytracer::frame::Frame;
use raytracer::utils::clamp;
use raytracer::scenes::{
    construct_reflect_scene,
    construct_room_scene,
    construct_plane_texture_test,
    construct_sphere_texture_test,
    construct_wallpaper_scene,
    construct_wallpaper_scene_2,
    construct_tree_scene};
use raytracer::cast::cast_ray;
use raytracer::utils::{ObjectVec};


fn main() {
    let ray_frame = ray_trace();
    write_image(&ray_frame);
}

// Do the thing!
fn ray_trace<'a>() -> Frame {
    
    println!("Tracing scene...");

    let start_time = Instant::now();
    
    // Create list of objects
    let objs = construct_reflect_scene();

    // Create frame
    let mut frame = Frame::new(RESOLUTION,RESOLUTION);
    let mut cells_done = 0;

    // Create thread wrappers
    let frame_mutex_arc: Arc<Mutex<&mut Frame>> = Arc::new(Mutex::new(&mut frame));
    let objs_arc: Arc<&ObjectVec> = Arc::new(&objs);
    let cells_done_mutex_arc = Arc::new(Mutex::new(&mut cells_done));

    // ray_trace_cell(&mut frame, &objs, 0, 0, RESOLUTION, RESOLUTION);

    crossbeam::scope(move |scope| {
        let row_column_count = (CELLS as f32).sqrt().round() as usize;
        let cell_size = RESOLUTION / row_column_count;

        print!("0.00%");
        std::io::stdout().flush().ok().expect("");

        let mut meta_rng = thread_rng();

        for x_cell in 0..row_column_count {
            for y_cell in 0..row_column_count {
                let objs_arc_clone = objs_arc.clone();
                let frame_mutex_arc_clone = frame_mutex_arc.clone();
                let cells_done_mutex_arc_clone = cells_done_mutex_arc.clone();
                let rng = SmallRng::from_rng(&mut meta_rng).unwrap();
                
                scope.spawn(move |_| {
                    ray_trace_cell(
                        frame_mutex_arc_clone, 
                        objs_arc_clone, 
                        rng,
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

    println!("Total time: {}s", Instant::now().duration_since(start_time).as_millis() as f32 / 1000.0);
    println!("done");

    return frame;
}

/**
 * Raytrace one square sub-portion of the image (exists to facilitate threading)
 */
fn ray_trace_cell(frame_mutex: Arc<Mutex<&mut Frame>>, objs: Arc<&ObjectVec>, mut rng: SmallRng, min_x: usize, min_y: usize, max_x: usize, max_y: usize) {
    
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
