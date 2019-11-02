
use crate::color::Color;
use crate::illumination::Illumination;
use crate::texture::Texture;
use crate::intersection::Intersection;
use crate::timing::{start,stop};

pub struct Material {
    pub texture_albedo: Option<Box<dyn Texture + Sync + Send>>,
    pub texture_specular: Option<Box<dyn Texture + Sync + Send>>,
    pub texture_emission: Option<Box<dyn Texture + Sync + Send>>,
}

impl Material {

    pub fn shade(&self, intersection: &Intersection, uv: (f32,f32), diffuse_illumination: &Option<Illumination>, specular_illumination: &Option<Illumination>) -> Illumination {
        let mut illumination = Illumination::new();
        let mut channels: f32 = 0.0;

        match &self.texture_albedo {
            Some(texture) => {
                illumination.color = illumination.color + (diffuse_illumination.unwrap().color * texture.color_at(uv));
                illumination.intensity += diffuse_illumination.unwrap().intensity * 0.5; // TODO: express roughness in data
                channels += 1.0;
            },
            _ => ()
        };

        match &self.texture_specular {
            Some(texture) => {
                illumination.color = illumination.color + (specular_illumination.unwrap().color * texture.color_at(uv));
                illumination.intensity += specular_illumination.unwrap().intensity * 0.9; // TODO: express roughness in data
                channels += 1.0;
            },
            _ => ()
        };

        match &self.texture_emission {
            Some(texture) => {
                illumination.color = illumination.color + texture.color_at(uv);
                illumination.intensity += 5.0; // TODO: Express brightness in data
                channels += 1.0;
            },
            _ => ()
        };

        illumination.color = illumination.color * (1.0 / channels);
        
        /*
        if channels > 1.2 { println!("diffuse  {:?}", diffuse_illumination); }
        if channels > 1.2 { println!("specular {:?}", specular_illumination); }
        if channels > 1.2 { println!("result   {:?}", illumination); }
*/

        return illumination;
    }
}
