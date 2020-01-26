
// Image resolution, in pixels.
// 3072 x 1920; 16:10
pub const RESOLUTION_Y: usize = 1920; // 1920
pub const RESOLUTION_X: usize = 1 + (RESOLUTION_Y as f32 * 1.6) as usize;

// Number of sample rays to cast for diffuse/specular illumination
pub const SAMPLE_COUNT: usize = 16;

// Max number of indirect bounces to make
pub const BOUNCES: u8 = 2;

// How many chunks the image should be split up into, for multithreading
pub const THREADS: usize = 256;

pub const PREVIEW_MODE: bool = false;


// derived for utility
pub const TOTAL_BUFFER_SIZE: usize = RESOLUTION_X * RESOLUTION_Y;
pub const PIXELS_PER_THREAD: usize = TOTAL_BUFFER_SIZE / THREADS;