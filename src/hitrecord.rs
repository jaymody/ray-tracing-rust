use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord<'a> {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(p: Vec3, normal: Vec3, t: f64, material: &'a dyn Material) -> HitRecord<'a> {
        HitRecord {
            p,
            normal,
            t,
            front_face: true,
            material: material,
        }
    }

    pub fn set_face_normal(&mut self, ray: Ray) {
        self.front_face = self.normal.dot(ray.direction) < 0.0;
        if !self.front_face {
            self.normal = -self.normal;
        }
    }
}
