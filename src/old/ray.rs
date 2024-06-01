use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        let Self { origin, direction } = self;
        &(origin + direction) * t
    }

    pub fn color(&self) -> String {
        let unit_direction = self.direction.unit_vector();
        let a = (unit_direction.y + 1.0) / 2.0;
        let v = &Vec3::one() * (1.0 - a) + &Vec3::new(0.5, 0.7, 1.0) * a;
        v.color()
    }
}
