

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::object::Object;
use crate::intersection::Intersection;
use crate::material::Material;
use crate::utils::{plane_intersection};
use crate::obj_importer::import_obj;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Face (pub usize, pub usize, pub usize);

pub struct Mesh {
    pub position: Vec3,
    pub material: Material,

    pub vertices: Vec<Vec3>,
    pub faces: Vec<Face>,
    pub uv_coords: Vec<(f32,f32)>
}

impl Mesh {

    pub fn new() -> Self {
        Self {
            position: Vec3::new(),
            material: Material {
                texture_albedo: None,
                texture_specular: None,
                texture_emission: None,
            },
            vertices: Vec::new(),
            faces: Vec::new(),
            uv_coords: Vec::new(),
        }
    }

    pub fn from_obj(path: &str) -> Self {
        import_obj(path)
    }
}

impl Object for Mesh {

    fn intersection(&self, ray: &Ray) -> Option<Intersection> {
        let mut nearest_intersection: Option<Intersection> = None;
        //println!("begin");

        for face in &self.faces {
            let vert0 = &(&self.vertices[face.0] * 0.5) + &self.position;
            //println!("{:?}", &vert0);
            let vert1 = &(&self.vertices[face.1] * 0.5) + &self.position;
            let vert2 = &(&self.vertices[face.2] * 0.5) + &self.position;
            let normal = triangle_normal(&vert0, &vert1, &vert2);

            match plane_intersection(&vert0, &normal, ray) {
                Some(intersection) => {
                    //println!("some intersection");
                    if intersection.distance > 0.0 && nearest_intersection.as_ref().map(|nearest| intersection.distance < nearest.distance).unwrap_or(true) {
                        //println!("intersection of interest");
                        let edge0 = &vert1 - &vert0; 
                        let edge1 = &vert2 - &vert1; 
                        let edge2 = &vert0 - &vert2; 
                        let c0 = &intersection.position - &vert0; 
                        let c1 = &intersection.position - &vert1; 
                        let c2 = &intersection.position - &vert2; 
                        //println!("check");
                        if  normal.dot(&edge0.cross(&c0)) > 0.0 && 
                            normal.dot(&edge1.cross(&c1)) > 0.0 && 
                            normal.dot(&edge2.cross(&c2)) > 0.0 {
                            //println!("found");
                            nearest_intersection = Some(intersection); // P is inside the triangle
                        }
                    }
                },
                None => ()
            };

            //println!("end loop");
        }

        //println!("return");
        return nearest_intersection;
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