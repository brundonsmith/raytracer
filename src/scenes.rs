
use crate::color::Color;
use crate::material::Material;
use crate::vec3::Vec3;
use crate::object::Object;
use crate::sphere::Sphere;
use crate::plane::Plane;
use crate::texture_solid::TextureSolid;
use crate::texture_checkered::TextureCheckered;
use crate::texture_image::TextureImage;

pub fn construct_reflect_scene() -> Vec<Box<dyn Object + Sync + Send>> {
    let mut objs: Vec<Box<dyn Object + Sync + Send>> = Vec::new();

    objs.push(Box::new(Sphere::new(
        Vec3 { x: 0.0, y: 0.0, z: -12.0 },
        1.0,
        Material {
            texture_albedo: None,//Some(Box::new(TextureSolid::new())),
            texture_specular: None,//Some(Box::new(TextureSolid::new())),
            texture_emission: Some(Box::new(TextureSolid::new()))
        }
    )));

    objs.push(Box::new(Plane::new(
        Vec3 { x: 0.0, y: -1.5, z: 0.0, },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        Material {
            texture_albedo: None,//Some(Box::new(TextureSolid::new())),
            texture_specular: Some(Box::new(TextureSolid::new())),
            texture_emission: None,//Some(Box::new(TextureSolid::new())),
        }
    )));

    return objs;
}

pub fn construct_image_texture_test() -> Vec<Box<dyn Object + Sync + Send>> {
    let mut objs: Vec<Box<dyn Object + Sync + Send>> = Vec::new();
    
    objs.push(Box::new(Sphere::new(
        Vec3 { x: 0.0, y: 0.0, z: -5.0 },
        1.0,
        Material {
            texture_albedo: Some(Box::new(TextureImage::new("C:\\Users\\Brundon\\git\\raytracer\\texture.jpg"))),
            texture_specular: None,//Some(Box::new(TextureSolid::new())),
            texture_emission: None,
        }
    )));

    
    // ceiling
    objs.push(Box::new(Plane::new(
        Vec3 { x: 0.0, y: 5.0, z: 0.0, },
        Vec3 { x: 0.0, y: -1.0, z: 0.0 },
        Material {
            texture_albedo: None,
            texture_specular: None,
            texture_emission: Some(Box::new(TextureSolid::new())),
        }
    )));
    

    // floor
    objs.push(Box::new(Plane::new(
        Vec3 { x: 0.0, y: -5.0, z: 0.0, },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        Material {
            texture_albedo: None,
            texture_specular: None,
            texture_emission: Some(Box::new(TextureSolid::new())),
        }
    )));

    
    // left wall
    objs.push(Box::new(Plane::new(
        Vec3 { x: -5.0, y: 0.0, z: 0.0, },
        Vec3 { x: 1.0, y: 0.0, z: 0.0 },
        Material {
            texture_albedo: None,
            texture_specular: None,
            texture_emission: Some(Box::new(TextureSolid::new())),
        }
    )));

    // right wall
    objs.push(Box::new(Plane::new(
        Vec3 { x: 5.0, y: 0.0, z: 0.0, },
        Vec3 { x: -1.0, y: 0.0, z: 0.0 },
        Material {
            texture_albedo: None,
            texture_specular: None,
            texture_emission: Some(Box::new(TextureSolid::new())),
        }
    )));

    // back wall
    objs.push(Box::new(Plane::new(
        Vec3 { x: 0.0, y: 0.0, z: -15.0, },
        Vec3 { x: 0.0, y: 0.0, z: 1.0 },
        Material {
            texture_albedo: None,
            texture_specular: None,
            texture_emission: Some(Box::new(TextureSolid::new())),
        }
    )));

    // near wall
    objs.push(Box::new(Plane::new(
        Vec3 { x: 0.0, y: 0.0, z: 1.0, },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: None,
            texture_specular: None,
            texture_emission: Some(Box::new(TextureSolid::new())),
        }
    )));
    
    return objs;
}

