
use crate::color::Color;
use crate::texture::Texture;

/**
 * A texture consisting of a rectangular grid of cells alternating
 * between two colors.
 */
pub struct TextureSolid {
    color: Color,
}

impl TextureSolid {
    pub fn new(color: Color) -> Self {
        Self {
            color
        }
    }
}

impl Texture for TextureSolid {
    fn color_at(&self, uv: (f32,f32)) -> Color {
        self.color
    }
}
