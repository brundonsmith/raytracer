
use std::fs;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::object::Object;
use crate::intersection::Intersection;
use crate::material::Material;
use crate::utils::{plane_intersection};
use crate::sphere::Sphere;
use crate::obj_parser::{parse,LineType};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Face (pub usize, pub usize, pub usize);

pub struct Mesh {
    position: Vec3,
    material: Material,

    vertices: Vec<Vec3>,
    faces: Vec<Face>,
    uv_coords: Vec<(f32,f32)>,

    bounding_sphere: Sphere,
}

impl Mesh {

    pub fn new(position: Vec3, material: Material, vertices: Vec<Vec3>, faces: Vec<Face>, uv_coords: Vec<(f32,f32)>) -> Self {
        let bounding_sphere = get_bounding_sphere(&position, &vertices);

        Self {
            position,
            material,
            vertices,
            faces,
            uv_coords,

            bounding_sphere
        }
    }

    pub fn from_obj(path: &str, position: Vec3, material: Material) -> Self {
        let data = fs::read_to_string(path).expect("Failed to open mesh file");

        println!("Loading obj...");

        let mut vertices = Vec::new();
        let mut faces = Vec::new();
        let uv_coords = Vec::new();


        for line in parse(&data) {
            match line {
                LineType::Vertex(x, y, z) => vertices.push(Vec3 { x, y, z }),
                LineType::Face(v0, v1, v2) => faces.push(Face(v0.0, v1.0, v2.0)),
                _ => ()
            }
        }
        
        println!("done");
        
        return Mesh::new(position, material, vertices, faces, uv_coords);
    }
}

fn get_bounding_sphere(position: &Vec3, vertices: &Vec<Vec3>) -> Sphere {
    let mut farthest_vertex_squared = 0.0;

    for v in vertices {
        let len_squared = (v - position).len_squared();
        if len_squared > farthest_vertex_squared {
            farthest_vertex_squared = len_squared;
        }
    }

    return Sphere::new(position.clone(), farthest_vertex_squared.sqrt(), Material::new());
}

impl Object for Mesh {

    fn intersection(&self, ray: &Ray) -> Option<Intersection> {
        
        if self.bounding_sphere.intersection(ray).is_none() {
            return None;
        } else {
            let mut nearest_intersection: Option<Intersection> = None;

            for face in &self.faces {
                let vert0 = &(&self.vertices[face.0] * 0.5) + &self.position;
                let vert1 = &(&self.vertices[face.1] * 0.5) + &self.position;
                let vert2 = &(&self.vertices[face.2] * 0.5) + &self.position;
                let normal = triangle_normal(&vert0, &vert1, &vert2);

                match plane_intersection(&vert0, &normal, ray) {
                    Some(intersection) => {
                        if intersection.distance > 0.0 && nearest_intersection.as_ref().map(|nearest| intersection.distance < nearest.distance).unwrap_or(true) {
                            let edge0 = &vert1 - &vert0; 
                            let edge1 = &vert2 - &vert1; 
                            let edge2 = &vert0 - &vert2; 
                            let c0 = &intersection.position - &vert0; 
                            let c1 = &intersection.position - &vert1; 
                            let c2 = &intersection.position - &vert2; 
                            if  normal.dot(&edge0.cross(&c0)) > 0.0 && 
                                normal.dot(&edge1.cross(&c1)) > 0.0 && 
                                normal.dot(&edge2.cross(&c2)) > 0.0 {

                                nearest_intersection = Some(intersection); // P is inside the triangle
                            }
                        }
                    },
                    None => ()
                };
            }

            return nearest_intersection;
        }
    }

    fn texture_coordinate(&self, point: &Vec3) -> (f32,f32) {
        // TODO
        (0.0, 0.0)
    }

    fn get_position(&self) -> &Vec3 {
        &self.position
    }

    fn get_material(&self) -> &Material {
        &self.material
    }
}

fn triangle_normal(vert0: &Vec3, vert1: &Vec3, vert2: &Vec3) -> Vec3 {
    (vert1 - vert0).cross(&(vert2 - vert0))
}