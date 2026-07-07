use crate::vector::*;

pub struct Ray {
    dir: Vector,
    origin: Vector
}

impl Ray {
    pub fn new(d: Vector, o: Vector) -> Self {
        Ray {
            dir: d,
            origin: o
        }
    }

    pub fn dir(&self) -> &Vector {
        &self.dir
    }

    pub fn origin(&self) -> &Vector {
        &self.origin
    }

    pub fn at(&self, t: f64) -> Vector {
        self.origin + self.dir * t
    }
}