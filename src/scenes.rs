
use std::collections::HashMap;

use crate::color::Color;
use crate::material::Material;
use crate::vec3::Vec3;
use crate::object::Object;
use crate::sphere::Sphere;
use crate::mesh::{Mesh,Face};
use crate::plane::Plane;
use crate::texture_solid::TextureSolid;
use crate::texture_checkered::TextureCheckered;
use crate::texture_image::TextureImage;
use crate::matrix::Matrix;
use crate::utils::{ObjectVec};
use crate::mtl_parser::load_and_parse;

pub fn construct_reflect_scene() -> ObjectVec {
    let mut objs: ObjectVec = Vec::new();

    objs.push(Box::new(Sphere::new(
        Vec3 { x: 0.0, y: 0.0, z: -12.0 },
        1.0,
        Material {
            texture_albedo: None,//Some(Box::new(TextureSolid::new())),
            texture_specular: None,//Some(Box::new(TextureSolid::new())),
            texture_normal: None,
            texture_emission: Some(Box::new(TextureSolid::new()))
        }
    )));

    objs.push(Box::new(Plane::new(
        Vec3 { x: 0.0, y: -1.5, z: 0.0, },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: Some(Box::new(TextureSolid { color: Color(1.0, 0.0, 0.0) })),
            texture_specular: Some(Box::new(TextureImage::new("/Users/brundolf/git/raytracer/specular.jpeg"))),
            texture_normal: None,
            texture_emission: None,
        }
    )));

    return objs;
}

pub fn construct_material_scene() -> ObjectVec {
    let mut objs: ObjectVec = Vec::new();

    objs.push(Box::new(Sphere::new(
        Vec3 { x: 0.0, y: 0.0, z: -12.0 },
        1.0,
        Material {
            texture_albedo: None,//Some(Box::new(TextureSolid::new())),
            texture_specular: None,//Some(Box::new(TextureSolid::new())),
            texture_normal: None,
            texture_emission: Some(Box::new(TextureSolid { color: Color(1.0, 0.0, 0.0) }))
        }
    )));

    // ceiling
    objs.push(Box::new(Plane::new(
        Vec3 { x: 0.0, y: 5.0, z: 0.0, },
        Vec3 { x: 0.0, y: -1.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: None,//Some(Box::new(TextureSolid { color: Color(1.0, 0.95, 0.8) })),
            texture_specular: None,
            texture_normal: None,
            texture_emission: Some(Box::new(TextureSolid { color: Color(1.0, 1.0, 1.0) })),
        }
    )));

    objs.push(Box::new(Plane::new(
        Vec3 { x: 0.0, y: -1.5, z: 0.0, },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: None,//Some(Box::new(TextureSolid::new())),
            texture_specular: Some(Box::new(TextureImage::new("/Users/brundolf/git/raytracer/CobblestoneSpecular.jpg"))),
            texture_normal: None,
            texture_emission: None,//Some(Box::new(TextureSolid::new())),
        }
    )));

    return objs;
}

pub fn construct_tree_scene() -> ObjectVec {
    let mut objs: ObjectVec = Vec::new();

    objs.push(Box::new(Mesh::from_obj(
        "/Users/brundolf/git/raytracer/floor.obj", 
        &(&Matrix::translation(&Vec3 { x: 0.0, y: 0.0, z: -3.0 }) *
          &Matrix::rotation_y(std::f32::consts::PI / -4.0)),
        load_and_parse("/Users/brundolf/git/raytracer/tree.mtl")
    )));

    objs.push(Box::new(Plane::new(
        Vec3 { x: 2.0, y: 0.0, z: 2.0, },
        Vec3 { x: -1.0, y: 0.0, z: -1.0 },
        Vec3 { x: 1.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: None,
            texture_specular: None,
            texture_normal: None,
            texture_emission: Some(Box::new(TextureSolid { color: Color(1.0, 0.52, 0.17) })),
        }
    )));

    return objs;
}

