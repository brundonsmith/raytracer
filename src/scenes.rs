
use crate::color::Color;
use crate::material::Material;
use crate::vec3::Vec3;
use crate::sphere::Sphere;
use crate::mesh::{Mesh};
use crate::plane::Plane;
use crate::texture::Texture;
use crate::matrix::Matrix;
use crate::object::ObjectEnum;

pub fn construct_reflect_scene() -> Vec<ObjectEnum> {
    let mut objs: Vec<ObjectEnum> = Vec::new();

    objs.push(ObjectEnum::Sphere(Sphere::new(
        Vec3 { x: 5.0, y: 5.0, z: -12.0 },
        5.0,
        Material {
            texture_albedo: None,//Some(Texture::Solid(Color::gray(1.0))),
            texture_specular: None,//Some(Texture::Solid(Color::gray(1.0))),
            texture_normal: None,
            texture_emission_color: None,
            texture_emission_intensity: Some(Texture::Solid(Color::gray(10.0)))
        }
    )));

    objs.push(ObjectEnum::Plane(Plane::new(
        Vec3 { x: 0.0, y: -1.5, z: 0.0, },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: Some(Texture::Solid(Color::gray(1.0))),
            texture_specular: None,
            texture_normal: Some(Texture::from_image("/Users/brundolf/git/raytracer/cobblestone_normal.jpg")),
            texture_emission_color: None,
            texture_emission_intensity: None,
        }
    )));

    return objs;
}

pub fn construct_material_scene() -> Vec<ObjectEnum> {
    let mut objs: Vec<ObjectEnum> = Vec::new();

    objs.push(ObjectEnum::Sphere(Sphere::new(
        Vec3 { x: 0.0, y: 0.0, z: -12.0 },
        1.0,
        Material {
            texture_albedo: None,//Some(Texture::Solid(Color::gray(1.0))),
            texture_specular: None,//Some(Texture::Solid(Color::gray(1.0))),
            texture_normal: None,
            texture_emission_color: Some(Texture::Solid(Color(1.0,0.0,0.0))),
            texture_emission_intensity: Some(Texture::Solid(Color::gray(1.0)))
        }
    )));

    // ceiling
    objs.push(ObjectEnum::Plane(Plane::new(
        Vec3 { x: 0.0, y: 5.0, z: 0.0, },
        Vec3 { x: 0.0, y: -1.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: None,//Some(Box::new(TextureSolid { color: Color(1.0, 0.95, 0.8) })),
            texture_specular: None,
            texture_normal: None,
            texture_emission_color: None,
            texture_emission_intensity: Some(Texture::Solid(Color::gray(1.0))),
        }
    )));

    objs.push(ObjectEnum::Plane(Plane::new(
        Vec3 { x: 0.0, y: -1.5, z: 0.0, },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: None,//Some(Texture::Solid(Color::gray(1.0))),
            texture_specular: Some(Texture::from_image("/Users/brundolf/git/raytracer/CobblestoneSpecular.jpg")),
            texture_normal: None,
            texture_emission_color: None,
            texture_emission_intensity: None,
        }
    )));

    return objs;
}

pub fn construct_tree_scene() -> Vec<ObjectEnum> {
    let mut objs: Vec<ObjectEnum> = Vec::new();

    objs.push(ObjectEnum::Mesh(Mesh::from_obj(
        "/Users/brundolf/git/raytracer/tree.obj", 
        &(Matrix::translation(&Vec3 { x: 0.0, y: 0.0, z: -3.0 })
        * Matrix::rotation_y(std::f32::consts::PI / -4.0)),
        None
    )));

    objs.push(ObjectEnum::Plane(Plane::new(
        Vec3 { x: 2.0, y: 0.0, z: 2.0, },
        Vec3 { x: -1.0, y: 0.0, z: -1.0 },
        Vec3 { x: 1.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: None,
            texture_specular: None,
            texture_normal: None,
            texture_emission_color: Some(Texture::Solid(Color(1.0, 0.52, 0.17))),
            texture_emission_intensity: Some(Texture::Solid(Color::gray(1.0)))
        }
    )));

    return objs;
}

