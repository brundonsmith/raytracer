
use std::fs;

use crate::mesh::{Mesh,Face};
use crate::vec3::Vec3;

pub fn import_obj(path: &str) -> Mesh {
    let data = fs::read_to_string(path).expect("Failed to open mesh file");

    println!("Loading obj...");

    let mut mesh = Mesh::new();

    for line in data.split("\n") {
        match parse(line) {
            LineType::Vertex(x, y, z) => mesh.vertices.push(Vec3 { x, y, z }),
            LineType::Face(v0, v1, v2) => mesh.faces.push(Face(v0.0, v1.0, v2.0)),
            _ => ()
        }
    }
    
    println!("done");

    return mesh;
}

fn parse(line: &str) -> LineType {
    let segments: Vec<&str> = line.trim().split(" ").collect();

    match segments[0] {
        "#" => LineType::Comment(String::from(segments[1])),
        "o" => LineType::Object(String::from(segments[1])),
        "v" => LineType::Vertex(
                    segments[1].parse().ok().unwrap(),
                    segments[2].parse().ok().unwrap(),
                    segments[3].parse().ok().unwrap()),
        "vn" => LineType::VertexNormal(
                    segments[1].parse().ok().unwrap(),
                    segments[2].parse().ok().unwrap(),
                    segments[3].parse().ok().unwrap()),
        "vt" => LineType::VertexTexture(
                    segments[1].parse().ok().unwrap(),
                    segments[2].parse().ok().unwrap()),
        "f" => LineType::Face(
                parse_face_vertex(segments[1]),
                parse_face_vertex(segments[2]),
                parse_face_vertex(segments[3])),

        _ => LineType::Unknown
    }
}

fn parse_face_vertex(segment: &str) -> FaceVertex {
    let nums: Vec<&str> = segment.trim().split("/").collect();

    FaceVertex(
        nums[0].parse::<usize>().ok().unwrap() - 1, // vertices are 1-indexed
        nums[1].parse().ok(), 
        nums[2].parse().ok())
}

#[derive(Debug, Clone, PartialEq)]
enum LineType {
    Comment(String),
    Object(String),
    Vertex(f32, f32, f32),
    VertexNormal(f32, f32, f32),
    VertexTexture(f32, f32),
    Face(FaceVertex, FaceVertex, FaceVertex),

    Unknown,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct FaceVertex (pub usize, pub Option<usize>, pub Option<usize>);