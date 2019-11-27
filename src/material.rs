
use crate::illumination::Illumination;
use crate::texture::Texture;
use crate::color::Color;
use crate::intersection::Intersection;

pub struct Material {
    pub texture_albedo: Option<Box<dyn Texture + Sync + Send>>,
    pub texture_specular: Option<Box<dyn Texture + Sync + Send>>,
    pub texture_normal: Option<Box<dyn Texture + Sync + Send>>,
    pub texture_emission: Option<Box<dyn Texture + Sync + Send>>,
}

impl Material {

    pub fn new() -> Self {
        Self {
            texture_albedo: None,
            texture_specular: None,
            texture_normal: None,
            texture_emission: None,
        }
    }

    pub fn shade(&self, intersection: &Intersection, uv: (f32,f32), diffuse_illumination: &Option<Illumination>, specular_illumination: &Option<Illumination>) -> Illumination {
        let mut illumination = Illumination::new();



        let mut color_elements = 1.0;
        illumination.color = self.intrinsic_color_at(uv);
    
        match diffuse_illumination {
            Some(i) => {
                illumination.color = illumination.color + i.color;
                color_elements += 1.0;
            },
            None => ()
        };

        match specular_illumination {
            Some(i) => {
                illumination.color = illumination.color + i.color;
                color_elements += 1.0;
            },
            None => ()
        };

        illumination.color = illumination.color * (1.0 / color_elements);



        illumination.intensity = 
            match diffuse_illumination {
                Some(ill) => ill.intensity * 0.5,
                None => 0.0
            }
            +
            match specular_illumination {
                Some(ill) => ill.intensity * self.intrinsic_specularity_at(uv),
                None => 0.0
            }
            +
            self.intrinsic_illumination_at(uv).intensity;
            
            

        return illumination;
    }

    fn intrinsic_color_at(&self, uv: (f32,f32)) -> Color {
        let mut color = Color(0.0, 0.0, 0.0);
        let mut elements = 0.0;

        match &self.texture_albedo {
            Some(texture) => {
                color = color + texture.color_at(uv);
                elements += 1.0;
            },
            None => ()
        };

        match &self.texture_emission {
            Some(texture) => {
                color = color + texture.color_at(uv);
                elements += 1.0;
            },
            None => ()
        };

        if elements > 0.0 {
            color = color * (1.0 / elements);
        }

        return color;
    }

    fn intrinsic_specularity_at(&self, uv: (f32,f32)) -> f32 {
        let mut specularity = 0.0;
        let mut elements = 0.0;

        match &self.texture_specular {
            Some(texture) => {
                specularity = specularity + texture.color_at(uv).0;
                elements += 1.0;
            },
            None => ()
        };

        if elements > 0.0 {
            specularity = specularity / elements;
        }

        return specularity;
    }

    fn intrinsic_illumination_at(&self, uv: (f32,f32)) -> Illumination {
        let mut illumination = Illumination::new();
        let mut elements = 0.0;

        match &self.texture_emission {
            Some(texture) => {
                illumination.color = texture.color_at(uv);
                illumination.intensity = 3.0; // TODO: Get this from alpha
                elements += 1.0;
            },
            None => ()
        };

        if elements > 0.0 {
            illumination.color = illumination.color * (1.0 / elements);
            illumination.intensity /= elements;
        }

        return illumination;
    }
}
