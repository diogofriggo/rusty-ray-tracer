use crate::triangle::Triangle;
use camera::Camera;
use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use ray::Ray;
use rayon::prelude::*;
use std::{
    fs::File,
    io::{BufWriter, Write},
};
use vec3::Vec3;

mod camera;
mod ray;
mod triangle;
mod vec3;

fn main() {
    let camera = Camera::new();

    let path = "hello_world.ppm";
    // get_obj_triangles();
    // println!("{:?}", "");
    write_to_ppm(path, &camera);
}

fn get_obj_triangles(obj_file: &str) -> Vec<Triangle> {
    let (models, _) =
        tobj::load_obj(obj_file, &tobj::LoadOptions::default()).expect("Failed to OBJ load file");

    let mut triangles = vec![];
    for m in models {
        let mesh = &m.mesh;
        println!("{}", mesh.positions.len());
        println!("{}", mesh.indices.len());

        assert!(mesh.positions.len() % 3 == 0);

        for indices in mesh.indices.chunks(3) {
            let i = indices[0] as usize;
            let a = Vec3::new(
                mesh.positions[i * 3] as f64,
                mesh.positions[i * 3 + 1] as f64,
                mesh.positions[i * 3 + 2] as f64,
            );
            let i = indices[1] as usize;
            let b = Vec3::new(
                mesh.positions[i * 3] as f64,
                mesh.positions[i * 3 + 1] as f64,
                mesh.positions[i * 3 + 2] as f64,
            );
            let i = indices[2] as usize;
            let c = Vec3::new(
                mesh.positions[i * 3] as f64,
                mesh.positions[i * 3 + 1] as f64,
                mesh.positions[i * 3 + 2] as f64,
            );
            let triangle = Triangle::new(a, b, c);
            // println!("{triangle:?}");
            triangles.push(triangle);
        }

        // for positions in mesh.positions.chunks(3) {
        //     println!(
        //         "{} {} {} {}",
        //         positions.len(),
        //         positions[0],
        //         positions[1],
        //         positions[2]
        //     );
        //     // Triangle::new()
        //     // println!(
        //     //     "              position[{}] = ({}, {}, {})",
        //     //     vtx,
        //     //     mesh.positions[3 * vtx],
        //     //     mesh.positions[3 * vtx + 1],
        //     //     mesh.positions[3 * vtx + 2]
        //     // );
        // }
    }
    triangles
}

fn explore_obj() {
    let obj_file = "teapot.obj";

    let (models, materials) =
        tobj::load_obj(&obj_file, &tobj::LoadOptions::default()).expect("Failed to OBJ load file");

    // Note: If you don't mind missing the materials, you can generate a default.
    let materials = materials.expect("Failed to load MTL file");

    println!("Number of models          = {}", models.len());
    println!("Number of materials       = {}", materials.len());

    for (i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        println!("");
        println!("model[{}].name             = \'{}\'", i, m.name);
        println!("model[{}].mesh.material_id = {:?}", i, mesh.material_id);

        println!(
            "model[{}].face_count       = {}",
            i,
            mesh.face_arities.len()
        );

        let mut next_face = 0;
        for face in 0..mesh.face_arities.len() {
            let end = next_face + mesh.face_arities[face] as usize;

            let face_indices = &mesh.indices[next_face..end];
            println!(" face[{}].indices          = {:?}", face, face_indices);

            if !mesh.texcoord_indices.is_empty() {
                let texcoord_face_indices = &mesh.texcoord_indices[next_face..end];
                println!(
                    " face[{}].texcoord_indices = {:?}",
                    face, texcoord_face_indices
                );
            }
            if !mesh.normal_indices.is_empty() {
                let normal_face_indices = &mesh.normal_indices[next_face..end];
                println!(
                    " face[{}].normal_indices   = {:?}",
                    face, normal_face_indices
                );
            }

            next_face = end;
        }

        // Normals and texture coordinates are also loaded, but not printed in
        // this example.
        println!(
            "model[{}].positions        = {}",
            i,
            mesh.positions.len() / 3
        );
        assert!(mesh.positions.len() % 3 == 0);

        for vtx in 0..mesh.positions.len() / 3 {
            println!(
                "              position[{}] = ({}, {}, {})",
                vtx,
                mesh.positions[3 * vtx],
                mesh.positions[3 * vtx + 1],
                mesh.positions[3 * vtx + 2]
            );
        }
    }

    // for (i, m) in materials.iter().enumerate() {
    //     println!("material[{}].name = \'{}\'", i, m.name);
    //     println!(
    //         "    material.Ka = ({}, {}, {})",
    //         m.ambient[0], m.ambient[1], m.ambient[2]
    //     );
    //     println!(
    //         "    material.Kd = ({}, {}, {})",
    //         m.diffuse[0], m.diffuse[1], m.diffuse[2]
    //     );
    //     println!(
    //         "    material.Ks = ({}, {}, {})",
    //         m.specular[0], m.specular[1], m.specular[2]
    //     );
    //     println!("    material.Ns = {}", m.shininess);
    //     println!("    material.d = {}", m.dissolve);
    //     println!("    material.map_Ka = {}", m.ambient_texture);
    //     println!("    material.map_Kd = {}", m.diffuse_texture);
    //     println!("    material.map_Ks = {}", m.specular_texture);
    //     println!("    material.map_Ns = {}", m.shininess_texture);
    //     println!("    material.map_Bump = {}", m.normal_texture);
    //     println!("    material.map_d = {}", m.dissolve_texture);
    //
    //     for (k, v) in &m.unknown_param {
    //         println!("    material.{} = {}", k, v);
    //     }
    // }
}

