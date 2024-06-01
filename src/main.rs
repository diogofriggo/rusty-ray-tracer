use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rayon::prelude::*;

const TOLERANCE: f64 = 1E-10;

fn main() {
    let obj_path = "/home/diogo/projects/ray-tracing/cow.obj";
    let png_path = "/home/diogo/projects/ray-tracing/cow.png";

    let light = Light {
        origin: Vector::new_xyz(-5.0, 5.0, -4.0),
        intensity: 4.0,
        color: Vector::new_xyz(1.0, 1.0, 1.0),
    };

    let sphere_material = Material {
        diffuse: Vector::new_xyz(1.0, 1.0, 1.0),
        specular: Vector::default(),
        brightness: 0.0,
    };
    let center = Vector::new_xyz(0.0, -5006.0, -30.0);
    let sphere = Sphere::new(center, 5000.0, &sphere_material);
    let mut shapes = vec![Shape::Sphere(sphere)];

    let triangle_material = Material {
        diffuse: Vector::new_xyz(0.2, 0.2, 0.6),
        specular: Vector::new_xyz(0.5, 0.6, 0.7),
        brightness: 40.0,
    };
    let mut triangles = read_obj(obj_path, &triangle_material);

    shapes.append(&mut triangles);

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 720.0;
    let image_height = image_width / aspect_ratio;
    let offset = 0.5;
    let origin = Vector::default();
    let of = origin.add_vec(&Vector::new_xyz(0.0, 0.0, 1.0)).coords;
    let n = image_width as usize;
    let m = image_height as usize;
    let tuples = (0..m)
        .cartesian_product(0..n)
        .collect::<Vec<(usize, usize)>>();

    let pixels = tuples
        .into_par_iter()
        .progress()
        // .into_iter()
        .map(|(j, i): (usize, usize)| {
            let ifloat = i as f64 + offset;
            let jfloat = j as f64 + offset;

            let u = (2.0 * ifloat / image_width - 1.0) * aspect_ratio;
            let v = 1.0 - 2.0 * jfloat / image_height;

            let p = Vector::new_xyz(u + of[0], v + of[1], of[2]);
            let direction = p.sub_vec(&origin).unit();
            // if i == 437 && j == 28 {
            //     println!("{} {} {:?} {:?}", i, j, p, origin);
            //     // 437 28 Vector { coords: [0.38271604938271586, 0.8592592592592593, 1.0] } Vector { coords: [0.0, 0.0, 0.0] }
            //
            //     panic!();
            // }
            // 437 28 Vector { coords: [0.27876886262601847, 0.6258810593151902, 0.7283960604099197] }
            // 437 28 (0.638275, -0.491003, 0.592893)

            let ray = Ray::new(Vector::default(), direction.clone());

            ray.pierce(i, j, &shapes, &light)
        })
        .collect::<Vec<_>>();

    write_png(png_path, n as u32, m as u32, &pixels);
}

#[derive(PartialEq, Clone, Debug)]
struct Ray {
    origin: Vector,
    direction: Vector,
}

impl Ray {
    fn new(origin: Vector, direction: Vector) -> Self {
        Self { origin, direction }
    }

