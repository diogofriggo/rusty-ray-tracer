use std::fmt::Display;

use crate::{ray::Ray, vec3::Vec3};

#[derive(Clone, Debug)]
pub struct Triangle {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Self { a, b, c }
    }

    pub fn normal(&self) -> Vec3 {
        (&self.b - &self.a).cross(&(&self.c - &self.a))
    }

    pub fn collides_with(&self, ray: &Ray) -> Option<f64> {
        None
    }
}

impl Display for Triangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.a, self.b, self.c)
    }
}
