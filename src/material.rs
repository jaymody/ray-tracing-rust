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
    fn scatter(&self, _ray: Ray, hit: &HitRecord) -> (Ray, Vec3) {
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

pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Dielectric {
        Dielectric { ir }
    }
}

fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = n.dot(-uv).min(1.0);
    let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
    let r_out_parallel = n * -((1.0 - r_out_perp.squared_length()).abs().sqrt());
    r_out_perp + r_out_parallel
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray, hit: &HitRecord) -> (Ray, Vec3) {
        let refraction_ratio = if hit.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = ray.direction.unit_vector();
        let cos_theta = hit.normal.dot(-unit_direction).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        (
            Ray::new(
                hit.p,
                if refraction_ratio * sin_theta > 1.0
                    || reflectance(cos_theta, refraction_ratio) > rand::random()
                {
                    reflect(unit_direction, hit.normal)
                } else {
                    refract(unit_direction, hit.normal, refraction_ratio)
                },
            ),
            Vec3::new(1.0, 1.0, 1.0),
        )
    }
}
