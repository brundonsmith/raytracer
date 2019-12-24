use rand::rngs::SmallRng;
use std::fs;
use std::collections::HashMap;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::object::Object;
use crate::intersection::Intersection;
use crate::material::Material;
use crate::utils::{plane_intersection,ObjectVec};
use crate::sphere::Sphere;
use crate::obj_parser::{parse,LineType};
use crate::matrix::Matrix;
use crate::illumination::Illumination;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Face (pub usize, pub usize, pub usize);

pub struct Mesh {
    materials: HashMap<String,Material>,

    vertices: Vec<Vec3>,
    faces: Vec<Face>,
    uv_coords: Vec<(f32,f32)>,
    face_materials: HashMap<usize,String>,

    bounding_sphere: Sphere,
}

impl Mesh {

    pub fn new(materials: HashMap<String,Material>, vertices: Vec<Vec3>, faces: Vec<Face>, face_materials: HashMap<usize,String>, uv_coords: Vec<(f32,f32)>) -> Self {
        let bounding_sphere = get_bounding_sphere(&vertices);

        Self {
            materials,
            vertices,
            faces,
            uv_coords,
            face_materials,

            bounding_sphere
        }
    }

    pub fn from_obj(path: &str, transform: &Matrix, materials: HashMap<String,Material>) -> Self {
        let data = fs::read_to_string(path).expect("Failed to open mesh file");

        println!("Loading obj...");

        let mut vertices = Vec::new();
        let mut faces = Vec::new();
        let mut face_materials = HashMap::new();
        let uv_coords = Vec::new();

        let mut current_mat: Option<String> = None;
        for line in parse(&data) {
            match line {
                LineType::Vertex(x, y, z) => vertices.push(Vec3 { x, y, z }.transformed(transform)),
                LineType::Face(v0, v1, v2) => {
                    current_mat.as_ref().map(|name| 
                        face_materials.insert(faces.len(), name.clone())
                    );
                    faces.push(Face(v0.0, v1.0, v2.0));
                },
                LineType::UseMaterial(name) => current_mat = Some(name),
                _ => ()
            }
        }

        println!("done");
        
        return Mesh::new(materials, vertices, faces, face_materials, uv_coords);
    }

    fn inner_intersection(&self, ray: &Ray) -> Option<(Intersection,usize)> {
        if self.bounding_sphere.intersection(ray).is_none() {
            return None;
        } else {
            let mut nearest_intersection: Option<(Intersection,usize)> = None;

            for face_index in 0..self.faces.len() {
                let face = self.faces[face_index];

                let vert0 = &self.vertices[face.0];
                let vert1 = &self.vertices[face.1];
                let vert2 = &self.vertices[face.2];
                let normal = triangle_normal(&vert0, &vert1, &vert2);

                match plane_intersection(&vert0, &normal, ray) {
                    Some(intersection) => {
                        if intersection.distance > 0.0 && nearest_intersection.as_ref().map(|nearest| intersection.distance < nearest.0.distance).unwrap_or(true) {
                            let edge0 = vert1 - vert0; 
                            let edge1 = vert2 - vert1; 
                            let edge2 = vert0 - vert2; 
                            let c0 = &intersection.position - vert0; 
                            let c1 = &intersection.position - vert1; 
                            let c2 = &intersection.position - vert2; 
                            if  normal.dot(&edge0.cross(&c0)) > 0.0 && 
                                normal.dot(&edge1.cross(&c1)) > 0.0 && 
                                normal.dot(&edge2.cross(&c2)) > 0.0 {

                                nearest_intersection = Some((intersection, face_index)); // P is inside the triangle
                            }
                        }
                    },
                    None => ()
                };
            }

            return nearest_intersection;
        }
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

    let radius =
        f32::max(f32::abs(max.x - center.x),
        f32::max(f32::abs(max.y - center.y),
        f32::max(f32::abs(max.z - center.z),
        f32::max(f32::abs(min.x - center.x),
        f32::max(f32::abs(min.y - center.y),
        f32::max(f32::abs(min.z - center.z), 0.0))))));

    return Sphere::new(center, radius, Material::new());
}

impl Object for Mesh {

    fn intersection(&self, ray: &Ray) -> Option<Intersection> {
        self.inner_intersection(ray).map(|int| int.0)
    }

    fn texture_coordinate(&self, point: &Vec3) -> (f32,f32) {
        // TODO
        (0.0, 0.0)
    }

    fn shade(&self, ray: &Ray, objs: &ObjectVec, rng: &mut SmallRng, depth: u8) -> Illumination {
        let mut intersection = self.inner_intersection(ray).unwrap();

        let material_name = self.face_materials.get(&intersection.1);
        let material = material_name.map(|name| self.materials.get(name).unwrap())
            .unwrap_or(self.materials.iter().next().map(|entry| entry.1).unwrap());

        let uv = self.texture_coordinate(&intersection.0.position);

        material.shade(&mut intersection.0, uv, objs, rng, depth)
    }
}

fn triangle_normal(vert0: &Vec3, vert1: &Vec3, vert2: &Vec3) -> Vec3 {
    (vert1 - vert0).cross(&(vert2 - vert0))
}