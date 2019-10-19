
use crate::color::Color;
use crate::illumination::Illumination;
use crate::texture::Texture;

pub struct Material {
    pub shading_type: ShadingType,
    pub texture: Box<dyn Texture + Sync + Send>,
}

impl Material {
    pub fn shade(&self, incident: f32, illumination: Illumination, uv: (f32,f32)) -> Color {
        println!("{}", incident);
        //println!("{}", intersection.incident_angle() / (PI / 2.0));
        match self.shading_type {
            ShadingType::Diffuse =>
                illumination.color * self.texture.color_at(uv) * illumination.intensity * incident,
            ShadingType::Light => illumination.color,
            _ => Color(1.0, 0.0, 1.0), // magenta; material "error"
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ShadingType {
    Diffuse,
    Specular,
    Transparent,
    Light
}
