use super::*;

#[derive(Clone, Copy, Debug)]
pub struct Dielectric {
    pub ref_idx: f64,
}

impl Dielectric {
    pub fn new(ri: f64) -> Self {
        Dielectric { ref_idx: ri }
    }

    pub fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f64, refracted: &mut Vec3) -> bool {
        let uv: Vec3 = Vec3::unit_vector(&v);
        let dt: f64 = Vec3::dot(&uv, &n);
        let discr: f64 = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
        if discr > 0.0 {
            *refracted = (uv - *n * dt) * ni_over_nt - *n * discr.sqrt();
            return true;
        } else {
            return false;
        }
    }

    pub fn schlick(&self, cosine: f64) -> f64 {
        let mut r0: f64 = (1.0 - self.ref_idx) / (1.0 + self.ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Scatterable for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let outward_normal: Vec3;
        let reflected: Vec3 = Metal::reflect(&r_in.direction(), &rec.normal);
        let ni_over_nt: f64;
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let mut refracted: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let reflect_prob: f64;
        let cosine: f64;
        let mut rng = rand::thread_rng();
        if Vec3::dot(&r_in.direction(), &rec.normal) > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * Vec3::dot(&r_in.direction(), &rec.normal)
                / r_in.direction().length();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -Vec3::dot(&r_in.direction(), &rec.normal) / r_in.direction().length();
        }
        if Self::refract(
            &r_in.direction(),
            &outward_normal,
            ni_over_nt,
            &mut refracted,
        ) {
            reflect_prob = self.schlick(cosine);
        } else {
            *scattered = Ray::new(rec.p, reflected);
            reflect_prob = 1.0;
        }
        if rng.gen::<f64>() < reflect_prob {
            *scattered = Ray::new(rec.p, reflected);
        } else {
            *scattered = Ray::new(rec.p, refracted);
        }
        return true;
    }
}
