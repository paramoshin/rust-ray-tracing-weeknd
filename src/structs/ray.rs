use crate::structs::vec3::Vec3;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Ray {
    pub a: Vec3,
    pub b: Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Ray {
        Ray { a: a, b: b }
    }

    pub fn origin(&self) -> Vec3 {
        self.a
    }

    pub fn direction(&self) -> Vec3 {
        self.b
    }

    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.a + self.b * t
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_origin() {
        let orig = Vec3 {
            d0: 1.0,
            d1: 1.0,
            d2: 1.0,
        };
        let dir = Vec3 {
            d0: 1.0,
            d1: 1.0,
            d2: 0.0,
        };
        let ray = Ray { a: orig, b: dir };
        assert_eq!(orig, ray.origin())
    }

    #[test]
    fn test_direction() {
        let orig = Vec3 {
            d0: 1.0,
            d1: 1.0,
            d2: 1.0,
        };
        let dir = Vec3 {
            d0: 1.0,
            d1: 1.0,
            d2: 0.0,
        };
        let ray = Ray { a: orig, b: dir };
        assert_eq!(dir, ray.direction())
    }

    #[test]
    fn test_p() {
        let f: f64 = 2.5;
        let orig = Vec3 {
            d0: 1.0,
            d1: 1.0,
            d2: 1.0,
        };
        let dir = Vec3 {
            d0: 1.0,
            d1: 1.0,
            d2: 0.0,
        };
        let ray = Ray { a: orig, b: dir };
        assert_eq!(
            Vec3 {
                d0: 3.5,
                d1: 3.5,
                d2: 1.0
            },
            ray.point_at_parameter(f)
        )
    }
}