pub fn construct_room_scene() -> Vec<ObjectEnum> {
    let mut objs: Vec<ObjectEnum> = Vec::new();

    // spheres
    objs.push(ObjectEnum::Sphere(Sphere::new(
        Vec3 { x: 3.0, y: -3.0, z: -13.0 },
        3.0,
        Material {
            texture_albedo: Some(Texture::Solid(Color::gray(1.0))),
            texture_specular: None,//Some(Texture::Solid(Color::gray(1.0))),
            texture_normal: Some(Texture::from_image("/Users/brundolf/git/raytracer/cobblestone_normal.jpg")),
            texture_emission_color: Some(Texture::Solid(Color(0.0, 1.0, 1.0))),
            texture_emission_intensity: Some(Texture::Solid(Color::gray(5.0)))
        }
    )));

    objs.push(ObjectEnum::Sphere(Sphere::new(
        Vec3 { x: -2.0, y: 0.0, z: -8.0 },
        1.0,
        Material {
            texture_albedo: None,//Some(Texture::Solid(Color::gray(1.0))),
            texture_specular: Some(Texture::Solid(Color::gray(1.0))),
            texture_normal: None,//Some(Texture::from_image("/Users/brundolf/git/raytracer/cobblestone_normal.jpg")),
            texture_emission_color: None,
            texture_emission_intensity: None,
        }
    )));

    /*
    objs.push(Box::new(Mesh::from_obj(
        "/Users/brundolf/git/raytracer/test.obj", 
        &(Matrix::translation(&Vec3 { x: 0.0, y: -3.0, z: -10.0 })
        * Matrix::rotation_y(std::f32::consts::PI)
        * Matrix::scale(&Vec3::from_scalar(0.5))),
        None
    )));*/
    
    // ceiling
    objs.push(ObjectEnum::Plane(Plane::new(
        Vec3 { x: 0.0, y: 5.0, z: 0.0, },
        Vec3 { x: 0.0, y: -1.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: Some(Texture::Solid(Color::gray(1.0))),
            texture_specular: None,
            texture_normal: None,
            texture_emission_color: None,//Some(Texture::Solid(Color(1.0, 0.95, 0.8))),
            texture_emission_intensity: None,//Some(Texture::Solid(Color(1.0, 0.95, 0.8))),
        }
    )));

    // floor
    objs.push(ObjectEnum::Plane(Plane::new(
        Vec3 { x: 0.0, y: -5.0, z: 0.0, },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: Some(Texture::Solid(Color::gray(1.0))),
            texture_specular: None,
            texture_normal: Some(Texture::from_image("/Users/brundolf/git/raytracer/cobblestone_normal.jpg")),
            texture_emission_color: None,
            texture_emission_intensity: None,
        }
    )));

    
    // left wall
    objs.push(ObjectEnum::Plane(Plane::new(
        Vec3 { x: -5.0, y: 0.0, z: 0.0, },
        Vec3 { x: 1.0, y: 0.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: Some(Texture::Solid(Color(1.0,0.0,0.0))),
            texture_specular: None,
            texture_normal: Some(Texture::from_image("/Users/brundolf/git/raytracer/cobblestone_normal.jpg")),
            texture_emission_color: None,
            texture_emission_intensity: None,
        }
    )));

    // right wall
    objs.push(ObjectEnum::Plane(Plane::new(
        Vec3 { x: 5.0, y: 0.0, z: 0.0, },
        Vec3 { x: -1.0, y: 0.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: Some(Texture::Solid(Color(0.0, 1.0, 0.0))),
            texture_specular: None,//Some(Texture::Solid(Color::gray(1.0))),
            texture_normal: Some(Texture::from_image("/Users/brundolf/git/raytracer/cobblestone_normal.jpg")),
            texture_emission_color: None,
            texture_emission_intensity: None,
        }
    )));

    // back wall
    objs.push(ObjectEnum::Plane(Plane::new(
        Vec3 { x: 0.0, y: 0.0, z: -15.0, },
        Vec3 { x: 0.0, y: 0.0, z: 1.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        Material {
            texture_albedo: Some(Texture::Solid(Color::gray(1.0))),
            texture_specular: None,
            texture_normal: None,//Some(Texture::from_image("/Users/brundolf/git/raytracer/cobblestone_normal.jpg")),
            texture_emission_color: None,
            texture_emission_intensity: None,
        }
    )));

    // near wall
    objs.push(ObjectEnum::Plane(Plane::new(
        Vec3 { x: 0.0, y: 0.0, z: 1.0, },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        Material {
            texture_albedo: Some(Texture::Solid(Color(0.0,0.0,1.0))),
            texture_specular: None,
            texture_normal: None,
            texture_emission_color: None,
            texture_emission_intensity: None,
        }
    )));
    
    return objs;
}


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
                texture_albedo: Some(Texture::Procedural(&checker)),
                texture_specular: None,
                texture_emission: None,
            }
        )))
    }*/


