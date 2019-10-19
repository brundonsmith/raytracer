
use crate::color::Color;

pub trait Texture {

    /**
     * Get this texture's color at a given UV coordinate.
     */
    fn color_at(&self, uv: (f32,f32)) -> Color;
}
