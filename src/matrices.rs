use std::ops;

use crate::vectors::Vector3;

#[derive(Clone)]
pub struct Matrix3 {
    buffer: Vec<f64>
}

impl Matrix3 {
    pub fn new(buffer: Vec<f64>) -> Self {
        if buffer.len() != 9 {
            panic!("Wrong size for the matrix buffer");
        }
        Matrix3 { buffer }
    }

    pub fn get(&self, i: usize, j: usize) -> Option<&f64> {
        let index = i * 4 + j;
        self.buffer.get(index)
    }

    pub fn x_rotation(angle: f64) -> Self {
        Matrix3::new(vec![
            1.0, 0.0, 0.0,
            0.0, f64::cos(angle), -f64::sin(angle),
            0.0, f64::sin(angle), f64::cos(angle)
        ])
    }

    pub fn y_rotation(angle: f64) -> Self {
        Matrix3::new(vec![
            f64::cos(angle), 0.0, f64::sin(angle),
            0.0, 1.0, 0.0,
            -f64::sin(angle), 0.0, f64::cos(angle)
        ])
    }

    pub fn identity() -> Self {
        Matrix3::new(vec![
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0
        ])
    }
}

impl ops::Mul<&Vector3> for &Matrix3 {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Vector3 {
        let x = self.buffer[0] * rhs.x + self.buffer[1] * rhs.y + self.buffer[2] * rhs.z;
        let y = self.buffer[3] * rhs.x + self.buffer[4] * rhs.y + self.buffer[5] * rhs.z;
        let z = self.buffer[6] * rhs.x + self.buffer[7] * rhs.y + self.buffer[8] * rhs.z;
        Vector3::new(x, y, z)
    }
}


impl ops::Mul<&Matrix3> for &Matrix3 {
    type Output = Matrix3;

    fn mul(self, rhs: &Matrix3) -> Matrix3 {
        let mut buffer = vec![0.0; 9];

        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    buffer[i * 3 + j] += self.buffer[i * 3 + k] * rhs.buffer[k * 3 + j];
                }
            }
        }

        Matrix3::new(buffer)
    }
}
