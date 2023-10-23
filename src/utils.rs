use glam::{Vec3, Vec4};
use image::Rgba;

pub fn rgba_from_float(r: f32, g: f32, b: f32, a: f32) -> Rgba<u8> {
    Rgba([
        ((255f32) * r) as u8,
        ((255f32) * g) as u8,
        ((255f32) * b) as u8,
        ((255f32) * a) as u8,
    ])
}

pub fn rgba_from_vec3(v: Vec3) -> Rgba<u8> {
    return Rgba([
        ((255f32) * v.x) as u8,
        ((255f32) * v.y) as u8,
        ((255f32) * v.z) as u8,
        255u8,
    ]);
}

pub fn rgba_from_vec4(v: Vec4) -> Rgba<u8> {
    Rgba([
        ((255f32) * v.x) as u8,
        ((255f32) * v.y) as u8,
        ((255f32) * v.z) as u8,
        ((255f32) * v.w) as u8,
    ])
}
