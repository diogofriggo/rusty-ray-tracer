use crate::vec3::Vec3;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: f64,
    pub image_height: f64,
    pub focal_length: f64,
    pub center: Vec3,
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub viewport_u: Vec3,
    pub viewport_v: Vec3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub viewport_upper_left: Vec3,
    pub pixel00_loc: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let image_width = 400.0;
        let image_height = ((image_width / aspect_ratio) as usize).max(1) as f64;

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * image_width / image_height;
        let center = Vec3::new(0.0, 0.0, 1.0);
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
        let pixel_delta_u = &viewport_u / image_width;
        let pixel_delta_v = &viewport_v / image_height;
        // makes no sense
        let viewport_upper_left =
            &center - &Vec3::new(0.0, 0.0, focal_length) - &viewport_u / 2.0 - &viewport_v / 2.0;
        let average = &(&pixel_delta_u + &pixel_delta_v) / 2.0;

        // makes even less sense
        let pixel00_loc = &viewport_upper_left + &average;

        Self {
            aspect_ratio,
            image_width,
            image_height,
            focal_length,
            center,
            viewport_height,
            viewport_width,
            viewport_u,
            viewport_v,
            pixel_delta_u,
            pixel_delta_v,
            viewport_upper_left,
            pixel00_loc,
        }
    }
}
