
// http://paulbourke.net/dataformats/mtl/

use std::fs;
use std::collections::HashMap;

use crate::material::Material;
use crate::color::Color;
use crate::texture::Texture;

pub fn load_and_parse(path: &str) -> HashMap<String,Material> {
    let data = fs::read_to_string(path).expect("Failed to open materials file");

    println!("Loading mtl...");

    let mats = parse(&data);
    
    println!("done");
    
    return mats;
}

pub fn parse(obj: &str) -> HashMap<String,Material> {
    let mut materials = HashMap::new();
    
    let mut mat: Option<(String,Material)> = None;
    for line in obj.split("\n") {
        let segments: Vec<&str> = line.trim().split(" ").collect();

        match segments[0] {
            "newmtl" => {
                mat.map(|material| {
                    materials.insert(material.0, material.1);
                });

                mat = Some((
                    String::from(segments[1]),
                    Material {
                        texture_albedo: None,
                        texture_specular: None,
                        texture_normal: None,
                        texture_emission_color: None,
                        texture_emission_intensity: None,
                    }
                ));
            },
            "Kd" => {
                mat.as_mut().unwrap().1.texture_albedo = Some(Texture::Solid(Color(
                    segments[1].parse().ok().unwrap(),
                    segments[2].parse().ok().unwrap(),
                    segments[3].parse().ok().unwrap()
                )));
            },
            
            /* specular tint
            "Ks" => {
                segments[1].parse().ok(),
                    segments[2].parse().ok(),
                    segments[3].parse().ok()
            },*/

            /* specular angle
            "Ns" => {
                segments[1].parse().ok()
            },*/

            /*
            "illum" => 

            0		Color on and Ambient off
            1		Color on and Ambient on
            2		Highlight on
            3		Reflection on and Ray trace on
            4		Transparency: Glass on
                    Reflection: Ray trace on
            5		Reflection: Fresnel on and Ray trace on
            6		Transparency: Refraction on
                    Reflection: Fresnel off and Ray trace on
            7		Transparency: Refraction on
                    Reflection: Fresnel on and Ray trace on
            8		Reflection on and Ray trace off
            9		Transparency: Glass on
                    Reflection: Ray trace off
            10		Casts shadows onto invisible surfaces
             */

            _ => ()
        }
    }

    mat.map(|material| {
        materials.insert(material.0, material.1);
    });

    return materials;
}
