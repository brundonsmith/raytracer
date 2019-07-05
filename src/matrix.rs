

const SIZE: usize = 4;
const BUFFER_SIZE: usize = SIZE * SIZE;
const IDENTITY: Matrix = Matrix {
    contents: [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    ]
};



#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix {
    contents: [f32; BUFFER_SIZE]
}

impl Matrix {

    pub fn new() -> Self {
        Matrix {
            contents: [0.0; BUFFER_SIZE]
        }
    }

    pub fn identity() -> Self {
        IDENTITY.clone()
    }

    pub fn index_for(&self, row: usize, col: usize) -> usize {
        row + col * SIZE
    }

    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.contents[self.index_for(row, col)]
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut f32 {
        &mut self.contents[self.index_for(row, col)]
    }

    pub fn set(&mut self, row: usize, col: usize, val: f32) {
        self.contents[self.index_for(row, col)] = val;
    }

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
            result.contents[i] = self.contents[i] + other.contents[i];
        }

        return result;
    }
}
impl std::ops::Sub for &Matrix {
    type Output = Matrix;

    fn sub(self, other: Self) -> Self::Output {
        let mut result = Self::Output::new();

        for i in 0..BUFFER_SIZE {
            result.contents[i] = self.contents[i] - other.contents[i];
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
impl std::ops::Mul<f32> for &Matrix {
    type Output = Matrix;
    
    fn mul(self, scale: f32) -> Self::Output {
        let mut result = Self::Output::new();

        for i in 0..BUFFER_SIZE {
            result.contents[i] = self.contents[i] * scale;
        }

        return result;
    }
}

#[test]
fn test_mul() {
    let mat_1 = Matrix {
        contents: [
            5.0, 2.0, 8.0, 3.0,
            7.0, 3.0, 10.0, 3.0,
            9.0, 3.0, 2.0, 4.0,
            10.0, 8.0, 3.0, 8.0
        ]
    };

    let mat_2 = Matrix {
        contents: [
            3.0, 12.0, 9.0, 3.0,
            10.0, 1.0, 10.0, 12.0,
            12.0, 4.0, 12.0, 4.0,
            18.0, 9.0, 2.0, 10.0
        ]
    };

    assert_eq!(&mat_1 * &mat_2, Matrix {
        contents: [
            210.0, 93.0, 171.0, 105.0,
            267.0, 149.0, 146.0, 169.0,
            236.0, 104.0, 172.0, 128.0,
            271.0, 149.0, 268.0, 169.0
        ]
    });
}