use std::fmt;
use std::ops;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub d0: f64,
    pub d1: f64,
    pub d2: f64,
}

impl Vec3 {
    pub fn new(d0: f64, d1: f64, d2: f64) -> Vec3 {
        Vec3 { d0, d1, d2 }
    }

    pub fn x(&self) -> f64 {
        self.d0
    }

    pub fn y(&self) -> f64 {
        self.d1
    }

    pub fn z(&self) -> f64 {
        self.d2
    }

    pub fn r(&self) -> f64 {
        self.d0
    }

    pub fn g(&self) -> f64 {
        self.d1
    }

    pub fn b(&self) -> f64 {
        self.d2
    }

    pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
        v1.d0 * v2.d0 + v1.d1 * v2.d1 + v1.d2 * v2.d2
    }

    pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3 {
            d0: (v2.d2 * v1.d1 - v2.d1 * v1.d2),
            d1: (v2.d0 * v1.d2 - v2.d2 * v1.d0),
            d2: (v2.d1 * v1.d0 - v2.d0 * v1.d1),
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.d0 * self.d0 + self.d1 * self.d1 + self.d2 * self.d2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(v: &Vec3) -> Vec3 {
        *v / v.length()
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.d0, self.d1, self.d2)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            d0: self.d0 + other.d0,
            d1: self.d1 + other.d1,
            d2: self.d2 + other.d2,
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.d0 += other.d0;
        self.d1 += other.d1;
        self.d2 += other.d2;
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            d0: self.d0 - other.d0,
            d1: self.d1 - other.d1,
            d2: self.d2 - other.d2,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            d0: self.d0 * other.d0,
            d1: self.d1 * other.d1,
            d2: self.d2 * other.d2,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, f: f64) -> Vec3 {
        Vec3 {
            d0: self.d0 * f,
            d1: self.d1 * f,
            d2: self.d2 * f,
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, f: f64) {
        self.d0 *= f;
        self.d1 *= f;
        self.d2 *= f;
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            d0: self.d0 / other.d0,
            d1: self.d1 / other.d1,
            d2: self.d2 / other.d2,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, f: f64) -> Vec3 {
        Vec3 {
            d0: self.d0 / f,
            d1: self.d1 / f,
            d2: self.d2 / f,
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, f: f64) {
        self.d0 /= f;
        self.d1 /= f;
        self.d2 /= f;
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            d0: -self.d0,
            d1: -self.d1,
            d2: -self.d2,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let vec1 = Vec3 {
            d0: 132 as f64,
            d1: 132 as f64,
            d2: 132 as f64,
        };
        let vec2 = Vec3 {
            d0: 123 as f64,
            d1: 123 as f64,
            d2: 123 as f64,
        };
        assert_eq!(
            Vec3 {
                d0: 255 as f64,
                d1: 255 as f64,
                d2: 255 as f64,
            },
            vec1 + vec2
        )
    }

    #[test]
    fn test_sub() {
        let vec1 = Vec3 {
            d0: 132 as f64,
            d1: 132 as f64,
            d2: 132 as f64,
        };
        let vec2 = Vec3 {
            d0: 123 as f64,
            d1: 123 as f64,
            d2: 123 as f64,
        };
        assert_eq!(
            Vec3 {
                d0: 9 as f64,
                d1: 9 as f64,
                d2: 9 as f64,
            },
            vec1 - vec2
        )
    }

    #[test]
    fn test_mul() {
        let vec1 = Vec3 {
            d0: 50 as f64,
            d1: 50 as f64,
            d2: 50 as f64,
        };
        let vec2 = Vec3 {
            d0: 1 as f64,
            d1: 2 as f64,
            d2: 3 as f64,
        };
        assert_eq!(
            Vec3 {
                d0: 50 as f64,
                d1: 100 as f64,
                d2: 150 as f64,
            },
            vec1 * vec2
        )
    }

    #[test]
    fn test_div() {
        let vec1 = Vec3 {
            d0: 50 as f64,
            d1: 50 as f64,
            d2: 50 as f64,
        };
        let vec2 = Vec3 {
            d0: 2 as f64,
            d1: 5 as f64,
            d2: 10 as f64,
        };
        assert_eq!(
            Vec3 {
                d0: 25 as f64,
                d1: 10 as f64,
                d2: 5 as f64,
            },
            vec1 / vec2
        )
    }

    #[test]
    fn test_mul_scal() {
        let vec1 = Vec3 {
            d0: 24.8,
            d1: 24.8,
            d2: 24.8,
        };
        let f = 2 as f64;
        assert_eq!(
            Vec3 {
                d0: 49.6,
                d1: 49.6,
                d2: 49.6,
            },
            vec1 * f
        )
    }

    #[test]
    fn test_div_scal() {
        let vec1 = Vec3 {
            d0: 24.8,
            d1: 24.8,
            d2: 24.8,
        };
        let f = 2 as f64;
        assert_eq!(
            Vec3 {
                d0: 12.4,
                d1: 12.4,
                d2: 12.4,
            },
            vec1 / f
        )
    }

    #[test]
    fn test_dot() {
        let vec1 = Vec3 {
            d0: 1.0,
            d1: 2.0,
            d2: 3.0,
        };
        let vec2 = Vec3 {
            d0: 2.5,
            d1: 1.5,
            d2: 3.0,
        };
        assert_eq!(14.5, Vec3::dot(&vec1, &vec2))
    }

    #[test]
    fn test_cross() {
        let vec1 = Vec3 {
            d0: 2.0,
            d1: 3.0,
            d2: 4.0,
        };
        let vec2 = Vec3 {
            d0: 5.0,
            d1: 6.0,
            d2: 7.0,
        };
        assert_eq!(
            Vec3 {
                d0: -3.0,
                d1: 6.0,
                d2: -3.0,
            },
            Vec3::cross(&vec1, &vec2)
        )
    }

    #[test]
    fn test_mul_assign() {
        let mut vec1 = Vec3 {
            d0: 2.0,
            d1: 3.0,
            d2: 4.0,
        };
        let f: f64 = 2.0;
        vec1 *= f;
        assert_eq!(
            Vec3 {
                d0: 4.0,
                d1: 6.0,
                d2: 8.0,
            },
            vec1
        )
    }

    #[test]
    fn test_div_assign() {
        let mut vec1 = Vec3 {
            d0: 2.0,
            d1: 3.0,
            d2: 4.0,
        };
        let f: f64 = 2.0;
        vec1 /= f;
        assert_eq!(
            Vec3 {
                d0: 1.0,
                d1: 1.5,
                d2: 2.0,
            },
            vec1
        )
    }

    #[test]
    fn test_add_assign() {
        let mut vec1 = Vec3 {
            d0: 2.0,
            d1: 3.0,
            d2: 4.0,
        };
        let vec2 = Vec3 {
            d0: 5.0,
            d1: 2.1,
            d2: 8.1,
        };
        vec1 += vec2;
        assert_eq!(
            Vec3 {
                d0: 7.0,
                d1: 5.1,
                d2: 12.1,
            },
            vec1
        )
    }

    #[test]
    fn test_neg() {
        let vec = Vec3 {
            d0: 23.2,
            d1: 1.0,
            d2: 6.4,
        };
        assert_eq!(
            Vec3 {
                d0: -23.2,
                d1: -1.0,
                d2: -6.4,
            },
            -vec
        )
    }
}
