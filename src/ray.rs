use crate::vec3::*;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    #[inline(always)]
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    #[inline(always)]
    pub fn at(self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}
