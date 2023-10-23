use crate::scene::{HitRecord, Hittable, Sphere};
use crate::{rays::Ray, utils::rgba_from_vec3};
use glam::Vec3;
use image::Rgba;

pub struct Camera {
    pub image_width: u32,
    pub image_height: u32,
    pub aspect_ratio: f32,
    pub viewport_height: f32,
    pub viewport_width: f32,
    pub center: Vec3,
    pub focal_length: f32,
}

impl Camera {
    pub fn new(
        image_width: u32,
        aspect_ratio: f32,
        viewport_height: f32,
        center: Vec3,
        focal_length: f32,
    ) -> Self {
        let mut image_height = (image_width as f32 / aspect_ratio) as u32;
        image_height = if image_height > 0 { image_height } else { 1 };
        let viewport_width = viewport_height * ((image_width as f32) / (image_height as f32));

        Camera {
            image_width,
            image_height,
            aspect_ratio,
            viewport_height,
            viewport_width,
            center,
            focal_length,
        }
    }

    pub fn ray_color(&self, ray: Ray) -> Rgba<u8> {
        let sphere = Sphere::new(Vec3::new(-0.1, 0.2, -1.0), 0.5);
        let mut rec = HitRecord::default();
        if sphere.hit(&ray, 0.0, 100.0, &mut rec) {
            return rgba_from_vec3(0.5 * (rec.normal + Vec3::new(1.0, 1.0, 1.0)));
        }

        let a = 0.5 * (ray.direction.normalize().y + 1.0);
        rgba_from_vec3((1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0))
    }

    pub fn render(&self) -> image::RgbaImage {
        // Calculate viewport edge vectors
        let viewport_u = Vec3::new(self.viewport_width, 0f32, 0f32);
        let viewport_v = Vec3::new(0f32, -self.viewport_height, 0f32);

        // Calculate step for each direction
        let du = viewport_u / self.image_width as f32;
        let dv = viewport_v / self.image_height as f32;

        // Calculate upper left pixel location
        let viewport_upper_left: Vec3 = self.center
            - Vec3::new(0f32, 0f32, self.focal_length)
            - viewport_u / 2f32
            - viewport_v / 2f32;
        let upper_left_pixel = viewport_upper_left + 0.5 * (du + dv);

        // Render
        let mut pixel_center: Vec3;
        let mut ray_dir: Vec3;
        let mut ray: Ray;
        // let mut pixel_color: Rgba<u8>;
        let mut imgbuf: image::RgbaImage =
            image::ImageBuffer::new(self.image_width, self.image_height);

        for j in 0..self.image_height {
            println!("Finished row {j}/{}", self.image_height);
            for i in 0..self.image_width {
                // Calculate the ray
                pixel_center = upper_left_pixel + i as f32 * du + j as f32 * dv;
                ray_dir = pixel_center - self.center;
                ray = Ray::new(self.center, ray_dir);

                // Set the pixel
                *imgbuf.get_pixel_mut(i, j) = self.ray_color(ray);
            }
        }
        imgbuf
    }
}
