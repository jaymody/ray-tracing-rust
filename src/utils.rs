use crate::vec3::Vec3;

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random(-1.0, 1.0);
        if p.length() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit_vector()
}

pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if normal.dot(in_unit_sphere) > 0.0 {
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}
