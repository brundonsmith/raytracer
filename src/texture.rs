
use std::fs::File;
use std::io::BufReader;
use image::{DynamicImage,ImageFormat,GenericImageView,load};

use crate::color::Color;


pub enum Texture {
    Solid(Color),
    Image(DynamicImage),
    Procedural(&'static (dyn Send + Sync + Fn((f32,f32)) -> Color)),
}

impl Texture {

    pub fn from_image(path: &str) -> Self {
        let f = File::open(path).expect("Failed to open texture file");
        let f = BufReader::new(f);
        let image = load(f, ImageFormat::JPEG).expect("Failed to load image");

        Texture::Image(image)
    }

    pub fn color_at(&self, uv: (f32,f32)) -> Color {
        match self {
            Texture::Solid(color) => *color,
            Texture::Image(image) => {
                let x = uv_to_pixel(uv.0, image.dimensions().0);
                let y = uv_to_pixel(uv.1, image.dimensions().1);
                let p = image.get_pixel(x, y);

                Color(
                    u8_to_f32(p[0]), 
                    u8_to_f32(p[1]), 
                    u8_to_f32(p[2]))
            }
            Texture::Procedural(func) => func(uv),
        }
    }
}

fn uv_to_pixel(uv: f32, dim: u32) -> u32 {
    (uv * dim as f32).floor() as u32
}

fn u8_to_f32(val: u8) -> f32 {
    val as f32 / 255.0
}

/**
 * Procedural texture callback for a checkerboard pattern
 */
pub fn checker(uv: (f32,f32)) -> Color {
    let scaled_u = (uv.0 * 4.0) as u32;
    let u_is_base = scaled_u % 2 == 0;

    let scaled_v = (uv.1 * 4.0) as u32;
    let v_is_base = scaled_v % 2 == 0;

    return if u_is_base == v_is_base { Color(1.0, 1.0, 1.0) }
           else { Color(0.5, 0.5, 0.5) }
}
