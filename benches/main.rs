
/*
 * NOTE: Always run with the following settings for consistent results:
 * 
 * 
    // Image resolution, in pixels.
    pub const RESOLUTION: usize = 256;

    // Number of sample rays to cast for diffuse/specular illumination
    pub const SAMPLE_COUNT: usize = 16;

    // Max number of indirect bounces to make
    pub const MAX_DEPTH: u8 = 3;

    // How many chunks the image should be split up into, for multithreading
    pub const CELLS: usize = 64; // must be the square of an integer

    pub const PREVIEW_MODE: bool = false;
 */

use criterion::{criterion_group, criterion_main};

mod cast;
criterion_group!(benches_cast, 
    cast::cast_ray_1, 
    cast::cast_ray_2, 
    cast::cast_ray_3);

mod ray;
criterion_group!(benches_ray, 
    ray::random_direction);

mod vec3;
criterion_group!(benches_vec3, 
    vec3::angles, 
    vec3::normalized, 
    vec3::angle, 
    vec3::projected_on, 
    vec3::rotated_around);

criterion_main!(benches_cast, benches_ray, benches_vec3);
