
pub fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    if val < min { min }
    else if val > max { max } 
    else { val }
}

pub fn avg(a: f32, b: f32) -> f32 {
    (a + b) / 2.0
}
