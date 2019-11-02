
use std::fs::File;
use std::io::BufReader;

use image::{DynamicImage,ImageFormat,GenericImageView,load};

use crate::color::Color;
use crate::texture::Texture;

/**
 * A texture consisting of a rectangular grid of cells alternating
 * between two colors.
 */
pub struct TextureImage {
    image: DynamicImage
}

impl TextureImage {
    pub fn new(path: &str) -> Self {
        let f = File::open(path).expect("Failed to open texture file");
        let f = BufReader::new(f);
        let image = load(f, ImageFormat::JPEG).expect("Failed to load image");

        Self {
            image
        }
    }

}

impl Texture for TextureImage {
    fn color_at(&self, uv: (f32,f32)) -> Color {
        let x = uv_to_pixel(uv.0, self.image.dimensions().0);
        let y = uv_to_pixel(uv.1, self.image.dimensions().1);
        let p = self.image.get_pixel(x, y);

        return Color(
            u8_to_f32(p[0]), 
            u8_to_f32(p[1]), 
            u8_to_f32(p[2]));
    }
}

fn uv_to_pixel(uv: f32, dim: u32) -> u32 {
    (uv * dim as f32).floor() as u32
}

fn u8_to_f32(val: u8) -> f32 {
    val as f32 / 255.0
}