pub fn construct_plane_texture_test() -> Vec<ObjectEnum> {
    let mut objs: Vec<ObjectEnum> = Vec::new();

    objs.push(ObjectEnum::Plane(Plane::new(
        Vec3 { x: 0.0, y: 5.0, z: 0.0, },
        Vec3 { x: 0.0, y: -1.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: None,
            texture_specular: None,
            texture_normal: None,
            texture_emission_color: None,
            texture_emission_intensity: Some(Texture::Solid(Color::gray(1.0))),
        }
    )));

    objs.push(ObjectEnum::Plane(Plane::new(
        Vec3 { x: 0.0, y: -1.5, z: 0.0, },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: Some(Texture::from_image("/Users/brundolf/git/raytracer/grid.jpg")),
            texture_specular: None,
            texture_normal: None,
            texture_emission_color: None,
            texture_emission_intensity: None,
        }
    )));

    return objs;
}

pub fn construct_sphere_texture_test() -> Vec<ObjectEnum> {
    let mut objs: Vec<ObjectEnum> = Vec::new();
    
    objs.push(ObjectEnum::Sphere(Sphere::new(
        Vec3 { x: 0.0, y: 0.0, z: -5.0 },
        1.0,
        Material {
            texture_albedo: Some(Texture::from_image("C:\\Users\\Brundon\\git\\raytracer\\texture.jpg")),
            texture_specular: None,//Some(Texture::Solid(Color::gray(1.0))),
            texture_normal: None,
            texture_emission_color: None,
            texture_emission_intensity: None,
        }
    )));

    
    // ceiling
    objs.push(ObjectEnum::Plane(Plane::new(
        Vec3 { x: 0.0, y: 5.0, z: 0.0, },
        Vec3 { x: 0.0, y: -1.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: None,
            texture_specular: None,
            texture_normal: None,
            texture_emission_color: None,
            texture_emission_intensity: Some(Texture::Solid(Color::gray(1.0))),
        }
    )));
    

    // floor
    objs.push(ObjectEnum::Plane(Plane::new(
        Vec3 { x: 0.0, y: -5.0, z: 0.0, },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: None,
            texture_specular: None,
            texture_normal: None,
            texture_emission_color: None,
            texture_emission_intensity: Some(Texture::Solid(Color::gray(1.0))),
        }
    )));

    
    // left wall
    objs.push(ObjectEnum::Plane(Plane::new(
        Vec3 { x: -5.0, y: 0.0, z: 0.0, },
        Vec3 { x: 1.0, y: 0.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: None,
            texture_specular: None,
            texture_normal: None,
            texture_emission_color: None,
            texture_emission_intensity: Some(Texture::Solid(Color::gray(1.0))),
        }
    )));

    // right wall
    objs.push(ObjectEnum::Plane(Plane::new(
        Vec3 { x: 5.0, y: 0.0, z: 0.0, },
        Vec3 { x: -1.0, y: 0.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: None,
            texture_specular: None,
            texture_normal: None,
            texture_emission_color: None,
            texture_emission_intensity: Some(Texture::Solid(Color::gray(1.0))),
        }
    )));

    // back wall
    objs.push(ObjectEnum::Plane(Plane::new(
        Vec3 { x: 0.0, y: 0.0, z: -15.0, },
        Vec3 { x: 0.0, y: 0.0, z: 1.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        Material {
            texture_albedo: None,
            texture_specular: None,
            texture_normal: None,
            texture_emission_color: None,
            texture_emission_intensity: Some(Texture::Solid(Color::gray(1.0))),
        }
    )));

    // near wall
    objs.push(ObjectEnum::Plane(Plane::new(
        Vec3 { x: 0.0, y: 0.0, z: 1.0, },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        Material {
            texture_albedo: None,
            texture_specular: None,
            texture_normal: None,
            texture_emission_color: None,
            texture_emission_intensity: Some(Texture::Solid(Color::gray(1.0))),
        }
    )));
    
    return objs;
}

