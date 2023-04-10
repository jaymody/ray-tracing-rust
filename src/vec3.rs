use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn x(self) -> f64 {
        self.x
    }
    pub fn y(self) -> f64 {
        self.y
    }
    pub fn z(self) -> f64 {
        self.z
    }

    pub fn squared_length(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(self) -> f64 {
        self.squared_length().sqrt()
    }
    pub fn unit(self) -> Vec3 {
        self / self.length()
    }

    pub fn dot(self, v: Vec3) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
    pub fn cross(self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
        )
    }
}

// display
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

// negate
impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

// add
impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}
impl Add<f64> for Vec3 {
    type Output = Vec3;
    fn add(self, f: f64) -> Vec3 {
        Vec3::new(self.x + f, self.y + f, self.z + f)
    }
}

// subtract
impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}
impl Sub<f64> for Vec3 {
    type Output = Vec3;
    fn sub(self, f: f64) -> Vec3 {
        Vec3::new(self.x - f, self.y - f, self.z - f)
    }
}

// multiply
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }
}
impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, f: f64) -> Vec3 {
        Vec3::new(self.x * f, self.y * f, self.z * f)
    }
}

// divide
impl Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x / v.x, self.y / v.y, self.z / v.z)
    }
}
impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, f: f64) -> Vec3 {
        Vec3::new(self.x / f, self.y / f, self.z / f)
    }
}