pub fn construct_room_scene() -> ObjectVec {
    let mut objs: ObjectVec = Vec::new();

    // spheres
    /*
    objs.push(Box::new(Sphere::new(
        Vec3 { x: -1.5, y: 0.0, z: -9.0 },
        1.0,
        Material {
            texture_albedo: Some(Box::new(TextureCheckered::new())),
            texture_specular: None,
            texture_emission: None,
        }
    )));*/
    
    /*
    objs.push(Box::new(Sphere::new(
        Vec3 { x: 0.0, y: 0.0, z: -5.0 },
        1.0,
        Material {
            texture_albedo: None,//Some(Box::new(TextureCheckered::from_colors(Color(1.0,1.0,1.0), Color(0.0,0.0,0.0)))),
            texture_specular: Some(Box::new(TextureCheckered::from_colors(Color(0.6,0.6,0.6), Color(0.01,0.01,0.01)))),
            texture_emission: None,//Some(Box::new(TextureSolid::new())),
        }
    )));*/

    /*
    objs.push(Box::new(Mesh {
        position: Vec3 { x: 0.0, y: 0.0, z: -5.0 },
        material: Material {
            texture_albedo: Some(Box::new(TextureCheckered::new())),
            texture_specular: None,
            texture_emission: None,
        },
        vertices: vec![
            Vec3 { x: 0.0, y: 0.0, z: -5.0 },
            Vec3 { x: 1.0, y: 0.0, z: -5.0 },
            Vec3 { x: 1.0, y: 1.0, z: -6.0 },
        ],
        faces: vec![
            Face (0, 1, 2)
        ],
        uv_coords: vec![]
    }));*/

    objs.push(Box::new(Sphere::new(
        Vec3 { x: 3.0, y: -3.0, z: -13.0 },
        1.0,
        Material {
            texture_albedo: None,//Some(Box::new(TextureSolid::new())),
            texture_specular: None,//Some(Box::new(TextureSolid::new())),
            texture_normal: None,
            texture_emission: Some(Box::new(TextureSolid { color: Color(0.0, 1.0, 1.0) }))
        }
    )));

    objs.push(Box::new(Sphere::new(
        Vec3 { x: -2.0, y: 0.0, z: -8.0 },
        1.0,
        Material {
            texture_albedo: None,//Some(Box::new(TextureSolid::new())),
            texture_specular: Some(Box::new(TextureSolid::new())),
            texture_normal: None,
            texture_emission: None,
        }
    )));


    let mut mats = HashMap::new();
    mats.insert(String::from("Default"), Material {
        texture_albedo: Some(Box::new(TextureSolid::new())),
        texture_specular: None,
        texture_normal: None,
        texture_emission: None,
    });

    objs.push(Box::new(Mesh::from_obj(
        "/Users/brundolf/git/raytracer/test.obj", 
        &(&Matrix::translation(&Vec3 { x: 0.0, y: -3.0, z: -10.0 }) *
        &(&Matrix::rotation_y(std::f32::consts::PI) *
          &Matrix::scale(&Vec3::from_scalar(0.5)))),
        mats
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
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: None,//Some(Box::new(TextureSolid::new())),
            texture_specular: None,
            texture_normal: None,
            texture_emission: Some(Box::new(TextureSolid { color: Color(1.0, 0.95, 0.8) })),
        }
    )));

    /*
    objs.push(Box::new(Mesh::new(
        Material {
            texture_albedo: None,//Some(Box::new(TextureSolid { color: Color(1.0, 0.95, 0.8) })),
            texture_specular: None,
            texture_normal: None,
            texture_emission: Some(Box::new(TextureSolid { color: Color(1.0, 0.95, 0.8) })),
        },
        vec![
            Vec3 { x: 1.0,  y: 4.99, z: -11.0 },
            Vec3 { x: 1.0,  y: 4.99, z: -13.0 },
            Vec3 { x: -1.0, y: 4.99, z: -13.0 },
            Vec3 { x: -1.0, y: 4.99, z: -11.0 },
        ],
        vec![
            Face(0, 1, 2),
            Face(2, 3, 0)
        ],
        vec![]
    )));*/
    

    // floor
    objs.push(Box::new(Plane::new(
        Vec3 { x: 0.0, y: -5.0, z: 0.0, },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: Some(Box::new(TextureImage::new("/Users/brundolf/git/raytracer/texture.jpg"))),
            texture_specular: None,//Some(Box::new(TextureImage::new("C:\\Users\\Brundon\\git\\raytracer\\texture.jpg"))),
            texture_normal: None,
            texture_emission: None,//Some(Box::new(TextureImage::new("C:\\Users\\Brundon\\git\\raytracer\\texture.jpg")))
        }
    )));

    
    // left wall
    objs.push(Box::new(Plane::new(
        Vec3 { x: -5.0, y: 0.0, z: 0.0, },
        Vec3 { x: 1.0, y: 0.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: Some(Box::new(TextureSolid { color: Color(1.0, 0.0, 0.0) })),
            texture_specular: None,
            texture_normal: None,
            texture_emission: None,//Some(Box::new(TextureSolid { color: Color(1.0, 0.0, 0.0) })),
        }
    )));

    // right wall
    objs.push(Box::new(Plane::new(
        Vec3 { x: 5.0, y: 0.0, z: 0.0, },
        Vec3 { x: -1.0, y: 0.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: Some(Box::new(TextureSolid { color: Color(0.0, 1.0, 0.0) })),
            texture_specular: None,//Some(Box::new(TextureSolid::new())),
            texture_normal: None,
            texture_emission: None,//Some(Box::new(TextureSolid { color: Color(0.0, 1.0, 0.0) })),
        }
    )));

    // back wall
    objs.push(Box::new(Plane::new(
        Vec3 { x: 0.0, y: 0.0, z: -15.0, },
        Vec3 { x: 0.0, y: 0.0, z: 1.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        Material {
            texture_albedo: Some(Box::new(TextureSolid::new())),
            texture_specular: None,
            texture_normal: None,
            texture_emission: None,//Some(Box::new(TextureSolid::new())),
        }
    )));

    // near wall
    objs.push(Box::new(Plane::new(
        Vec3 { x: 0.0, y: 0.0, z: 1.0, },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        Material {
            texture_albedo: Some(Box::new(TextureSolid { color: Color(0.0, 0.0, 1.0) })),
            texture_specular: None,
            texture_normal: None,
            texture_emission: None,//Some(Box::new(TextureSolid { color: Color(0.0, 0.0, 1.0) })),
        }
    )));
    
    return objs;
}

