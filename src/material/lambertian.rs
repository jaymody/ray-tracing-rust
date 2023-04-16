use crate::utils::random_in_hemisphere;
use crate::{hitrecord::HitRecord, material::Material, ray::Ray, vec3::Vec3};

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: Ray, hit: &HitRecord) -> (Ray, Vec3) {
        (
            Ray::new(hit.p, random_in_hemisphere(hit.normal)),
            self.albedo,
        )
    }
}
