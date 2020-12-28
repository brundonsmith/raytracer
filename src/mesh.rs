use rand::rngs::SmallRng;
//use flamer::flame;
use std::fs;
use std::collections::HashMap;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::object::{Object,ObjectEnum};
use crate::intersection::Intersection;
use crate::material::Material;
use crate::utils::{plane_intersection,adjusted_for_normal,color_to_normal};
use crate::sphere::Sphere;
use crate::obj_parser::{parse,LineType};
use crate::mtl_parser::{load_and_parse};
use crate::matrix::Matrix;
use crate::illumination::Illumination;
use crate::texture::Texture;
use crate::color::Color;

const DEFAULT_MATERIAL: Material = Material {
    texture_albedo: Some(Texture::Solid(Color(1.0,1.0,1.0))),
    texture_specular: None,
    texture_normal: None,
    texture_emission_color: None,
    texture_emission_intensity: None,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Face {
    pub v0: usize,
    pub v1: usize,
    pub v2: usize,

    pub mat: Option<usize>,
    pub normal: Vec3,
}

pub struct Mesh {
    materials: Vec<Material>,
    default_material: Material,

    vertices: Vec<Vec3>,
    faces: Vec<Face>,
    uv_coords: Vec<(f32,f32)>,

    bounding_sphere: Sphere,
}

impl Mesh {

    pub fn from_obj(path: &str, transform: &Matrix, default_material: Option<Material>) -> Self {
        let data = fs::read_to_string(path).expect("Failed to open mesh file");

        println!("Loading obj...");

        let mut vertices = Vec::new();
        let mut faces = Vec::new();
        let uv_coords = Vec::new();

        let mut materials: Vec<Material> = Vec::new();
        let mut material_names: HashMap<String,usize> = HashMap::new();

        let mut current_mat: Option<usize> = None;
        for line in parse(&data) {
            match line {
                LineType::Vertex(x, y, z) => vertices.push(Vec3 { x, y, z }.transformed(transform)),
                LineType::Face(v0, v1, v2) => {

                    let vert0 = &vertices[v0.0];
                    let vert1 = &vertices[v1.0];
                    let vert2 = &vertices[v2.0];
                    let normal = triangle_normal(&vert0, &vert1, &vert2);

                    faces.push(Face {
                        v0: v0.0, 
                        v1: v1.0,
                        v2: v2.0,

                        mat: current_mat,
                        normal,
                    });
                },
                LineType::MTLib(file) => {
                    let segments: Vec<&str> = path.split("/").collect();
                    let mut local_dir = String::new();
                    for i in 0..segments.len() - 1 {
                        local_dir += segments[i];
                        local_dir += "/";
                    }

                    for (name, mat) in load_and_parse(&(local_dir + &file)) {
                        material_names.insert(name, materials.len());
                        materials.push(mat);
                    }
                },
                LineType::UseMaterial(name) => current_mat = material_names.get(&name).map(|x| *x),
                _ => ()
            }
        }

        let bounding_sphere = get_bounding_sphere(&vertices);
        
        return Self {
            materials,
            default_material: default_material.unwrap_or(DEFAULT_MATERIAL),
            vertices,
            faces,
            uv_coords,
            bounding_sphere,
        }
    }
    
    fn inner_intersection(&self, ray: &Ray) -> Option<(Intersection,usize)> {
//        flame::start("Mesh::sphere_intersect");
        //let sphere_intersect = self.bounding_sphere.intersection(ray);
//        flame::end("Mesh::sphere_intersect");

        if self.bounding_sphere.intersection(ray).is_none() {
            return None;
        } else {
            let mut nearest_intersection: Option<(Intersection,usize)> = None;

            for face_index in 0..self.faces.len() {
                let face = &self.faces[face_index];

                let vert0 = &self.vertices[face.v0];
                let vert1 = &self.vertices[face.v1];
                let vert2 = &self.vertices[face.v2];

                let distance_squared = nearest_intersection.as_ref().map(|i| i.0.distance * i.0.distance);
                if distance_squared.is_none() ||
                        (vert0 - &ray.origin).len_squared() < distance_squared.unwrap() ||
                        (vert1 - &ray.origin).len_squared() < distance_squared.unwrap() ||
                        (vert2 - &ray.origin).len_squared() < distance_squared.unwrap() {

//                    flame::start("Mesh::plane_intersection");
                    let plane_intersect = plane_intersection(&vert0, &face.normal, ray);
//                    flame::end("Mesh::plane_intersection");
                    match plane_intersect {
                        Some(intersection) => {
                            if intersection.distance > 0.0 && nearest_intersection.as_ref().map(|nearest| intersection.distance < nearest.0.distance).unwrap_or(true) {
                                let edge0 = vert1 - vert0; 
                                let edge1 = vert2 - vert1; 
                                let edge2 = vert0 - vert2; 
                                let c0 = &intersection.position - vert0; 
                                let c1 = &intersection.position - vert1; 
                                let c2 = &intersection.position - vert2; 
                                if  face.normal.dot(&edge0.cross(&c0)) > 0.0 && 
                                    face.normal.dot(&edge1.cross(&c1)) > 0.0 && 
                                    face.normal.dot(&edge2.cross(&c2)) > 0.0 {
    
                                    nearest_intersection = Some((intersection, face_index)); // P is inside the triangle
                                }
                            }
                        },
                        None => ()
                    };
                }

            }

            return nearest_intersection;
        }
    }

    fn material_for_face_index(&self, index: usize) -> &Material {
        let face = &self.faces[index];
        return face.mat.map(|i| self.materials.get(i).unwrap_or(&self.default_material)).unwrap_or(&self.default_material);
    }
}

fn get_bounding_sphere(vertices: &Vec<Vec3>) -> Sphere {
    let mut min = Vec3::new();
    let mut max = Vec3::new();

    for v in vertices {
        min.x = f32::min(min.x, v.x);
        min.y = f32::min(min.y, v.y);
        min.z = f32::min(min.z, v.z);

        max.x = f32::max(max.x, v.x);
        max.y = f32::max(max.y, v.y);
        max.z = f32::max(max.z, v.z);
    }

    let center = Vec3 {
        x: (min.x + max.x) / 2.0,
        y: (min.y + max.y) / 2.0,
        z: (min.z + max.z) / 2.0,
    };

    let mut radius = 0.0;
    for v in vertices {
        radius = f32::max(radius, (v - &center).len());
    }

    return Sphere::new(center, radius + 0.001, Material::new());
}

impl Object for Mesh {

//    #[flame("Mesh")]
    fn intersection(&self, ray: &Ray) -> Option<Intersection> {
        self.inner_intersection(ray)
            .map(|mut intersection| {

                self.material_for_face_index(intersection.1).texture_normal.as_ref().map(|texture_normal| {
                    let normal_color = texture_normal.color_at(self.texture_coordinate(&intersection.0.position));
                    intersection.0.normal = adjusted_for_normal(&intersection.0.normal, &color_to_normal(&normal_color));
                });
                
                intersection.0
            })
    }

//    #[flame("Mesh")]
    fn texture_coordinate(&self, point: &Vec3) -> (f32,f32) {
        // TODO
        (0.0, 0.0)
    }

//    #[flame("Mesh")]
    fn shade(&self, ray: &Ray, objs: &Vec<ObjectEnum>, rng: &mut SmallRng, bounces_remaining: u8) -> Illumination {
        let mut intersection = self.inner_intersection(ray).unwrap();

        let material = self.material_for_face_index(intersection.1);

        let uv = self.texture_coordinate(&intersection.0.position);

        material.shade(&mut intersection.0, uv, objs, rng, bounces_remaining)
    }
}

fn triangle_normal(vert0: &Vec3, vert1: &Vec3, vert2: &Vec3) -> Vec3 {
    (vert1 - vert0).cross(&(vert2 - vert0))
}