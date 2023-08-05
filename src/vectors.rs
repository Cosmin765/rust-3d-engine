use std::ops::MulAssign;
use std::ops::Mul;
use std::ops::Add;
use std::ops::Sub;


#[derive(Debug, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 {x, y, z}
    }
}

impl Mul<f64> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3 {x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
    }
}


impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Add<&Vector3> for &Vector3 {
    type Output = Vector3;

    fn add(self, rhs: &Vector3) -> Self::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<&Vector3> for &Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &Vector3) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}


#[derive(Debug, Clone)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 {x, y}
    }
}

impl Add<&Vector2> for &Vector2 {
    type Output = Vector2;

    fn add(self, rhs: &Vector2) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}
