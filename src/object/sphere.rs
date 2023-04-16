use crate::hitrecord::HitRecord;
use crate::material::Material;
use crate::object::Object;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Object for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;

        let a = ray.direction.squared_length();
        let half_b = ray.direction.dot(oc);
        let c = oc.squared_length() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let mut root = (-half_b - discriminant.sqrt()) / a;
        if !(t_min < root && root < t_max) {
            root = (-half_b + discriminant.sqrt()) / a;
            if !(t_min < root && root < t_max) {
                return None;
            }
        }

        let t = root;
        let p = ray.at(t);
        let normal = (p - self.center) / self.radius;
        let mut rec = HitRecord::new(p, normal, t, &*self.material);
        rec.set_face_normal(ray);

        Some(rec)
    }
}
