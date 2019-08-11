

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8);

impl std::ops::Mul<&Color> for &Color {
    type Output = Color;
    
    fn mul(self, other: &Color) -> Self::Output {
        Color (
            (self.0 as f32 * other.0 as f32).round() as u8, 
            (self.1 as f32 * other.1 as f32).round() as u8, 
            (self.2 as f32 * other.2 as f32).round() as u8, 
        )
    }
}

impl std::ops::Mul<f32> for &Color {
    type Output = Color;
    
    fn mul(self, scale: f32) -> Self::Output {
        Color (
            (self.0 as f32 * scale).round() as u8, 
            (self.1 as f32 * scale).round() as u8, 
            (self.2 as f32 * scale).round() as u8, 
        )
    }
}