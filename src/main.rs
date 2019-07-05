
extern crate rand;
extern crate image;
use image::{ImageBuffer, Rgb};

mod vec3;
mod matrix;
mod ray;
mod object;
mod sphere;

use vec3::Vec3;
use ray::Ray;
use sphere::Sphere;
use object::Object;

const RESOLUTION: usize = 512;
const CAMERA_POSITION: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
const FOCAL_LENGTH: f32 = 1.0;
const CAMERA_WIDTH: f32 = 2.0;
const CAMERA_HEIGHT: f32 = 2.0;
const CAMERA_TOP_LEFT: Vec3 = Vec3 { 
    x: CAMERA_POSITION.x - CAMERA_WIDTH / 2.0, 
    y: CAMERA_POSITION.y + CAMERA_HEIGHT / 2.0, 
    z: CAMERA_POSITION.z - FOCAL_LENGTH
};
// TODO: Camera transformation via matrix


#[derive(Debug, Copy, Clone)]
struct Color(u8,u8,u8);


struct Frame {
    pub buffer: Vec<Vec<Color>>,
    units_per_pixel_x: f32,
    units_per_pixel_y: f32,
}

impl Frame {

    pub fn new(width: usize, height: usize) -> Self {
        Frame { 
            buffer: vec![vec![Color(0, 0, 0); height]; width],
            units_per_pixel_x: CAMERA_WIDTH / width as f32,
            units_per_pixel_y: CAMERA_HEIGHT / height as f32
        }
    }

    pub fn units_per_pixel_x(&self) -> f32 { self.units_per_pixel_x }
    pub fn units_per_pixel_y(&self) -> f32 { self.units_per_pixel_y }
    
    pub fn pixel_to_world(&self, pixel: &(usize,usize)) -> Vec3 {
        Vec3 { 
            x: CAMERA_TOP_LEFT.x + (0.5 + pixel.0 as f32) * self.units_per_pixel_x(),
            y: CAMERA_TOP_LEFT.y - (0.5 + pixel.1 as f32) * self.units_per_pixel_y(),
            z: CAMERA_TOP_LEFT.z
        }
    }

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


fn ray_trace() -> Frame {
    
    println!("Tracing scene...");
    
    let mut ray_frame = Frame::new(RESOLUTION,RESOLUTION);
    let mut objs: Vec<Box<dyn Object>> = Vec::new();
    for _ in 0..10 {
        objs.push(Box::new(Sphere {
            position: Vec3 {
                x: (rand::random::<u8>() % 10) as f32 - 5.0,
                y: (rand::random::<u8>() % 10) as f32 - 5.0,
                z: (rand::random::<u8>() % 10) as f32 - 15.0,
            },
            radius: 1.0
        }))
    }

    for x in 0..RESOLUTION {
        for y in 0..RESOLUTION {
            let ray = ray_frame.pixel_to_ray(&(x, y));

            for obj in &objs {
                match obj.intersection(&ray) {
                    Some(distance) => {
                        if x == 0 && y == 0 {
                            println!("{:?}", distance);
                        }

                        ray_frame.buffer[x as usize][y as usize] = Color(0, (255.0 - (distance * 10.0)) as u8, 0)
                    },
                    _ => ()
                }
            }

            //println!("{:?}", ray);
            //ray_frame.buffer[x as usize][y as usize] = Color((ray.direction.x.abs() * 100.0) as u8, (ray.direction.y.abs() * 100.0) as u8, (ray.direction.z.abs() * 100.0) as u8);
        }
    }

    println!("done");

    return ray_frame;
}



fn main() {
    png_render();
}


fn png_render() {
    let ray_frame = ray_trace();
    write_image(&ray_frame);
}

fn write_image(ray_frame: &Frame) {
    println!("Writing to png...");

    let mut image: ImageBuffer::<Rgb<u8>,Vec<u8>> = ImageBuffer::new(RESOLUTION as u32, RESOLUTION as u32);

    for x in 0..RESOLUTION {
        for y in 0..RESOLUTION {
            let color = ray_frame.buffer[x][y];
            image.get_pixel_mut(x as u32, y as u32).data = [ color.0, color.1, color.2 ];
        }
    }

    image.save("output.png").unwrap();

    println!("done");
}






// nannou
extern crate nannou;
use nannou::prelude::*;

fn nannou_render() {
    nannou::sketch(view);
}

fn view(app: &App, frame: nannou::frame::Frame) -> nannou::frame::Frame {

    let ray_frame = ray_trace();

    println!("Drawing to window...");

    // Draw to window frame
    app.main_window().set_inner_size_pixels(RESOLUTION as u32, RESOLUTION as u32);
    let draw = app.draw();

    for x in 0..RESOLUTION {
        for y in 0..RESOLUTION {
            let color = ray_frame.buffer[x][y];
            if color.0 > 0 || color.1 > 0 || color.2 > 0 {
                draw.rect()
                    .x_y(x as f32 - 256.0, y as f32 - 256.0)
                    .w_h(1.0, 1.0)
                    .rgb(color.0 as f32 / 255.0, color.1 as f32 / 255.0, color.2 as f32 / 255.0);
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();

    println!("done");



    // Return the drawn frame.
    return frame;
}

