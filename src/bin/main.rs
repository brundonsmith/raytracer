#![allow(unused_doc_comments)]

use std::fs::File;

extern crate rand;
use rand::prelude::*;
use rand::{thread_rng};
use rand::rngs::SmallRng;
extern crate image;
use image::{ImageBuffer, Rgb};
extern crate crossbeam;
//extern crate flame;
#[macro_use]
//extern crate flamer;

use std::sync::{Arc, Mutex};
use std::io::Write;
use std::time::{Instant};

use raytracer::fidelity_consts::{RESOLUTION,BOUNCES,THREADS,TOTAL_BUFFER_SIZE,PIXELS_PER_THREAD};
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
use raytracer::color::Color;
use raytracer::object::ObjectEnum;


fn main() {
    let ray_frame = ray_trace();
    write_image(&ray_frame);
}

// Do the thing!
fn ray_trace<'a>() -> Frame {
    
    println!("Tracing scene...");

    let start_time = Instant::now();
    
    // Create list of objects
    let objs: Vec<ObjectEnum> = construct_tree_scene();

    // Create frame
    let mut frame = Frame::new();
    let mut threads_done = 0;

    // Create thread wrappers
    let frame_mutex_arc: Arc<Mutex<&mut Frame>> = Arc::new(Mutex::new(&mut frame));
    let objs_arc: Arc<&Vec<ObjectEnum>> = Arc::new(&objs);
    let threads_done_mutex_arc = Arc::new(Mutex::new(&mut threads_done));

    // ray_trace_segment(&mut frame, &objs, 0, 0, RESOLUTION, RESOLUTION);

    crossbeam::scope(move |scope| {
        print!("0.00%");
        std::io::stdout().flush().ok().expect("");

        let mut meta_rng = thread_rng();

        for thread in 0..THREADS {
            let start_index = thread * PIXELS_PER_THREAD;
            let objs_arc_clone = objs_arc.clone();
            let frame_mutex_arc_clone = frame_mutex_arc.clone();
            let threads_done_mutex_arc_clone = threads_done_mutex_arc.clone();
            let rng = SmallRng::from_rng(&mut meta_rng).unwrap();
            
            scope.spawn(move |_| {
                ray_trace_segment(
                    frame_mutex_arc_clone, 
                    objs_arc_clone, 
                    rng,
                    start_index,
                    usize::min(start_index + PIXELS_PER_THREAD, TOTAL_BUFFER_SIZE)
                );
                
                let mut threads_done = threads_done_mutex_arc_clone.lock().unwrap();
                **threads_done = (**threads_done) + 1;

                print!("\r{}%           ", format!("{:.*}", 2, (**threads_done as f32 / THREADS as f32) * 100.0));
                std::io::stdout().flush().ok().expect("");
            });
        }
    }).unwrap();

    println!("Total time: {}s", Instant::now().duration_since(start_time).as_millis() as f32 / 1000.0);
    println!("done");

    return frame;
}

/**
 * Raytrace one square sub-portion of the image (exists to facilitate threading)
 */
fn ray_trace_segment(frame_mutex: Arc<Mutex<&mut Frame>>, objs: Arc<&Vec<ObjectEnum>>, mut rng: SmallRng, start: usize, end: usize) {
    let mut buffer = [Color(0.0,0.0,0.0); PIXELS_PER_THREAD];
    let range = end - start;

    // Cast ray from each pixel
    for i in 0..range {
        let xy = Frame::pos_from_index(i + start);
        let ray = Frame::pixel_to_ray(&xy);

        let illumination = cast_ray(&ray, &objs, &mut rng, BOUNCES);

        buffer[i] = illumination.color * clamp(illumination.intensity, 0.0, 1.0);
    }

    let mut frame = frame_mutex.lock().unwrap();
    for i in 0..range {
        frame.buffer[i + start] = buffer[i];
    }
    std::mem::drop(frame);

    if start == 0 {
//        flame::dump_html(&mut File::create("target/flame-graph.html").unwrap()).unwrap();
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
