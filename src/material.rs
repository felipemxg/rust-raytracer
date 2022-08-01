use crate::hittable::*;
use crate::random::randomf32;
use crate::ray::*;
use crate::vec3::*;

#[derive(Clone, Copy)]
pub enum MaterialType {
    Lambertian,
    Metal,
    Dieletric,
}

#[derive(Clone, Copy)]
pub struct Material {
    pub mat_type: MaterialType,
    pub albedo: Color,
    pub fuzz_ir: f32,
}

#[inline(always)]
fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * f32::powf(1.0 - cosine, 5.0)
}

impl Material {
    pub fn new(mat_type: MaterialType, albedo: Color, fuzz_ir: f32) -> Self {
        match mat_type {
            MaterialType::Metal => {
                let fuzz_ir = if fuzz_ir < 1.0 { fuzz_ir } else { 1.0 };
                Self {
                    mat_type,
                    albedo,
                    fuzz_ir,
                }
            }
            _ => Self {
                mat_type,
                albedo,
                fuzz_ir,
            },
        }
    }

    #[inline(always)]
    pub fn scatter(
        self,
        r_in: Ray,
        rec: HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match self.mat_type {
            MaterialType::Lambertian => {
                let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }

                *scattered = Ray::new(rec.p, scatter_direction);
                *attenuation = self.albedo;
                true
            }
            MaterialType::Metal => {
                let reflected = r_in.direction.normalized().reflect(rec.normal);
                *scattered = Ray::new(
                    rec.p,
                    reflected + self.fuzz_ir * Vec3::random_in_unit_sphere(),
                );
                *attenuation = self.albedo;
                scattered.direction.dot(rec.normal) > 0.0
            }
            MaterialType::Dieletric => {
                *attenuation = color!(1.0);
                let refraction_ratio = if rec.front_face {
                    1.0 / self.fuzz_ir
                } else {
                    self.fuzz_ir
                };

                let unit_direction = r_in.direction.normalized();
                let cos_theta = f32::min((-unit_direction).dot(rec.normal), 1.0);
                let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);

                let cannot_refract = (refraction_ratio * sin_theta) > 1.0;

                let direction =
                    if cannot_refract || reflectance(cos_theta, refraction_ratio) > randomf32() {
                        unit_direction.reflect(rec.normal)
                    } else {
                        unit_direction.refract(rec.normal, refraction_ratio)
                    };

                *scattered = Ray::new(rec.p, direction);
                true
            }
        }
    }
}
