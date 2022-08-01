use crate::hittable::*;
use crate::ray::*;
use crate::sphere::*;

pub struct HittableList {
    pub spheres: Vec<Sphere>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { spheres: vec![] }
    }

    pub fn clear(&mut self) {
        self.spheres.clear();
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }
}

impl Hittable for HittableList {
    #[inline(always)]
    fn hit(&self, r: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for sphere in &self.spheres {
            if sphere.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        hit_anything
    }
}
