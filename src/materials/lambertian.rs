use super::*;

#[derive(Copy, Clone, Debug)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(a: Vec3) -> Lambertian {
        Lambertian { albedo: a }
    }
}

impl Scatterable for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let target: Vec3 = rec.p + rec.normal + random_in_unit_sphere();
        *scattered = Ray::new(rec.p, target - rec.p);
        *attenuation = self.albedo;
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_point() {
        let vec: Vec3 = random_in_unit_sphere();
        assert_eq!(Vec3::dot(&vec, &Vec3::new(0.0, 0.0, 0.0)), 0.0);
    }
}
