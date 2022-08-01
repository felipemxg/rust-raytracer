use crate::material::*;
use crate::ray::*;
use crate::vec3::*;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Material,
    pub t: f32,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

impl HitRecord {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            p: vec3!(0.0),
            normal: vec3!(0.0),
            t: 0.0,
            front_face: false,
            material: Material::new(MaterialType::Lambertian, color!(), 0.0),
        }
    }

    #[inline(always)]
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
