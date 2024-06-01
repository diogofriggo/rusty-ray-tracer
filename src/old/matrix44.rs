use crate::vector::Vector;
use std::{f64::consts::PI, ops::Mul};

#[derive(Clone, Debug, PartialEq)]
pub struct Matrix44 {
    pub rows: Vec<Vector>,
    // pub cols: Vec<Vector>,
}

impl Matrix44 {
    pub fn from_rows(rows: Vec<Vector>) -> Self {
        // let cols = transpose(&rows);
        Self { rows }
    }
}

const PI2: f64 = PI / 2.0;
const PI6: f64 = PI / 2.0;

// pub fn transpose(rows: &[Vector]) -> Vec<Vector> {
//     let mut row0 = vec![];
//     let mut row1 = vec![];
//     let mut row2 = vec![];
//     let mut row3 = vec![];
//     for row in rows {
//         row0.push(row.coords[0]);
//         row1.push(row.coords[1]);
//         row2.push(row.coords[2]);
//         row3.push(row.coords[3]);
//     }
//     let rows = vec![row0, row1, row2, row3];
//     rows.into_iter().map(|coords| Vector { coords }).collect()
// }

pub fn world_matrix() -> Matrix44 {
    let a = Matrix44::from_rows(vec![
        Vector::new_abcd(1.0, 0.0, 0.0, 0.0),
        Vector::new_abcd(0.0, (-PI2).cos(), (-PI2).sin(), 0.0),
        Vector::new_abcd(0.0, -(-PI2).sin(), (-PI2).cos(), 0.0),
        Vector::new_abcd(0.0, 0.0, 0.0, 1.0),
    ]);

    let b = Matrix44::from_rows(vec![
        Vector::new_abcd(PI2.cos(), 0.0, -PI2.sin(), 0.0),
        Vector::new_abcd(0.0, 1.0, 0.0, 0.0),
        Vector::new_abcd(PI2.sin(), 0.0, PI2.cos(), 0.0),
        Vector::new_abcd(0.0, 0.0, 0.0, 1.0),
    ]);

    let c = Matrix44::from_rows(vec![
        Vector::new_abcd(1.0, 0.0, 0.0, 0.0),
        Vector::new_abcd(0.0, (-PI6).cos(), (-PI6).sin(), 0.0),
        Vector::new_abcd(0.0, -(-PI6).sin(), (-PI6).cos(), 0.0),
        Vector::new_abcd(0.0, 0.0, 0.0, 1.0),
    ]);

    let d = Matrix44::from_rows(vec![
        Vector::new_abcd(15.0, 0.0, 0.0, 0.0),
        Vector::new_abcd(0.0, 15.0, 0.0, 0.0),
        Vector::new_abcd(0.0, 0.0, 15.0, 0.0),
        Vector::new_abcd(0.0, 0.5, 10.0, 1.0),
    ]);

    print!("test: {:?}", (a.clone() * b.clone()));
    a * b * c * d
}

impl Mul<&Matrix44> for &Vector {
    type Output = Vector;
    fn mul(self, rhs: &Matrix44) -> Self::Output {
        rhs * self
    }
}

impl Mul<&Vector> for &Matrix44 {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Self::Output {
        let m = self
            .rows
            .iter()
            .map(|row| row.coords.clone())
            .collect::<Vec<_>>();

        let x = &rhs.coords;

        let a = x[0] * m[0][0] + x[1] * m[1][0] + x[2] * m[2][0] + m[3][0];
        let b = x[0] * m[0][1] + x[1] * m[1][1] + x[2] * m[2][1] + m[3][1];
        let c = x[0] * m[0][2] + x[1] * m[1][2] + x[2] * m[2][2] + m[3][2];
        let w = x[0] * m[0][3] + x[1] * m[1][3] + x[2] * m[2][3] + m[3][3];

        let coords = vec![a / w, b / w, c / w];
        Vector { coords }
    }
}

impl Mul for Matrix44 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let a = self.rows;
        let b = rhs.rows;
        // println!("START");
        let mut rows = vec![];

        for i in 0..4 {
            let mut coords = vec![];
            for j in 0..4 {
                let ei0 = a[i].coords[0] * b[0].coords[j];
                let ei1 = a[i].coords[1] * b[1].coords[j];
                let ei2 = a[i].coords[2] * b[2].coords[j];
                let ei3 = a[i].coords[3] * b[3].coords[j];
                let e = ei0 + ei1 + ei2 + ei3;
                coords.push(e);
            }
            rows.push(Vector { coords });
        }

        // for row in &self.rows {
        //     let mut coords = vec![];
        //     for col in &rhs.cols {
        //         let coord = row
        //             .coords
        //             .iter()
        //             .zip(col.coords.iter())
        //             .map(|(a, b)| {
        //                 println!("{a} * {b}");
        //                 a * b
        //             })
        //             .sum();
        //         coords.push(coord);
        //     }
        //     rows.push(Vector { coords });
        // }
        // println!("END");

        Matrix44::from_rows(rows)
    }
}

#[cfg(test)]
mod tests {
    use crate::{matrix44::world_matrix, vector::Vector};

    #[test]
    fn test_world() {
        let harcoded = vec![
            Vector {
                coords: vec![0.0, -7.5, -12.99038, 0.0],
            },
            Vector {
                coords: vec![-15.0, 0.0, -0.0, 0.0],
            },
            Vector {
                coords: vec![0.0, 12.99038, -7.5, 0.0],
            },
            Vector {
                coords: vec![0.0, 0.5, 10.0, 1.0],
            },
        ];

        let m = world_matrix();
        println!("{:?}", m);
        assert_eq!(m.rows, harcoded);
    }
}
// m * Vector { coords: [0.0, 1.0, -1.0] } = Vector { coords: [-15.0, -12.49038, 17.5] }
