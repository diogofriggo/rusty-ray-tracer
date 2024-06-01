use crate::vector::Vector;
use std::{
    fs::File,
    io::{BufWriter, Write},
};

pub fn write(path: &str, m: usize, n: usize, pixels: &[Vector]) {
    let file = File::create(path).unwrap();
    let mut file = BufWriter::new(file);
    writeln!(file, "P3").unwrap();
    writeln!(file, "{m} {n}").unwrap();
    writeln!(file, "255").unwrap();

    let color = |v: &Vector| -> String {
        let v = v
            .unit()
            .coords
            .iter()
            .map(|c| (c.max(0.0).min(1.0) * 255.0) as usize)
            .collect::<Vec<_>>();

        format!("{} {} {}", v[0], v[1], v[2])
    };

    for pixel in pixels {
        writeln!(file, "{}", color(pixel)).unwrap();
    }
}
