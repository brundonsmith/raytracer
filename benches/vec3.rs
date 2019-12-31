
use criterion::{black_box, Criterion};

use raytracer::vec3::Vec3;

const VEC_BASIC: Vec3 = Vec3 { x: 1.0, y: 1.0, z: 1.0 };
const VEC_ASKEW: Vec3 = Vec3 { x: 0.5, y: 0.8, z: 1.5 };

pub fn angles(c: &mut Criterion) {
    c.bench_function("Vec3 { x: 1.0, y: 1.0, z: 1.0 }.angles()", |b| 
        b.iter(|| black_box(black_box(VEC_BASIC).angles())));
}

pub fn normalized(c: &mut Criterion) {
    c.bench_function("Vec3 { x: 1.0, y: 1.0, z: 1.0 }.normalized()", |b| 
        b.iter(|| black_box(black_box(VEC_BASIC).normalized())));
}

pub fn angle(c: &mut Criterion) {
    c.bench_function("Vec3 { x: 1.0, y: 1.0, z: 1.0 }.angle()", |b| 
        b.iter(|| black_box(black_box(VEC_BASIC).angle(black_box(&VEC_ASKEW)))));
}

pub fn projected_on(c: &mut Criterion) {
    c.bench_function("Vec3 { x: 1.0, y: 1.0, z: 1.0 }.projected_on(Vec3 { x: 0.5, y: 0.8, z: 1.5 })", |b| 
        b.iter(|| black_box(black_box(VEC_BASIC).projected_on(black_box(&VEC_ASKEW)))));
}

pub fn rotated_around(c: &mut Criterion) {
    c.bench_function("Vec3 { x: 1.0, y: 1.0, z: 1.0 }.rotated_around(Vec3 { x: 0.5, y: 0.8, z: 1.5 }, 25.3)", |b| 
        b.iter(|| black_box(black_box(VEC_BASIC).rotated_around(black_box(&VEC_ASKEW), black_box(25.3)))));
}