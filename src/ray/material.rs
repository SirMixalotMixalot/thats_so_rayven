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
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1. {
                fuzz
            }else {
                1.
            }
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record : HitRecord,) -> Option<(Color,Ray)> {
        let reflected = Vec3::reflect(&ray.direction().normalized(), &record.normal);
        let scattered = Ray::new(record.p, reflected + Vec3::random_in_unit() * self.fuzz);
        if scattered.direction().dot(&record.normal).is_sign_positive() {
            Some((self.albedo, scattered))
        }else {
            None
        }
    }
}
#[derive(Debug)]
pub struct Dielectric {
    pub index_of_refraction: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, record : HitRecord,) -> Option<(Color,Ray)> {
        let refraction_ratio = if record.front_face {
            1./self.index_of_refraction
        }else {
            self.index_of_refraction
        };
        let unit_direction = ray.direction().normalized();
        let cos_theta = (-unit_direction).dot(&record.normal).min(1.);
        let sin_theta = (1. - cos_theta.powi(2)).sqrt();

        let can_refract = refraction_ratio * sin_theta <= 1.;
        let direction = if can_refract {
            Vec3::refract(&unit_direction, &record.normal, refraction_ratio)
        }else {
            Vec3::reflect(&unit_direction, &record.normal)
        };

        let scattered = Ray::new(record.p, direction);

        Some(((1.,1.,1.).into(), scattered))
    }
}