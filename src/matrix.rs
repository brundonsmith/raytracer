
use crate::vec3::Vec3;

const SIZE: usize = 4;
const BUFFER_SIZE: usize = SIZE * SIZE;
pub const IDENTITY: Matrix = Matrix([
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    0.0, 0.0, 0.0, 1.0
]);



#[derive(Copy, Clone, PartialEq)]
pub struct Matrix( [f32; BUFFER_SIZE] );

impl Matrix {

    pub fn new() -> Self {
        IDENTITY.clone()
    }

    pub fn translation(offset: &Vec3) -> Self {
        Self([
            1.0, 0.0, 0.0, offset.x,
            0.0, 1.0, 0.0, offset.y,
            0.0, 0.0, 1.0, offset.z,
            0.0, 0.0, 0.0, 1.0
        ])
    }

    pub fn rotation_x(theta: f32) -> Self {
        Self([
            1.0, 0.0,         0.0,                0.0,
            0.0, theta.cos(), -1.0 * theta.sin(), 0.0,
            0.0, theta.sin(), theta.cos(),        0.0,
            0.0, 0.0,         0.0,                1.0
        ])
    }

    pub fn rotation_y(theta: f32) -> Self {
        Self([
            theta.cos(),        0.0, theta.sin(), 0.0,
            0.0,                1.0, 0.0,         0.0,
            -1.0 * theta.sin(), 0.0, theta.cos(), 0.0,
            0.0,                0.0, 0.0,         1.0
        ])
    }

    pub fn rotation_z(theta: f32) -> Self {
        Self([
            theta.cos(), -1.0 * theta.sin(), 0.0, 0.0,
            theta.sin(), theta.cos(),        0.0, 0.0,
            0.0,         0.0,                1.0, 0.0,
            0.0,         0.0,                0.0, 1.0
        ])
    }
    
    pub fn scale(scale: &Vec3) -> Self {
        Self([
            scale.x, 0.0,     0.0,     0.0,
            0.0,     scale.y, 0.0,     0.0,
            0.0,     0.0,     scale.z, 0.0,
            0.0,     0.0,     0.0,     1.0
        ])
    }

    pub fn from_to_rotation(from: &Vec3, to: &Vec3) -> Self {
        let mut w = from.cross(&to);
        w.normalize();

        let k = Self([
            0.0,       -1.0 * w.z,  w.y,        0.0,
            w.z,        0.0,        -1.0 * w.x, 0.0,
            -1.0 * w.y, w.x,        0.0,        0.0,
            0.0,        0.0,        0.0,        1.0
        ]);

        let theta = from.angle(&to);


        return &(&IDENTITY + &(&k * theta.sin())) + &(&(&k * &k) * (1.0 - theta.cos()));
    }

    // core ops
    fn index_for(&self, row: usize, col: usize) -> usize {
        row * SIZE + col
    }

    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.0[self.index_for(row, col)]
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut f32 {
        &mut self.0[self.index_for(row, col)]
    }

    pub fn set(&mut self, row: usize, col: usize, val: f32) {
        self.0[self.index_for(row, col)] = val;
    }

    // other ops
    pub fn invert(&mut self) {
        unimplemented!();
    }

    pub fn inverse(&self) -> Matrix {
        let mut result = self.clone();
        result.invert();
        return result;
    }

    pub fn transpose(&mut self) {
        for r in 0..SIZE {
            for c in 0..r {
                let temp = self.get(r, c);
                self.set(r, c, self.get(c, r));
                self.set(c, r, temp);
            }
        }
    }

    pub fn transposition(&self) -> Matrix {
        let mut result = self.clone();
        result.transpose();
        return result;
    }
}

// standard traits
impl std::ops::Add for &Matrix {
    type Output = Matrix;

    fn add(self, other: Self) -> Self::Output {
        let mut result = Self::Output::new();

        for i in 0..BUFFER_SIZE {
            result.0[i] = self.0[i] + other.0[i];
        }

        return result;
    }
}
impl std::ops::Sub for &Matrix {
    type Output = Matrix;

    fn sub(self, other: Self) -> Self::Output {
        let mut result = Self::Output::new();

        for i in 0..BUFFER_SIZE {
            result.0[i] = self.0[i] - other.0[i];
        }

        return result;
    }
}
impl std::ops::Mul for &Matrix {
    type Output = Matrix;

    fn mul(self, other: Self) -> Self::Output {
        let mut result = Self::Output::new();

        for r in 0..SIZE {
            for c in 0..SIZE {
                let mut val = 0.0;

                for n in 0..SIZE {
                    val += self.get(r, n) * other.get(n, c);
                }

                result.set(r, c, val);
            }
        }

        return result;
    }
}
impl std::ops::Mul for Matrix {
    type Output = Matrix;

    fn mul(self, other: Self) -> Self::Output {
        &self * &other
    }
}

impl std::ops::Mul<f32> for &Matrix {
    type Output = Matrix;
    
    fn mul(self, scale: f32) -> Self::Output {
        let mut result = Self::Output::new();

        for i in 0..BUFFER_SIZE {
            result.0[i] = self.0[i] * scale;
        }

        return result;
    }
}
impl std::fmt::Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Matrix([\n\t{}, {}, {}, {},\n\t{}, {}, {}, {},\n\t{}, {}, {}, {},\n\t{}, {}, {}, {},])", 
            self.0[0],  self.0[1],  self.0[2],   self.0[3],
            self.0[4],  self.0[5],  self.0[6],   self.0[7],
            self.0[8],  self.0[9],  self.0[10],  self.0[11],
            self.0[12], self.0[13], self.0[14],  self.0[15],)
    }
}

#[test]
fn test_mul() {
    let mat_1 = Matrix([
        5.0, 2.0, 8.0, 3.0,
        7.0, 3.0, 10.0, 3.0,
        9.0, 3.0, 2.0, 4.0,
        10.0, 8.0, 3.0, 8.0
    ]);

    let mat_2 = Matrix([
        3.0, 12.0, 9.0, 3.0,
        10.0, 1.0, 10.0, 12.0,
        12.0, 4.0, 12.0, 4.0,
        18.0, 9.0, 2.0, 10.0
    ]);

    assert_eq!(&mat_1 * &mat_2, Matrix([
        210.0, 93.0, 171.0, 105.0,
        267.0, 149.0, 146.0, 169.0,
        236.0, 104.0, 172.0, 128.0,
        271.0, 149.0, 268.0, 169.0
    ]));
}

#[test]
fn test_from_to() {
    // NOTE: This will fail because of floating point comparison, but it should be 
    // clear from the output whether or not it did what it was supposed to
    let from = Vec3 { x: 0.2, y: 0.1, z: 0.3 }.normalized();
    let to = Vec3 { x: 0.5, y: -0.1, z: 0.4 }.normalized();

    let rotation = Matrix::from_to_rotation(&from, &to);

    assert_eq!(&to, &from.transformed(&rotation));
}