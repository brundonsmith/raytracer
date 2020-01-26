
use crate::color::Color;
use crate::vec3::Vec3;
use crate::matrix::{Matrix,IDENTITY};
use crate::ray::Ray;
use crate::fidelity_consts::{RESOLUTION_X,RESOLUTION_Y,TOTAL_BUFFER_SIZE};


const CAMERA_POSITION: Vec3 = Vec3 { 
    x: 0.0, 
    y: 0.0, 
    z: 0.0 
};

// construct within a function so as to further scope these constants
fn construct_camera_matrix() -> Matrix {
    let camera_height: f32 = 2.0;
    let camera_width: f32 = camera_height * 1.6;
    let focal_length: f32 = 2.0;
    
    return IDENTITY
        //* Matrix::rotation_x(-1.0 / 4.0 * std::f32::consts::PI)
        * Matrix::translation(&Vec3 { 
            x: -1.0 * camera_width / 2.0 + CAMERA_POSITION.x, 
            y: camera_height / 2.0 + CAMERA_POSITION.y, 
            z: -1.0 * focal_length + CAMERA_POSITION.z
        })
        * Matrix::scale(&Vec3 { 
            x: camera_width / RESOLUTION_X as f32, 
            y: -1.0 * camera_height / RESOLUTION_Y as f32, 
            z: 1.0 
        });
}

// HACK: .cos() isn't a const fn, for some reason, so lazy_static is 
// required to initialize this matrix
lazy_static! {
    static ref CAMERA_MARTIX: Matrix = construct_camera_matrix();
}

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
        x + y * RESOLUTION_X
    }

    pub fn pos_from_index(index: usize) -> (usize,usize) {
        (index % RESOLUTION_X, index / RESOLUTION_X)
    }

    /**
     * Find the world position of a pixel in this frame.
     */
    fn pixel_to_world(pixel: &(usize,usize)) -> Vec3 {
        Vec3 { 
            x: pixel.0 as f32,
            y: pixel.1 as f32,
            z: 0.0
        }.transformed(&CAMERA_MARTIX)
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