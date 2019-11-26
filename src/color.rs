
use crate::utils::{avg,clamp};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color(pub f32, pub f32, pub f32);

impl Color {

    pub fn to_u8(&self) -> [u8;3] {
        [ 
            clamp(self.0 * 255.0, 0.0, 255.0) as u8, 
            clamp(self.1 * 255.0, 0.0, 255.0) as u8, 
            clamp(self.2 * 255.0, 0.0, 255.0) as u8
        ]
    }
}

impl std::ops::Add<Color> for Color {
    type Output = Color;
    
    fn add(self, other: Color) -> Self::Output {
        Color (
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
        )
    }
}

impl std::ops::Mul<Color> for Color {
    type Output = Color;
    
    fn mul(self, other: Color) -> Self::Output {
        Color (
            avg(self.0, other.0), 
            avg(self.1, other.1), 
            avg(self.2, other.2), 
        )
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Color;
    
    fn mul(self, scale: f32) -> Self::Output {
        Color (
            self.0 * scale, 
            self.1 * scale, 
            self.2 * scale, 
        )
    }
}

impl std::ops::Div<usize> for Color {
    type Output = Color;
    
    fn div(self, divisor: usize) -> Self::Output {
        Color (
            self.0 / divisor as f32, 
            self.1 / divisor as f32, 
            self.2 / divisor as f32, 
        )
    }
}