    fn pierce(&self, i: usize, j: usize, shapes: &Vec<Shape>, light: &Light) -> Vector {
        let mut hit_distance = f64::INFINITY;
        let mut hit_shape = None;

        for shape in shapes {
            if let Some(distance) = shape.hit(i, j, self) {
                if distance <= hit_distance {
                    hit_distance = distance;
                    hit_shape = Some(shape);
                }
            }
        }

        let mut pixel = Vector::default();
        if let Some(hit_shape) = hit_shape {
            let hit = self.origin.add_vec(&self.direction.mul_float(hit_distance));

            let direction = light.origin.sub_vec(&hit).unit();
            let ray = Ray::new(hit.clone(), direction);

            let light_not_absorbed = shapes
                .iter()
                .filter(|s| s != &hit_shape)
                .all(|s| s.hit(i, j, &ray).is_none());

            if light_not_absorbed {
                let ray = Ray::new(self.origin.clone(), hit.clone());
                let reflection = hit_shape.reflect(&ray, light);
                pixel = pixel.add_vec(&reflection);
            }
        }

        pixel
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Light {
    origin: Vector,
    intensity: f64,
    color: Vector,
}

#[derive(PartialEq, Debug)]
struct Material {
    diffuse: Vector,
    specular: Vector,
    brightness: f64,
}

#[derive(PartialEq, Debug)]
struct Sphere<'a> {
    center: Vector,
    r: f64,
    material: &'a Material,
}

impl<'a> Sphere<'a> {
    fn new(center: Vector, r: f64, material: &'a Material) -> Sphere<'a> {
        Self {
            center,
            r,
            material,
        }
    }
}

#[derive(PartialEq, Debug)]
struct Triangle<'a> {
    id: usize,
    points: Vec<Vector>,
    normal: Vector,
    material: &'a Material,
}

impl<'a> Triangle<'a> {
    fn new(id: usize, points: Vec<Vector>, material: &'a Material) -> Triangle<'a> {
        let ab = points[1].sub_vec(&points[0]);
        let ac = points[2].sub_vec(&points[0]);
        let normal = ab.cross(&ac).unit();
        Self {
            id,
            points,
            normal,
            material,
        }
    }
}

#[derive(PartialEq, Debug)]
enum Shape<'a> {
    Triangle(Triangle<'a>),
    Sphere(Sphere<'a>),
}

impl Shape<'_> {
    fn hit(&self, i: usize, j: usize, ray: &Ray) -> Option<f64> {
        match self {
            Shape::Triangle(triangle) => hit_triangle(i, j, triangle, ray),
            Shape::Sphere(sphere) => hit_sphere(i, j, sphere, ray),
        }
    }

    fn reflect(&self, ray: &Ray, light: &Light) -> Vector {
        match self {
            Shape::Triangle(triangle) => reflect_triangle(triangle, ray, light),
            Shape::Sphere(sphere) => reflect_sphere(sphere, ray, light),
        }
    }
}

fn hit_triangle(i: usize, j: usize, triangle: &Triangle, ray: &Ray) -> Option<f64> {
    let points = &triangle.points;
    let ab = points[1].sub_vec(&points[0]);
    let ac = points[2].sub_vec(&points[0]);

    let h = ray.direction.cross(&ac);

    let k = ab.dot(&h);

    if k.abs() < TOLERANCE {
        return None;
    }

    let f = 1.0 / k;

    let s = ray.origin.sub_vec(&points[0]);
    let u = f * s.dot(&h);

    if !(0.0..=1.0).contains(&u) {
        return None;
    }

    let q = s.cross(&ab);
    let v = f * ray.direction.dot(&q);
    if v < 0.0 || (u + v) > 1.0 {
        return None;
    }

    let distance = f * ac.dot(&q);

    if distance > TOLERANCE {
        Some(distance)
    } else {
        None
    }
}

fn hit_sphere(i: usize, j: usize, sphere: &Sphere, ray: &Ray) -> Option<f64> {
    let oc = ray.origin.sub_vec(&sphere.center);
    let a = ray.direction.length_squared();
    let b = 2.0 * oc.dot(&ray.direction);
    let c = oc.length_squared() - sphere.r.powf(2.0);
    let discriminant = b.powf(2.0) - 4.0 * a * c;
    if discriminant < 0.0 {
        None
    } else {
        let left = -b;
        let right = discriminant.sqrt();
        let p_root = (left + right) / 2.0;
        let n_root = (left - right) / 2.0;
        if p_root < 0.0 && n_root < 0.0 {
            None
        } else {
            Some(p_root.min(n_root))
        }
    }
}

fn reflect_triangle(triangle: &Triangle, ray: &Ray, light: &Light) -> Vector {
    reflect(ray, &triangle.normal, light, triangle.material)
}

fn reflect_sphere(sphere: &Sphere, ray: &Ray, light: &Light) -> Vector {
    let normal = ray.direction.sub_vec(&sphere.center).unit();
    reflect(ray, &normal, light, sphere.material)
}

fn reflect(ray: &Ray, normal: &Vector, light: &Light, material: &Material) -> Vector {
    let s = &light.origin.sub_vec(&ray.direction);
    let sl = s.unit();
    let l = light
        .color
        .mul_float(light.intensity)
        .div_float(sl.length());

    let n_dot_l = normal.dot(&sl);
    let n_dot_l_floor = n_dot_l.max(0.0);
    let brdf = l
        .mul_vec(&material.diffuse.div_float(PI))
        .mul_float(n_dot_l_floor);
    let r = normal.mul_float(n_dot_l).sub_vec(&sl).mul_float(2.0).unit();

    let v = ray.origin.sub_vec(&ray.direction).unit();
    let r_dot_v = r.dot(&v).max(0.0);

    let m = r_dot_v.powf(material.brightness) * n_dot_l_floor;
    brdf.add_vec(&material.specular.mul_float(m))
}

/// obj
use std::{
    f64::consts::PI,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

pub fn read_obj<'a>(path: &'_ str, material: &'a Material) -> Vec<Shape<'a>> {
    let mut positions = vec![];

    let mut min = [f64::MAX, f64::MAX, f64::MAX];
    let mut max = [f64::MIN, f64::MIN, f64::MIN];
    let file = || {
        let file = File::open(path).unwrap();
        BufReader::new(file)
    };

    for line in file().lines() {
        let line = line.unwrap();

        if !line.starts_with('v') {
            continue;
        }

        let position = parse::<f64>(line);
        debug_assert!(position.len() == 3);

        for i in 0..3 {
            if position[i] < min[i] {
                min[i] = position[i];
            }
            if position[i] > max[i] {
                max[i] = position[i];
            }
        }

        positions.push(position);
    }

    debug_assert!(positions.len() > 1);

    let c = min
        .iter()
        .zip(max.iter())
        .map(|(min, max)| (max + min) / 2.0)
        .collect::<Vec<_>>();

    let d = min
        .iter()
        .zip(max.iter())
        .map(|(min, max)| max - min)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    debug_assert!(d > 0.0);

    let positions = positions
        .into_iter()
        .map(|p| (0..3).map(|i| (p[i] - c[i]) / d).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let m = get_transform_matrix();
    let mut shapes = vec![];
    let mut id = 0;
    for line in file().lines() {
        let line = line.unwrap();

        if !line.starts_with('f') {
            continue;
        }

        let indices = parse::<usize>(line);
        debug_assert!(indices.len() == 3);

        let points = indices
            .into_iter()
            .map(|i| {
                let p = &positions[i - 1];

                let a = p[0] * m[0][0] + p[1] * m[1][0] + p[2] * m[2][0] + m[3][0];
                let b = p[0] * m[0][1] + p[1] * m[1][1] + p[2] * m[2][1] + m[3][1];
                let c = p[0] * m[0][2] + p[1] * m[1][2] + p[2] * m[2][2] + m[3][2];
                let w = p[0] * m[0][3] + p[1] * m[1][3] + p[2] * m[2][3] + m[3][3];
                let coords = vec![a, b, c].into_iter().map(|c| c / w).collect();
                Vector::new(coords)
            })
            .collect::<Vec<_>>();

        shapes.push(Shape::Triangle(Triangle::new(id, points, material)));
        id += 1;
    }

    shapes
}

fn parse<T>(mut line: String) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    // the first char is either "v" or "f"
    line.split_off(1)
        .split_whitespace()
        .map(|n| n.trim().parse::<T>().unwrap())
        .collect::<Vec<T>>()
}

fn get_transform_matrix() -> [[f64; 4]; 4] {
    let a2 = PI / 2.0 - 0.5;
    let a6 = PI / 6.0;

    let n_cos_a6 = (-a6).cos();
    let n_cos_a2 = (-a2).cos();
    let p_cos_a2 = (a2).cos();
    let n_sin_a6 = (-a6).sin();
    let n_sin_a2 = (-a2).sin();
    let p_sin_a2 = (a2).sin();

    let a = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, n_cos_a2, n_sin_a2, 0.0],
        [0.0, -n_sin_a2, n_cos_a2, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let b = [
        [p_cos_a2, 0.0, -p_sin_a2, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [p_sin_a2, 0.0, p_cos_a2, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let c = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, n_cos_a6, n_sin_a6, 0.0],
        [0.0, -n_sin_a6, n_cos_a6, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];

    let d = [
        [15.0, 0.0, 0.0, 0.0],
        [0.0, 15.0, 0.0, 0.0],
        [0.0, 0.0, 15.0, 0.0],
        [0.0, 0.5, 10.0, 1.0],
    ];

    mul(mul(mul(a, b), c), d)
}

type Matrix4x4 = [[f64; 4]; 4];

fn mul(a: Matrix4x4, b: Matrix4x4) -> Matrix4x4 {
    let mut m = [
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
    ];

    for i in 0..4 {
        for j in 0..4 {
            m[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j] + a[i][3] * b[3][j];
        }
    }

    m
}

/// vector
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

/// png
use image::{ImageBuffer, Rgb, RgbImage};

pub fn write_png(path: &str, width: u32, height: u32, pixels: &[Vector]) {
    let mut buffer: RgbImage = ImageBuffer::new(width, height);

    for ((_, _, dst_pixel), src_pixel) in buffer.enumerate_pixels_mut().zip(pixels.iter()) {
        let p = src_pixel
            .coords
            .iter()
            .map(|c| c.max(0.0).min(1.0) * 255.0)
            .collect::<Vec<_>>();
        *dst_pixel = Rgb([p[0] as u8, p[1] as u8, p[2] as u8]);
    }

    buffer.save(path).unwrap();
}
