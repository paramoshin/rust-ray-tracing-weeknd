use super::*;

#[derive(Clone, Copy, Debug)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(a: Vec3, f: f64) -> Metal {
        let fuzz: f64;
        if f < 1.0 {
            fuzz = f;
        } else {
            fuzz = 1.0;
        }
        Metal {
            albedo: a,
            fuzz: fuzz,
        }
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - *n * (2.0 * Vec3::dot(v, n))
    }
}

impl Scatterable for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected: Vec3 = Metal::reflect(&Vec3::unit_vector(&r_in.direction()), &rec.normal);
        *scattered = Ray::new(rec.p, reflected + random_in_unit_sphere() * self.fuzz);
        *attenuation = self.albedo;
        Vec3::dot(&scattered.direction(), &rec.normal) > 0.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reflect() {
        let v: Vec3 = Vec3::new(1.0, 2.0, 4.0);
        let n: Vec3 = Vec3::new(0.0, 0.0, 0.0);

        assert_eq!(Metal::reflect(&v, &n), v);
    }
}
