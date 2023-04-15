use crate::{
    hittable::HitRecord,
    ray::Ray,
    utils::{random_in_hemisphere, random_in_unit_sphere},
    vec3::Vec3,
};

pub trait Material: Sync {
    fn scatter(&self, ray: Ray, hit: &HitRecord) -> (Ray, Vec3);
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: Ray, hit: &HitRecord) -> (Ray, Vec3) {
        (
            Ray::new(hit.p, random_in_hemisphere(hit.normal)),
            self.albedo,
        )
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    let b = -(normal * v.dot(normal));
    v + b * 2.0
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
