
use crate::color::Color;
use crate::texture::Texture;

pub struct TextureCheckered {
    pub rows: u32,
    pub cols: u32,
    pub color_1: Color,
    pub color_2: Color,
}

impl TextureCheckered {
    pub fn new() -> Self {
        Self {
            rows: 2,
            cols: 2,
            color_1: Color(255, 255, 255),
            color_2: Color(200, 200, 200)
        }
    }
}

impl Texture for TextureCheckered {
    fn color_at(&self, u: f32, v: f32) -> Color {
        let scaled_u = (u * self.cols as f32) as u32;
        let u_is_base = scaled_u % 2 == 0;

        let scaled_v = (v * self.cols as f32) as u32;
        let v_is_base = scaled_v % 2 == 0;

        return if u_is_base == v_is_base { self.color_1 }
               else { self.color_2 }
    }
}


#[test]
fn test() {
    let tex = TextureCheckered::new();

    assert_eq!(tex.color_at(0.1, 0.1), Color(255, 255, 255));
    assert_eq!(tex.color_at(0.4, 0.1), Color(200, 200, 200));
    assert_eq!(tex.color_at(0.1, 0.4), Color(200, 200, 200));
    assert_eq!(tex.color_at(0.4, 0.4), Color(255, 255, 255));
}