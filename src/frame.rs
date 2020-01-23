
use crate::color::Color;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::fidelity_consts::{RESOLUTION,TOTAL_BUFFER_SIZE};

const CAMERA_WIDTH: f32 = 2.0;
const CAMERA_HEIGHT: f32 = 2.0;
const FOCAL_LENGTH: f32 = 2.0;
const CAMERA_POSITION: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
const CAMERA_TOP_LEFT: Vec3 = Vec3 { 
    x: CAMERA_POSITION.x - CAMERA_WIDTH / 2.0, 
    y: CAMERA_POSITION.y + CAMERA_HEIGHT / 2.0, 
    z: CAMERA_POSITION.z - FOCAL_LENGTH
};


const UNITS_PER_PIXEL_X: f32 = CAMERA_WIDTH / RESOLUTION as f32;
const UNITS_PER_PIXEL_Y: f32 = CAMERA_HEIGHT / RESOLUTION as f32;

pub struct Frame {
    pub buffer: Box<[Color]>,
}

impl Frame {

    pub fn new() -> Self {
        Frame { 
            buffer: vec![Color(0.0,0.0,0.0); TOTAL_BUFFER_SIZE].into_boxed_slice(),
        }
    }

    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        self.buffer[Frame::index(x,y)] = color;
    }

    pub fn get(&self, x: usize, y: usize) -> Color {
        self.buffer[Frame::index(x,y)]
    }

    fn index(x: usize, y: usize) -> usize {
        x + y * RESOLUTION
    }

    pub fn pos_from_index(index: usize) -> (usize,usize) {
        (index % RESOLUTION, index / RESOLUTION)
    }

    /**
     * Find the world position of a pixel in this frame.
     */
    pub fn pixel_to_world(pixel: &(usize,usize)) -> Vec3 {
        Vec3 { 
            x: CAMERA_TOP_LEFT.x + (0.5 + pixel.0 as f32) * UNITS_PER_PIXEL_X,
            y: CAMERA_TOP_LEFT.y - (0.5 + pixel.1 as f32) * UNITS_PER_PIXEL_Y,
            z: CAMERA_TOP_LEFT.z
        }
    }

    /**
     * Initialize a ray projecting out from one pixel in this frame 
     * (incorporating not only origin position, but direction from 
     * the focal point).
     */
    pub fn pixel_to_ray(pixel: &(usize,usize)) -> Ray {
        let pixel_position = Frame::pixel_to_world(&pixel);
        let mut direction = &pixel_position - &CAMERA_POSITION;
        direction.normalize();

        Ray {
            origin: pixel_position,
            direction: direction
        }
    }
}