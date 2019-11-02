

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 { 
    pub x: f32, 
    pub y: f32, 
    pub z: f32
}

impl Vec3 {

    pub fn new() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn from_angles(alpha: f32, beta: f32) -> Self {
        Self {
            x: alpha.cos() * beta.cos(),
            y: beta.sin(),
            z: alpha.sin() * beta.cos(),
        }
    }

    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&mut self) {
        let len = self.len();
        self.scale(1.0 / len);
    }

    pub fn normalized(&self) -> Vec3 {
        let mut result = self.clone();
        result.normalize();
        return result;
    }

    pub fn scale(&mut self, scale: f32) {
        self.x *= scale;
        self.y *= scale;
        self.z *= scale;
    } 

    pub fn dot(self, other: &Self) -> f32 {
        self.x * other.x +
        self.y * other.y + 
        self.z * other.z
    }

    pub fn angle(&self, other: &Vec3) -> f32 {
        ((self * other) / (self.len() * other.len())).acos()
    }
}


// standard traits
impl std::ops::Add for &Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Self::Output {
         Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}
impl std::ops::Add<f32> for &Vec3 {
    type Output = Vec3;
    
    fn add(self, diff: f32) -> Self::Output {
        Self::Output {
            x: self.x + diff,
            y: self.y + diff,
            z: self.z + diff
        }
    }
}
impl std::ops::Sub for &Vec3 {
    type Output = Vec3;
    
    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}
impl std::ops::Sub<f32> for &Vec3 {
    type Output = Vec3;
    
    fn sub(self, diff: f32) -> Self::Output {
        Self::Output {
            x: self.x - diff,
            y: self.y - diff,
            z: self.z - diff
        }
    }
}

impl std::ops::Mul for &Vec3 {
    type Output = f32;

    // (dot-product)
    fn mul(self, other: Self) -> Self::Output {
        self.dot(other)
    }
}
impl std::ops::Mul<f32> for &Vec3 {
    type Output = Vec3;
    
    fn mul(self, scale: f32) -> Self::Output {
        Self::Output {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale
        }
    }
}



// tests
#[test]
fn test_ops() {
    let vec1 = Vec3 { x: 1.0, y: 3.0, z: -5.0 };
    let vec2 = Vec3 { x: 4.0, y: -2.0, z: -1.0 };

    assert_eq!(&vec1 + &vec2, Vec3{ x: 5.0, y: 1.0, z: -6.0 });
    assert_eq!(&vec1 * &vec2, 3.0);
}