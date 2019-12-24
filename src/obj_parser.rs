
pub fn parse(obj: &str) -> Vec<LineType> {
    let mut lines = Vec::new();
    
    for line in obj.split("\n") {
        lines.push(parse_line(line));
    }

    return lines;
}

pub fn parse_line(line: &str) -> LineType {
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
        "usemtl" => LineType::UseMaterial(
                String::from(segments[1])),

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
pub enum LineType {
    Comment(String),
    Object(String),
    Vertex(f32, f32, f32),
    VertexNormal(f32, f32, f32),
    VertexTexture(f32, f32),
    Face(FaceVertex, FaceVertex, FaceVertex),
    UseMaterial(String),

    Unknown,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FaceVertex (pub usize, pub Option<usize>, pub Option<usize>);