fn write_to_ppm(path: &str, camera: &Camera) {
    let file = File::create(path).unwrap();
    let mut file = BufWriter::new(file);
    writeln!(file, "P3").unwrap();
    let image_width = camera.image_width as usize;
    let image_height = camera.image_height as usize;
    writeln!(file, "{image_width} {image_height}").unwrap();
    writeln!(file, "255").unwrap();

    let sphere_center = Vec3::new(0.0, 0.0, -1.0);
    let red = Vec3::new(1.0, 0.0, 0.0).color();
    println!("pixel00_loc: {}", camera.pixel00_loc);
    println!("pixel_delta_u: {}", camera.pixel_delta_u);
    println!("pixel_delta_v: {}", camera.pixel_delta_v);

    let a = Vec3::new(-1.0, -1.0, -1.0);
    let b = Vec3::new(1.0, -1.0, -1.0);
    let c = Vec3::new(0.0, 1.0, -1.0);

    let a = Vec3::new(1.3680740594863892, 2.435436964035034, -0.22740299999713898);
    let b = Vec3::new(1.3819680213928223, 2.4000000953674316, -0.22971199452877045);
    let c = Vec3::new(1.399999976158142, 2.4000000953674316, 0.0);
    let triangle = Triangle::new(a, b, c);
    let triangles = vec![triangle];
    //Triangle { a: Vec3 { x: 0.23714900016784668, y: 0.08452499657869339, z: 1.4267090559005737 }, b: Vec3 { x: 0.22682400047779083, y: 0.06480000168085098, z: 1.36459505558013 92 }, c: Vec3 { x: 0.4410409927368164, y: 0.06480000168085098, z: 1.3129479885101318 } }

    let triangles = get_obj_triangles("octa.obj")
        // let triangles = get_obj_triangles("test.obj")
        .into_iter()
        // .skip(5000)
        // .take(1)
        .collect::<Vec<_>>();
    //
    for triangle in &triangles {
        println!("{}", triangle);
    }

    // f 2909 2921 2939
    //  1.368074            2.435437           -0.227403
    // (1.3680740594863892, 2.435436964035034, -0.22740299999713898)
    //  1.381968            2.400000            -0.229712
    // (1.3819680213928223, 2.4000000953674316, -0.22971199452877045)
    //  1.400000           2.400000            0.000000
    // (1.399999976158142, 2.4000000953674316, 0)

    let tuples = (0..image_height)
        .cartesian_product(0..image_width)
        .collect::<Vec<(usize, usize)>>();

    let colors = tuples
        .into_par_iter()
        .progress()
        .flat_map(|(j, i): (usize, usize)| {
            let a = &camera.pixel_delta_u * i;
            let b = &camera.pixel_delta_v * j;
            let pixel_center = &camera.pixel00_loc + &(&a + &b);
            let ray_direction = &pixel_center - &camera.center;
            let ray = Ray::new(pixel_center, ray_direction);

            let ray_color = ray.color();
            // let color = if hits_sphere(&sphere_center, 0.5, &ray) {

            triangles
                .iter()
                .map(|triangle| {
                    if hits_triangle(triangle, &ray) {
                        red.clone()
                    } else {
                        ray_color.clone()
                    }
                })
                .collect::<Vec<String>>()
        })
        .collect::<Vec<String>>();

    for color in colors {
        writeln!(file, "{}", color).unwrap();
    }
}

fn hits_sphere(sphere_center: &Vec3, r: f64, ray: &Ray) -> bool {
    let oc = sphere_center - &ray.origin;
    let a = ray.direction.length_squared();
    let b = ray.direction.dot(&oc) * -2.0;
    let c = oc.length_squared() - r * r;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}

fn hits_triangle(triangle: &Triangle, ray: &Ray) -> bool {
    let tri_flat_normal = &triangle.normal();
    let point_on_plane = &triangle.a;
    let n_dot_d = tri_flat_normal.dot(&ray.direction);
    if n_dot_d.abs() < 0.0001 {
        return false;
    }
    let n_dot_ps = tri_flat_normal.dot(&(point_on_plane - &ray.origin));
    let t = n_dot_ps / n_dot_d;
    let plane_point = &ray.origin + &(&ray.direction * t);

    // now
    let a_to_b_edge = &triangle.b - &triangle.a;
    let b_to_c_edge = &triangle.c - &triangle.b;
    let c_to_a_edge = &triangle.a - &triangle.c;

    let a_to_point = &plane_point - &triangle.a;
    let b_to_point = &plane_point - &triangle.b;
    let c_to_point = &plane_point - &triangle.c;

    let a_test_vec = a_to_b_edge.cross(&a_to_point);
    let b_test_vec = b_to_c_edge.cross(&b_to_point);
    let c_test_vec = c_to_a_edge.cross(&c_to_point);

    let a_test_vec_matches_normal = a_test_vec.dot(tri_flat_normal) > 0.0;
    let b_test_vec_matches_normal = b_test_vec.dot(tri_flat_normal) > 0.0;
    let c_test_vec_matches_normal = c_test_vec.dot(tri_flat_normal) > 0.0;

    a_test_vec_matches_normal && b_test_vec_matches_normal && c_test_vec_matches_normal
}
