
// Image resolution, in pixels.
pub const RESOLUTION: usize = 1024;

// Number of sample rays to cast for diffuse/specular illumination
pub const SAMPLE_COUNT: usize = 32;

// Max number of indirect bounces to make
pub const MAX_DEPTH: u8 = 3;

// How many chunks the image should be split up into, for multithreading
pub const CELLS: usize = 64; // must be the square of an integer