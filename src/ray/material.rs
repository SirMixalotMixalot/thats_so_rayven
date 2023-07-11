use std::fmt::Debug;

use crate::camera::Vec3;
use crate::vec3::color::Color;

use super::hit::HitRecord;
use super::Ray;

pub trait Material : Debug {
    fn scatter(&self, ray: &Ray, record : HitRecord,) -> Option<(Color,Ray)>;
}
#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Color
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, record : HitRecord) -> Option<(Color, Ray)> {
        let mut scatter = record.normal + Vec3::random_in_hemisphere(record.normal);

        if scatter.is_near_zero() {
            scatter = record.normal;
        }

        Some(
            (self.albedo, Ray::new(record.p, scatter))
        )

    }
}

#[derive(Debug)]
pub struct Metal {
    pub albedo: Color
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record : HitRecord,) -> Option<(Color,Ray)> {
        let reflected = Vec3::reflect(&ray.direction().normalized(), &record.normal);
        let scattered = Ray::new(record.p, reflected);
        if scattered.direction().dot(&record.normal).is_sign_positive() {
            Some((self.albedo, scattered))
        }else {
            None
        }
    }
}