pub fn construct_room_scene() -> Vec<Box<dyn Object + Sync + Send>> {
    let mut objs: Vec<Box<dyn Object + Sync + Send>> = Vec::new();

    // spheres
    objs.push(Box::new(Sphere::new(
        Vec3 { x: -1.5, y: 0.0, z: -9.0 },
        1.0,
        Material {
            texture_albedo: Some(Box::new(TextureImage::new("C:\\Users\\Brundon\\git\\raytracer\\texture.jpg"))),
            texture_specular: Some(Box::new(TextureImage::new("C:\\Users\\Brundon\\git\\raytracer\\texture.jpg"))),
            texture_emission: None,
        }
    )));
    
    objs.push(Box::new(Sphere::new(
        Vec3 { x: 2.0, y: -3.0, z: -12.0 },
        1.0,
        Material {
            texture_albedo: None,//Some(Box::new(TextureSolid { color: Color(0.0, 1.0, 1.0) })),
            texture_specular: Some(Box::new(TextureSolid::new())),
            texture_emission: None,//Some(Box::new(TextureSolid::new())),
        }
    )));

    /*
    for _ in 0..10 {
        objs.push(Box::new(Sphere::new(
            Vec3 {
                x: (rand::random::<u8>() % 10) as f32 - 5.0,
                y: (rand::random::<u8>() % 10) as f32 - 5.0,
                z: (rand::random::<u8>() % 10) as f32 - 15.0,
            },
            1.0,
            Material {
                texture_albedo: Some(Box::new(TextureCheckered::new())),
                texture_specular: None,
                texture_emission: None,
            }
        )))
    }*/

    
    // ceiling
    objs.push(Box::new(Plane::new(
        Vec3 { x: 0.0, y: 5.0, z: 0.0, },
        Vec3 { x: 0.0, y: -1.0, z: 0.0 },
        Material {
            texture_albedo: None,//Some(Box::new(TextureSolid { color: Color(1.0, 0.95, 0.8) })),
            texture_specular: None,
            texture_emission: Some(Box::new(TextureSolid { color: Color(1.0, 0.95, 0.8) })),
        }
    )));
    

    // floor
    objs.push(Box::new(Plane::new(
        Vec3 { x: 0.0, y: -5.0, z: 0.0, },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        Material {
            texture_albedo: Some(Box::new(TextureSolid::new())),
            texture_specular: None,//Some(Box::new(TextureSolid::new())),
            texture_emission: None,//Some(Box::new(TextureSolid::new())),
        }
    )));

    
    // left wall
    objs.push(Box::new(Plane::new(
        Vec3 { x: -5.0, y: 0.0, z: 0.0, },
        Vec3 { x: 1.0, y: 0.0, z: 0.0 },
        Material {
            texture_albedo: Some(Box::new(TextureSolid { color: Color(1.0, 0.0, 0.0) })),
            texture_specular: None,
            texture_emission: None,//Some(Box::new(TextureSolid { color: Color(1.0, 0.0, 0.0) })),
        }
    )));

    // right wall
    objs.push(Box::new(Plane::new(
        Vec3 { x: 5.0, y: 0.0, z: 0.0, },
        Vec3 { x: -1.0, y: 0.0, z: 0.0 },
        Material {
            texture_albedo: Some(Box::new(TextureSolid { color: Color(0.0, 1.0, 0.0) })),
            texture_specular: None,//Some(Box::new(TextureSolid::new())),
            texture_emission: None,//Some(Box::new(TextureSolid { color: Color(0.0, 1.0, 0.0) })),
        }
    )));

    // back wall
    objs.push(Box::new(Plane::new(
        Vec3 { x: 0.0, y: 0.0, z: -15.0, },
        Vec3 { x: 0.0, y: 0.0, z: 1.0 },
        Material {
            texture_albedo: Some(Box::new(TextureSolid::new())),
            texture_specular: None,
            texture_emission: None,//Some(Box::new(TextureSolid::new())),
        }
    )));

    // near wall
    objs.push(Box::new(Plane::new(
        Vec3 { x: 0.0, y: 0.0, z: 1.0, },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: Some(Box::new(TextureSolid { color: Color(0.0, 0.0, 1.0) })),
            texture_specular: None,
            texture_emission: None,//Some(Box::new(TextureSolid { color: Color(0.0, 0.0, 1.0) })),
        }
    )));
    
    return objs;
}