
use crate::color::Color;

pub trait Texture {
    fn color_at(&self, u: f32, v: f32) -> Color;
}
