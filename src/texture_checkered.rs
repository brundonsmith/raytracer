
use crate::color::Color;
use crate::texture::Texture;

/**
 * A texture consisting of a rectangular grid of cells alternating
 * between two colors.
 */
pub struct TextureCheckered {
    pub rows: u32,
    pub cols: u32,
    pub color_1: Color,
    pub color_2: Color,
}

impl TextureCheckered {
    pub fn new() -> Self {
        Self {
            rows: 8,
            cols: 8,
            color_1: Color(1.0, 1.0, 1.0),
            color_2: Color(0.0, 0.0, 0.0)
        }
    }
    pub fn from_colors(color_1: Color, color_2: Color) -> Self {
        Self {
            rows: 8,
            cols: 8,
            color_1,
            color_2
        }
    }
}

impl Texture for TextureCheckered {
    fn color_at(&self, uv: (f32,f32)) -> Color {
        let scaled_u = (uv.0 * self.cols as f32) as u32;
        let u_is_base = scaled_u % 2 == 0;

        let scaled_v = (uv.1 * self.cols as f32) as u32;
        let v_is_base = scaled_v % 2 == 0;

        return if u_is_base == v_is_base { self.color_1 }
               else { self.color_2 }
    }
}


#[test]
fn test() {
    let tex = TextureCheckered::new();

    assert_eq!(tex.color_at((0.01, 0.01)), Color(1.0, 1.0, 1.0));
    assert_eq!(tex.color_at((0.4, 0.01)), Color(0.5, 0.5, 0.5));
    assert_eq!(tex.color_at((0.01, 0.4)), Color(0.5, 0.5, 0.5));
    assert_eq!(tex.color_at((0.4, 0.4)), Color(1.0, 1.0, 1.0));
    assert_eq!(tex.color_at((0.01, 0.4)), Color(0.5, 0.5, 0.5));
    assert_eq!(tex.color_at((0.4, 0.4)), Color(1.0, 1.0, 1.0));
}