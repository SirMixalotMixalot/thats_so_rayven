use std::{fmt::Debug, rc::Rc};

use crate::ray::{Ray,Point};
use crate::vec3::Vec3;
#[derive(Debug)]
pub struct HitRecord {
   pub p : Point,
   pub normal : Vec3,
   pub t : f64,
   pub front_face : bool,
}

impl HitRecord {
    
    pub fn new(p : Point, normal : Vec3, t : f64, dir : &Vec3) -> Self {
        let d = normal.dot(dir);
        let front_face = d < 0.;
        let normal = if front_face { normal} else { -normal};
        Self {p,normal,t,front_face}
    }
}

pub trait Hittable : Debug {
    fn hit(&self,ray : &Ray, t_min : f64, t_max : f64) -> Option<HitRecord>;
}


pub type HittableList = Vec<Rc<dyn Hittable>>;

impl Hittable for HittableList {

    fn hit(&self, ray : &Ray, t_min : f64, t_max : f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut rec = None;
        for obj in self {
            if let Some(hr) = obj.hit(ray,t_min,closest_so_far) {
                closest_so_far = hr.t;
                rec = Some(hr);
            }
        }
        
        rec

    }

}
