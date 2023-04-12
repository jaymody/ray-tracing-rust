use crate::{ray::Ray, vec3::Vec3};

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(origin: Vec3, horizontal: Vec3, vertical: Vec3, lower_left_corner: Vec3) -> Camera {
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn from_width_height_focal(
        origin: Vec3,
        viewport_width: f64,
        viewport_height: f64,
        focal_length: f64,
    ) -> Camera {
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let depth = Vec3::new(0.0, 0.0, focal_length);
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - depth;
        Camera::new(origin, horizontal, vertical, lower_left_corner)
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
