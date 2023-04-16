use crate::vec3::Vec3;

use rand::Rng;

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random(-1.0, 1.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if normal.dot(in_unit_sphere) > 0.0 {
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(
            rand::thread_rng().gen_range(-1.0..1.0),
            rand::thread_rng().gen_range(-1.0..1.0),
            0.0,
        );
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

pub fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    let b = -(normal * v.dot(normal));
    v + b * 2.0
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = n.dot(-uv).min(1.0);
    let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
    let r_out_parallel = n * -((1.0 - r_out_perp.squared_length()).abs().sqrt());
    r_out_perp + r_out_parallel
}

pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
