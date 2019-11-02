
use crate::color::Color;
use crate::illumination::Illumination;
use crate::texture::Texture;
use crate::intersection::Intersection;
use crate::timing::{start,stop};

const ERROR_COLOR: Color = Color(1.0, 0.0, 1.0);

pub struct Material {
    pub texture_albedo: Option<Box<dyn Texture + Sync + Send>>,
    pub texture_specular: Option<Box<dyn Texture + Sync + Send>>,
    pub texture_emission: Option<Box<dyn Texture + Sync + Send>>,
}

impl Material {

    pub fn new() -> Self {
        Material {
            texture_albedo: None,
            texture_specular: None,
            texture_emission: None,
        }
    }

    pub fn shade(&self, intersection: &Intersection, uv: (f32,f32), diffuse_illumination: &Option<Illumination>, specular_illumination: &Option<Illumination>) -> Illumination {
        let mut illumination = Illumination::new();

        // TODO: Blend channels together instead of having them clobber each other

        match &self.texture_albedo {
            Some(texture) => {
                illumination.color = diffuse_illumination.unwrap().color * texture.color_at(uv);
                illumination.intensity = diffuse_illumination.unwrap().intensity * 0.5; // TODO: express roughness in data
            },
            _ => ()
        };

        match &self.texture_specular {
            Some(texture) => {
                illumination.color = specular_illumination.unwrap().color * texture.color_at(uv);
                illumination.intensity = specular_illumination.unwrap().intensity * 0.9; // TODO: express roughness in data
            },
            _ => ()
        };

        match &self.texture_emission {
            Some(texture) => {
                illumination.color = texture.color_at(uv);
                illumination.intensity = 2.0; // TODO: Express brightness in data
            },
            _ => ()
        };
        
        return illumination;
    }
}
