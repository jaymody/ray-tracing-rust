use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new_empty() -> HitRecord {
        HitRecord {
            p: Vec3::ZERO,
            normal: Vec3::ZERO,
            t: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.front_face = outward_normal.dot(ray.direction) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
