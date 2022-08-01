use crate::random::*;
use std::ops::*;

#[macro_export]
macro_rules! vec3 {
    () => {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    };
    ($s:expr) => {
        Vec3 {
            x: $s as f32,
            y: $s as f32,
            z: $s as f32,
        }
    };
    ($x:expr, $y:expr, $z:expr) => {
        Vec3 {
            x: $x as f32,
            y: $y as f32,
            z: $z as f32,
        }
    };
}

#[macro_export]
macro_rules! color {
    () => {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    };
    ($s:expr) => {
        Vec3 {
            x: $s as f32,
            y: $s as f32,
            z: $s as f32,
        }
    };
    ($x:expr, $y:expr, $z:expr) => {
        Vec3 {
            x: $x as f32,
            y: $y as f32,
            z: $z as f32,
        }
    };
}

pub(crate) use color;
pub(crate) use vec3;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type Color = Vec3;

impl Vec3 {
    pub const ZERO: Self = vec3!(0, 0, 0);

    #[inline(always)]
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    #[inline(always)]
    pub fn lensq(self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    #[inline(always)]
    pub fn len(self) -> f32 {
        self.lensq().sqrt()
    }

    #[inline(always)]
    pub fn normalized(self) -> Vec3 {
        self * (1.0 / self.len())
    }

    #[inline(always)]
    pub fn dot(self, other: Vec3) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    #[inline(always)]
    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    #[inline(always)]
    pub fn lerp(a: Vec3, t: f32, b: Vec3) -> Vec3 {
        (1.0 - t) * a + t * b
    }

    #[inline(always)]
    pub fn hadamard(a: Vec3, b: Vec3) -> Vec3 {
        Vec3 {
            x: a.x * b.x,
            y: a.y * b.y,
            z: a.z * b.z,
        }
    }

    #[inline(always)]
    pub fn random() -> Vec3 {
        vec3!(randomf32(), randomf32(), randomf32())
    }

    #[inline(always)]
    pub fn random_range(min: f32, max: f32) -> Vec3 {
        vec3!(
            randomf32_range(min, max),
            randomf32_range(min, max),
            randomf32_range(min, max)
        )
    }

    #[inline(always)]
    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = vec3!(randomf32_range(-1.0, 1.0), randomf32_range(-1.0, 1.0), 0.0);
            if p.lensq() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    #[inline(always)]
    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.lensq() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    #[inline(always)]
    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().normalized()
    }

    #[inline(always)]
    pub fn near_zero(self) -> bool {
        let s = 1e-8f32;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    #[inline(always)]
    pub fn reflect(self, other: Vec3) -> Vec3 {
        self - 2.0 * self.dot(other) * other
    }

    #[inline(always)]
    pub fn refract(self, n: Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = f32::min((-self).dot(n), 1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -f32::sqrt(f32::abs(1.0 - r_out_perp.lensq())) * n;
        r_out_perp + r_out_parallel
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<Vec3> for f32 {
    type Output = Vec3;

    #[inline(always)]
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self + other.x,
            y: self + other.y,
            z: self + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<Vec3> for f32 {
    type Output = Vec3;

    #[inline(always)]
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self - other.x,
            y: self - other.y,
            z: self - other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn div(self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl Div<Vec3> for f32 {
    type Output = Vec3;

    #[inline(always)]
    fn div(self, other: Vec3) -> Vec3 {
        let s = 1.0 / self;
        Vec3 {
            x: s * other.x,
            y: s * other.y,
            z: s * other.z,
        }
    }
}

impl AddAssign for Vec3 {
    #[inline(always)]
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl SubAssign for Vec3 {
    #[inline(always)]
    fn sub_assign(&mut self, other: Vec3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline(always)]
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl DivAssign<f32> for Vec3 {
    #[inline(always)]
    fn div_assign(&mut self, scalar: f32) {
        let s = 1.0 / scalar;
        self.x *= s;
        self.y *= s;
        self.z *= s;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl PartialEq for Vec3 {
    #[inline(always)]
    fn eq(&self, other: &Vec3) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }

    #[inline(always)]
    fn ne(&self, other: &Vec3) -> bool {
        self.x != other.x || self.y != other.y || self.z != other.z
    }
}
