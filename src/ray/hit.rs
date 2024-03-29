use std::rc::Rc;
use crate::ray::{Ray,Point};
use crate::vec3::Vec3;

use super::material::Material;
#[derive(Clone)]
pub struct HitRecord {
   pub p : Point,
   pub normal : Vec3,
   pub t : f64,
   pub front_face : bool,
   pub material : Rc<dyn Material>,
}

impl HitRecord {
    
    pub fn new(p : Point, normal : Vec3, t : f64, dir : &Vec3, material: Rc<dyn Material>) -> Self {
        let d = normal.dot(dir);
        let front_face = d < 0.;
        let normal = if front_face { normal } else { -normal };
        Self {p,normal,t,front_face, material}
    }
}

pub trait Hittable {
    fn hit(&self,ray : &Ray, t_min : f64, t_max : f64) -> Option<HitRecord>;
}


pub type HittableList = Vec<Rc<dyn Hittable>>;

impl Hittable for HittableList {

    fn hit(&self, ray : &Ray, t_min : f64, t_max : f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut record = None;
        for obj in self {
            if let Some(hr) = obj.hit(ray,t_min,closest_so_far) {
                closest_so_far = hr.t;
                record = Some(hr);
            }
        }
        
        record

    }

}
