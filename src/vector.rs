#[derive(Clone, Debug, PartialEq)]
pub struct Vector {
    pub coords: Vec<f64>,
}

impl Vector {
    pub fn new(coords: Vec<f64>) -> Self {
        Self { coords }
    }

    pub fn new_xyz(x: f64, y: f64, z: f64) -> Self {
        Self {
            coords: vec![x, y, z],
        }
    }

    pub fn length(&self) -> f64 {
        (self.length_squared()).sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        self.coords
            .iter()
            .zip(other.coords.iter())
            .map(|(a, b)| a * b)
            .sum()
    }

    pub fn cross(&self, other: &Vector) -> Vector {
        let s = &self.coords;
        let o = &other.coords;
        let x = s[1] * o[2] - s[2] * o[1];
        let y = s[2] * o[0] - s[0] * o[2];
        let z = s[0] * o[1] - s[1] * o[0];

        Self::new(vec![x, y, z])
    }

    pub fn unit(&self) -> Vector {
        self.div_float(self.length())
    }

    pub fn mul_float(&self, other: f64) -> Self {
        let coords = self.coords.iter().map(|a| a * other).collect();
        Self::new(coords)
    }

    pub fn div_float(&self, other: f64) -> Self {
        let coords = self.coords.iter().map(|a| a / other).collect();
        Self::new(coords)
    }

    pub fn add_vec(&self, other: &Vector) -> Self {
        let coords = self
            .coords
            .iter()
            .zip(other.coords.iter())
            .map(|(a, b)| a + b)
            .collect();
        Self::new(coords)
    }

    pub fn sub_vec(&self, other: &Vector) -> Self {
        let coords = self
            .coords
            .iter()
            .zip(other.coords.iter())
            .map(|(a, b)| a - b)
            .collect();
        Self::new(coords)
    }

    pub fn mul_vec(&self, other: &Vector) -> Self {
        let coords = self
            .coords
            .iter()
            .zip(other.coords.iter())
            .map(|(a, b)| a * b)
            .collect();
        Self::new(coords)
    }
}

impl Default for Vector {
    fn default() -> Self {
        Self {
            coords: vec![0.0, 0.0, 0.0],
        }
    }
}
