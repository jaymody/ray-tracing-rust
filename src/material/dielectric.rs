use crate::utils::{reflect, reflectance, refract};
use crate::{hitrecord::HitRecord, material::Material, ray::Ray, vec3::Vec3};

pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Dielectric {
        Dielectric { ir }
    }
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
