use crate::rays::Ray;
use glam::Vec3;

pub struct Scene {
    hitables: Vec<Box<dyn Hittable>>,
    lights: (),
}

#[derive(Default)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
}

impl HitRecord {
    pub fn new(p: Vec3, normal: Vec3, t: f32) -> HitRecord {
        HitRecord { p, normal, t }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, rec: &mut HitRecord) -> bool;
}

#[derive(Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, rec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = ray.direction.dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrt_d = discriminant.sqrt();

        // Find root within range
        let mut root = (-half_b - sqrt_d) / a;
        if root <= tmin || tmax <= root {
            root = (-half_b + sqrt_d) / a;
            if root <= tmin || tmax <= root {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(root);
        rec.normal = (rec.p - self.center) / self.radius;
        true
    }
}
