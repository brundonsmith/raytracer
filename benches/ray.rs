
use criterion::{black_box, Criterion};

extern crate rand;
use rand::prelude::*;
use rand::{thread_rng};
use rand::rngs::SmallRng;

use raytracer::ray::Ray;
use raytracer::vec3::Vec3;

const ORIGIN: Vec3 = Vec3 { x: 1.0, y: 1.0, z: 1.0 };

pub fn random_direction(c: &mut Criterion) {
    let mut meta_rng = thread_rng();
    let mut rng = SmallRng::from_rng(&mut meta_rng).unwrap();

    c.bench_function("random_direction(Vec3 { x: 1.0, y: 1.0, z: 1.0 }, [rng])", |b| 
        b.iter(|| Ray::random_direction(black_box(ORIGIN), &mut rng)));
}