use crate::hitrecord::HitRecord;
use crate::ray::Ray;

pub mod sphere;

pub use sphere::Sphere;

pub trait Object: Sync {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
