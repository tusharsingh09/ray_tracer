use crate::util::*;
use std::ops;
#[derive(Debug, Copy, Clone)]
pub struct Point(f64, f64, f64);

impl ops::Add<Vector> for Point {
    type Output = Point;
    fn add(self, rhs: Vector) -> Self {
        Point(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2
        )
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Vector(pub f64, pub f64, pub f64);

impl Vector {
    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn direction(&self) -> Self{
        *self / self.length()
    }

    pub fn print(&self) -> () {
        println!("{} {} {}", self.0, self.1, self.2);
    }

    pub fn random() -> Vector {
        Vector(rand(), rand(), rand())
    }
     
    pub fn rand_range(min: f64, max: f64) -> Vector {
        Vector(rand_range(min, max), rand_range(min, max), rand_range(min, max))
    }

    pub fn random_unit() -> Vector {
        loop {
            let p = Vector::rand_range(-1.0, 1.0);
            let len = p.length_squared();
            if len <= 1.0 && len >= 1e-160 { return p / len.sqrt(); }
        }
    }

    pub fn rand_on_hemi(n: &Vector) -> Vector {
        let unit_v = Self::random_unit();
        if dot(&unit_v, &n) > 0.0 { return unit_v; }
        else { return unit_v * -1.0 };
    }
}

impl ops::Add<Self> for Vector {
    type Output = Vector;
    fn add(self, _rhs: Vector) -> Self {
        Vector(self.0 + _rhs.0, self.1 + _rhs.1, self.2 + _rhs.2)
    }
}

impl ops::Sub<Self> for Vector {
    type Output = Self;
    fn sub(self, _rhs: Vector) -> Self {
        Vector(self.0 - _rhs.0, self.1 - _rhs.1, self.2 - _rhs.2)
    }
}

impl ops::AddAssign<Self> for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;    
    }
}

impl ops::SubAssign<Self> for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl ops::Mul<f64> for Vector {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self{
        Vector(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::Div<f64> for Vector {
    type Output = Vector;
    fn div(self, rhs: f64) -> Self {
        Vector(
            self.0 / rhs,
            self.1 / rhs,
            self.2 / rhs
        )
    }
}

pub fn cross(a: &Vector, b: &Vector) -> Vector {
    Vector(
        a.1 * b.2 - a.2 * b.1,
        a.2 * b.0 - a.0 * b.2,
        a.0 * b.1 - a.1 * b.0
    )
}

pub fn dot(a: &Vector, b: &Vector) -> f64 {
        a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

// utilities
pub fn unit_vector(v: &Vector) -> Vector {
    v.direction() / v.length()
}