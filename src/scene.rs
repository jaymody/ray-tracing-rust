use crate::hitrecord::HitRecord;
use crate::object::Object;
use crate::ray::Ray;

pub struct Scene {
    pub list: Vec<Box<dyn Object>>,
}

impl Scene {
    pub fn new_empty() -> Scene {
        Scene { list: Vec::new() }
    }

    pub fn add(&mut self, object: Box<dyn Object>) {
        self.list.push(object);
    }
}

impl Object for Scene {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for obj in self.list.iter() {
            if let Some(temp_rec) = obj.hit(ray, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                rec = Some(temp_rec);
            }
        }

        rec
    }
}

#[allow(dead_code)]
pub mod scenes {
    use crate::material::{Dielectric, Lambertian, Material, Metal};
    use crate::object::Sphere;
    use crate::scene::Scene;
    use crate::vec3::Vec3;

    use rand::Rng;

    pub fn scene1() -> Scene {
        let mut scene = Scene::new_empty();

        scene.add(Box::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
        )));
        scene.add(Box::new(Sphere::new(
            Vec3::new(0.0, 1.0, 0.0),
            1.0,
            Box::new(Lambertian::new(Vec3::new(0.6, 0.2, 0.2))),
        )));

        scene
    }

    pub fn scene2() -> Scene {
        let mut scene = Scene::new_empty();

        scene.add(Box::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
        )));

        scene.add(Box::new(Sphere::new(
            Vec3::new(0.0, 1.0, 0.0),
            1.0,
            Box::new(Dielectric::new(1.5)),
        )));
        scene.add(Box::new(Sphere::new(
            Vec3::new(-4.0, 1.0, 0.0),
            1.0,
            Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
        )));
        scene.add(Box::new(Sphere::new(
            Vec3::new(4.0, 1.0, 0.0),
            1.0,
            Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.5)),
        )));

        scene
    }

    pub fn scene3() -> Scene {
        let mut scene = Scene::new_empty();

        // ground material
        scene.add(Box::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
        )));

        // random spheres in the scene
        for a in -11..11 {
            for b in -11..11 {
                let choose_mat: f64 = rand::thread_rng().gen_range(0.0..1.0);
                let center = Vec3::new(
                    a as f64 + 0.9 * rand::thread_rng().gen_range(0.0..1.0),
                    0.2,
                    b as f64 + 0.9 * rand::thread_rng().gen_range(0.0..1.0),
                );

                if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    let material: Box<dyn Material> = if choose_mat < 0.8 {
                        Box::new(Lambertian::new(
                            Vec3::random(0.0, 1.0) * Vec3::random(0.0, 1.0),
                        ))
                    } else if choose_mat < 0.95 {
                        Box::new(Metal::new(
                            Vec3::random(0.5, 1.0),
                            rand::thread_rng().gen_range(0.0..0.5),
                        ))
                    } else {
                        Box::new(Dielectric::new(1.5))
                    };

                    scene.add(Box::new(Sphere::new(center, 0.2, material)));
                }
            }
        }

        scene.add(Box::new(Sphere::new(
            Vec3::new(0.0, 1.0, 0.0),
            1.0,
            Box::new(Dielectric::new(1.5)),
        )));
        scene.add(Box::new(Sphere::new(
            Vec3::new(-4.0, 1.0, 0.0),
            1.0,
            Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
        )));
        scene.add(Box::new(Sphere::new(
            Vec3::new(4.0, 1.0, 0.0),
            1.0,
            Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.5)),
        )));

        scene
    }
}
