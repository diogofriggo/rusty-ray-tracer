use std::{
    f64::consts::PI,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use crate::{vector::Vector, Material, Shape, Triangle};

pub fn read<'a>(path: &'_ str, material: &'a Material) -> Vec<Shape<'a>> {
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

        shapes.push(Shape::Triangle(Triangle::new(points, material)));
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
