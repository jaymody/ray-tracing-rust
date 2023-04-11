use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
}

impl HitRecord {
    pub fn new_empty() -> HitRecord {
        HitRecord {
            p: Vec3::ZERO,
            normal: Vec3::ZERO,
            t: 0.0,
        }
    }
}

pub trait Hittable {
    fn hit(self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
