
// Image resolution, in pixels.
pub const RESOLUTION: usize = 128;

// Number of sample rays to cast for diffuse/specular illumination
pub const SAMPLE_COUNT: usize = 16;

// Max number of indirect bounces to make
pub const BOUNCES: u8 = 2;

// How many chunks the image should be split up into, for multithreading
pub const THREADS: usize = 256;

pub const PREVIEW_MODE: bool = false;


// derived for utility
pub const TOTAL_BUFFER_SIZE: usize = RESOLUTION * RESOLUTION;
pub const PIXELS_PER_THREAD: usize = TOTAL_BUFFER_SIZE / THREADS;