pub fn construct_plane_texture_test() -> ObjectVec {
    let mut objs: ObjectVec = Vec::new();

    objs.push(Box::new(Plane::new(
        Vec3 { x: 0.0, y: 5.0, z: 0.0, },
        Vec3 { x: 0.0, y: -1.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: None,
            texture_specular: None,
            texture_normal: None,
            texture_emission: Some(Box::new(TextureSolid::new())),
        }
    )));

    objs.push(Box::new(Plane::new(
        Vec3 { x: 0.0, y: -1.5, z: 0.0, },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: Some(Box::new(TextureImage::new("/Users/brundolf/git/raytracer/grid.jpg"))),
            texture_specular: None,
            texture_normal: None,
            texture_emission: None,
        }
    )));

    return objs;
}

pub fn construct_sphere_texture_test() -> ObjectVec {
    let mut objs: ObjectVec = Vec::new();
    
    objs.push(Box::new(Sphere::new(
        Vec3 { x: 0.0, y: 0.0, z: -5.0 },
        1.0,
        Material {
            texture_albedo: Some(Box::new(TextureImage::new("C:\\Users\\Brundon\\git\\raytracer\\texture.jpg"))),
            texture_specular: None,//Some(Box::new(TextureSolid::new())),
            texture_normal: None,
            texture_emission: None,
        }
    )));

    
    // ceiling
    objs.push(Box::new(Plane::new(
        Vec3 { x: 0.0, y: 5.0, z: 0.0, },
        Vec3 { x: 0.0, y: -1.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: None,
            texture_specular: None,
            texture_normal: None,
            texture_emission: Some(Box::new(TextureSolid::new())),
        }
    )));
    

    // floor
    objs.push(Box::new(Plane::new(
        Vec3 { x: 0.0, y: -5.0, z: 0.0, },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: None,
            texture_specular: None,
            texture_normal: None,
            texture_emission: Some(Box::new(TextureSolid::new())),
        }
    )));

    
    // left wall
    objs.push(Box::new(Plane::new(
        Vec3 { x: -5.0, y: 0.0, z: 0.0, },
        Vec3 { x: 1.0, y: 0.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: None,
            texture_specular: None,
            texture_normal: None,
            texture_emission: Some(Box::new(TextureSolid::new())),
        }
    )));

    // right wall
    objs.push(Box::new(Plane::new(
        Vec3 { x: 5.0, y: 0.0, z: 0.0, },
        Vec3 { x: -1.0, y: 0.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: None,
            texture_specular: None,
            texture_normal: None,
            texture_emission: Some(Box::new(TextureSolid::new())),
        }
    )));

    // back wall
    objs.push(Box::new(Plane::new(
        Vec3 { x: 0.0, y: 0.0, z: -15.0, },
        Vec3 { x: 0.0, y: 0.0, z: 1.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        Material {
            texture_albedo: None,
            texture_specular: None,
            texture_normal: None,
            texture_emission: Some(Box::new(TextureSolid::new())),
        }
    )));

    // near wall
    objs.push(Box::new(Plane::new(
        Vec3 { x: 0.0, y: 0.0, z: 1.0, },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        Material {
            texture_albedo: None,
            texture_specular: None,
            texture_normal: None,
            texture_emission: Some(Box::new(TextureSolid::new())),
        }
    )));
    
    return objs;
}

pub fn construct_wallpaper_scene() -> ObjectVec {
    let mut objs: ObjectVec = Vec::new();

    // floor
    objs.push(Box::new(Plane::new(
        Vec3 { x: 0.0, y: -5.0, z: 0.0, },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: Some(Box::new(TextureSolid::new())),
            texture_specular: None,
            texture_normal: None,
            texture_emission: None,
        }
    )));

    for x_inc in 0..COUNT_X {
        for z_inc in 0..COUNT_Z {
            let x = x_inc as f32 * SPACING;
            let z = z_inc as f32 * -1.0 * SPACING;
            let y = (&(&Vec3 { x: x, y: ORIGIN.y, z: z } - &ORIGIN) * (1.0 / 50.0)).len_squared();

            objs.push(Box::new(Sphere::new(
                &ORIGIN + &Vec3 { x, y, z },
                0.5,
                Material {
                    texture_albedo: None,
                    texture_specular: None,
                    texture_normal: None,
                    texture_emission: Some(Box::new(TextureSolid { color: Color(0.0, z_inc as f32 / 4.0, x_inc as f32 / 8.0) }))
                }
            )));
        }
    }

    return objs;
}

const COUNT_X: usize = 8;
const COUNT_Z: usize = 4;
const SPACING: f32 = 2.0;
const ORIGIN: Vec3 = Vec3 { 
    x: -1.0 * ((COUNT_X - 1) as f32 * SPACING / 2.0),
    z: -12.0,
    y: -4.0
};