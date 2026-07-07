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

#[derive(Debug, Copy, Clone)]
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
        Vector(self.0 - _rhs.0, self.1 - _rhs.1, self.2 - _rhs.1)
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