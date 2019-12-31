
use criterion::{black_box, Criterion};

extern crate rand;
use rand::prelude::*;
use rand::{thread_rng};
use rand::rngs::SmallRng;

use raytracer::ray::Ray;
use raytracer::vec3::Vec3;
use raytracer::color::Color;
use raytracer::material::Material;
use raytracer::sphere::Sphere;
use raytracer::mesh::{Mesh};
use raytracer::plane::Plane;
use raytracer::texture::Texture;
use raytracer::matrix::Matrix;
use raytracer::utils::{ObjectVec};

const TEST_RAY_1: Ray = Ray {
    origin: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
    direction: Vec3 { x: -0.5, y: -0.5, z: -1.0 }
};

pub fn cast_ray_1(c: &mut Criterion) {
    let mut meta_rng = thread_rng();
    let mut rng = SmallRng::from_rng(&mut meta_rng).unwrap();
    let objs = construct_room_scene();

    c.bench_function("cast_ray_1([TEST_RAY_1], [objs], [rng], 3)", |b| 
        b.iter(|| raytracer::cast::cast_ray(black_box(&TEST_RAY_1), &objs, &mut rng, black_box(3))));
}


const TEST_RAY_2: Ray = Ray {
    origin: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
    direction: Vec3 { x: 0.0, y: -0.5, z: -1.0 }
};

pub fn cast_ray_2(c: &mut Criterion) {
    let mut meta_rng = thread_rng();
    let mut rng = SmallRng::from_rng(&mut meta_rng).unwrap();
    let objs = construct_room_scene();

    c.bench_function("cast_ray_2([TEST_RAY_2], [objs], [rng], 3)", |b| 
        b.iter(|| raytracer::cast::cast_ray(black_box(&TEST_RAY_2), &objs, &mut rng, black_box(3))));
}


const TEST_RAY_3: Ray = Ray {
    origin: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
    direction: Vec3 { x: -0.5, y: 0.0, z: -1.0 }
};

pub fn cast_ray_3(c: &mut Criterion) {
    let mut meta_rng = thread_rng();
    let mut rng = SmallRng::from_rng(&mut meta_rng).unwrap();
    let objs = construct_room_scene();

    c.bench_function("cast_ray_3([TEST_RAY_3], [objs], [rng], 3)", |b| 
        b.iter(|| raytracer::cast::cast_ray(black_box(&TEST_RAY_3), &objs, &mut rng, black_box(3))));
}


fn construct_room_scene() -> ObjectVec {
    let mut objs: ObjectVec = Vec::new();

    // spheres
    objs.push(Box::new(Sphere::new(
        Vec3 { x: 3.0, y: -3.0, z: -13.0 },
        1.0,
        Material {
            texture_albedo: None,
            texture_specular: None,
            texture_normal: None,
            texture_emission_color: Some(Texture::Solid(Color(0.0, 1.0, 1.0))),
            texture_emission_intensity: Some(Texture::Solid(Color::gray(1.0))),
        }
    )));

    objs.push(Box::new(Sphere::new(
        Vec3 { x: -2.0, y: 0.0, z: -8.0 },
        1.0,
        Material {
            texture_albedo: None,
            texture_specular: Some(Texture::Solid(Color(1.0,1.0,1.0))),
            texture_normal: None,
            texture_emission_color: None,
            texture_emission_intensity: None,
        }
    )));

    // mesh
    objs.push(Box::new(Mesh::from_obj(
        "/Users/brundolf/git/raytracer/test.obj", 
        &(&Matrix::translation(&Vec3 { x: 0.0, y: -3.0, z: -10.0 }) *
        &(&Matrix::rotation_y(std::f32::consts::PI) *
          &Matrix::scale(&Vec3::from_scalar(0.5))))
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
            texture_emission_color: Some(Texture::Solid(Color(1.0, 0.95, 0.8))),
            texture_emission_intensity: Some(Texture::Solid(Color::gray(1.0))),
        }
    )));

    // floor
    objs.push(Box::new(Plane::new(
        Vec3 { x: 0.0, y: -5.0, z: 0.0, },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: Some(Texture::from_image("/Users/brundolf/git/raytracer/texture.jpg")),
            texture_specular: None,
            texture_normal: None,
            texture_emission_color: None,
            texture_emission_intensity: None,
        }
    )));

    
    // left wall
    objs.push(Box::new(Plane::new(
        Vec3 { x: -5.0, y: 0.0, z: 0.0, },
        Vec3 { x: 1.0, y: 0.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: Some(Texture::Solid(Color(1.0,0.0,0.0))),
            texture_specular: None,
            texture_normal: None,
            texture_emission_color: None,
            texture_emission_intensity: None,
        }
    )));

    // right wall
    objs.push(Box::new(Plane::new(
        Vec3 { x: 5.0, y: 0.0, z: 0.0, },
        Vec3 { x: -1.0, y: 0.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        Material {
            texture_albedo: Some(Texture::Solid(Color(0.0, 1.0, 0.0))),
            texture_specular: None,
            texture_normal: None,
            texture_emission_color: None,
            texture_emission_intensity: None,
        }
    )));

    // back wall
    objs.push(Box::new(Plane::new(
        Vec3 { x: 0.0, y: 0.0, z: -15.0, },
        Vec3 { x: 0.0, y: 0.0, z: 1.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        Material {
            texture_albedo: Some(Texture::Solid(Color(1.0,1.0,1.0))),
            texture_specular: None,
            texture_normal: None,
            texture_emission_color: None,
            texture_emission_intensity: None,
        }
    )));

    // near wall
    objs.push(Box::new(Plane::new(
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