pub fn construct_wallpaper_scene() -> Vec<ObjectEnum> {
    let mut objs: Vec<ObjectEnum> = Vec::new();

    // floor
    objs.push(ObjectEnum::Plane(Plane::new(
        Vec3 { x: 0.0, y: -5.0, z: 0.0, },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: Some(Texture::Solid(Color::gray(1.0))),
            texture_specular: None,
            texture_normal: None,
            texture_emission_color: None,
            texture_emission_intensity: None,
        }
    )));

    for x_inc in 0..COUNT_X {
        for z_inc in 0..COUNT_Z {
            let x = x_inc as f32 * SPACING;
            let z = z_inc as f32 * -1.0 * SPACING;
            let y = (&(&Vec3 { x: x, y: ORIGIN.y, z: z } - &ORIGIN) * (1.0 / 50.0)).len_squared();

            objs.push(ObjectEnum::Sphere(Sphere::new(
                &ORIGIN + &Vec3 { x, y, z },
                0.5,
                Material {
                    texture_albedo: None,
                    texture_specular: None,
                    texture_normal: None,
                    texture_emission_color: Some(Texture::Solid(Color(0.0, z_inc as f32 / 4.0, x_inc as f32 / 8.0))),
                    texture_emission_intensity: Some(Texture::Solid(Color::gray(1.0)))
                }
            )));
        }
    }

    return objs;
}

pub fn construct_wallpaper_scene_2() -> Vec<ObjectEnum> {
    let mut objs: Vec<ObjectEnum> = Vec::new();

    objs.push(ObjectEnum::Mesh(Mesh::from_obj(
        "/Users/brundolf/git/raytracer/Geometric.obj", 
        &(Matrix::translation(&Vec3 { x: -0.5, y: 0.0, z: -8.0 })
        * Matrix::rotation_x(std::f32::consts::PI * -1.0 / 8.0)
        * Matrix::rotation_y(std::f32::consts::PI * -1.0 / 8.0)
        /*
          Matrix::rotation_y(std::f32::consts::PI * 5.0 / 4.0) *
          Matrix::rotation_x(std::f32::consts::PI * 1.0 / 4.0)*/),
        Some(Material {
            texture_albedo: None,
            texture_specular: Some(Texture::Solid(Color::gray(0.8))),
            texture_normal: None,
            texture_emission_color: None,
            texture_emission_intensity: None,
        })
    )));

    for a in 0..8 {
        for b in 0..4 {
            let a_portion = a as f32 / 8.0;
            let b_portion = b as f32 / 4.0;

            let longitude = (2.0 * std::f32::consts::PI) * a_portion + (std::f32::consts::PI / 8.0);
            let latitude = std::f32::consts::PI * b_portion - std::f32::consts::PI / 2.0;

            println!("{}, {}", longitude, latitude);

            let pos = &Vec3::from_angles(longitude, latitude) * 3.0;

            objs.push(ObjectEnum::Sphere(Sphere::new(
                &Vec3 { x: 0.0, y: 0.0, z: -8.0 } + &pos,
                0.5,
                Material {
                    texture_albedo: None,
                    texture_specular: None,
                    texture_normal: None,
                    texture_emission_color: Some(Texture::Solid(Color(1.0, 0.0, 0.0))),
                    texture_emission_intensity: Some(Texture::Solid(Color::gray(5.0)))
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