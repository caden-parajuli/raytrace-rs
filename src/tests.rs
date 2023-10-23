use glam::Vec3;
use image::Rgba;
use std::env;

use crate::camera::Camera;

#[test]
fn images() {
    let mut filename = env::temp_dir();
    filename.push("test.png");
    let height: u32 = 256;
    let width: u32 = 256;
    let mut imgbuf: image::RgbaImage = image::ImageBuffer::new(width, height);

    let my_closure = |(i, j, pixel): (u32, u32, &mut Rgba<u8>)| {
        *pixel = image::Rgba([
            (i as f32 * 255f32 / (width as f32 - 1f32)) as u8,
            (j as f32 * 255f32 / (height as f32 - 1f32)) as u8,
            0,
            255,
        ])
    };

    imgbuf.enumerate_pixels_mut().for_each(my_closure);
    imgbuf.save(filename.to_str().unwrap()).unwrap();
}

#[test]
fn vectors() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(6.0, 4.0, 5.0);
    assert_eq!(a + b, Vec3::new(7.0, 6.0, 8.0));
    assert_eq!(a * b, Vec3::new(6.0, 8.0, 15.0));
    assert_eq!(a.dot(b), 29.0);
    assert_eq!(a.length_squared(), 14.0);
    assert_eq!(b / 2.0, Vec3::new(3.0, 2.0, 2.5));
}

#[test]
fn render_test() {
    let mut filename = env::temp_dir();
    filename.push("render_test.png");

    let cam: Camera = Camera::new(1920, 16.0 / 9.0, 2.0, Vec3::new(0.0, 0.0, 0.0), 1.0);
    cam.render().save(filename.to_str().unwrap()).unwrap();
}
