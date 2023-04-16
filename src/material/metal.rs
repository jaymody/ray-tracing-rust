use crate::utils::{random_in_unit_sphere, reflect};
use crate::{hitrecord::HitRecord, material::Material, ray::Ray, vec3::Vec3};

pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, hit: &HitRecord) -> (Ray, Vec3) {
        (
            Ray::new(
                hit.p,
                reflect(ray.direction, hit.normal) + random_in_unit_sphere() * self.fuzz,
            ),
            self.albedo,
        )
    }
}
