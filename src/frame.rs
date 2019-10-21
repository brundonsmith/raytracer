
use crate::color::Color;
use crate::vec3::Vec3;
use crate::ray::Ray;

const CAMERA_WIDTH: f32 = 2.0;
const CAMERA_HEIGHT: f32 = 2.0;
const FOCAL_LENGTH: f32 = 2.0;
const CAMERA_POSITION: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
const CAMERA_TOP_LEFT: Vec3 = Vec3 { 
    x: CAMERA_POSITION.x - CAMERA_WIDTH / 2.0, 
    y: CAMERA_POSITION.y + CAMERA_HEIGHT / 2.0, 
    z: CAMERA_POSITION.z - FOCAL_LENGTH
};

pub struct Frame {
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

    /**
     * Find the world position of a pixel in this frame.
     */
    pub fn pixel_to_world(&self, pixel: &(usize,usize)) -> Vec3 {
        Vec3 { 
            x: CAMERA_TOP_LEFT.x + (0.5 + pixel.0 as f32) * self.units_per_pixel_x,
            y: CAMERA_TOP_LEFT.y - (0.5 + pixel.1 as f32) * self.units_per_pixel_y,
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
            origin: pixel_position,
            direction: direction
        }
    }
}