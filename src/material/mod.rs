use crate::{hitrecord::HitRecord, ray::Ray, vec3::Vec3};

pub mod dielectric;
pub mod lambertian;
pub mod metal;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

pub trait Material: Sync {
    fn scatter(&self, ray: Ray, hit: &HitRecord) -> (Ray, Vec3);
}
