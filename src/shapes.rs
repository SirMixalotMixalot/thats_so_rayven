use std::rc::Rc;

use crate::ray::Ray;
use crate::ray::Point;
use crate::ray::hit::{HitRecord,Hittable};
use crate::ray::material::Material;

#[derive(Debug)]
pub struct Sphere {
  pub  centre : Point,
  pub  radius : f64,
  pub material: Rc<dyn Material>,
}
impl Sphere {
    pub fn new(centre : Point, radius : f64, material: Rc<dyn Material>) -> Self {
        Self {centre,radius, material}
    }
}
impl Hittable for Sphere {
  fn hit(&self,ray : &Ray, t_min : f64, t_max : f64) -> Option<HitRecord> {
      let oc = ray.origin() - self.centre;
      let a = ray.direction().len_squared();
      let b =  oc.dot(&ray.direction());
      let c = oc.len_squared() - (self.radius * self.radius);
      let discriminant = (b*b) -  a * c;       
      if discriminant < 0. {
        return None;
      }
      let disc_sqrt = discriminant.sqrt();
      let root1 = (-b - disc_sqrt) / a;
      let root2 = (-b + disc_sqrt) / a;
      //eprintln!("root1 : {}, root2 : {}",root1,root2 );
      let range = t_min..=t_max;
      
      let root1_invalid = !range.contains(&root1);
      let root2_invalid = !range.contains(&root2);
/*       if !range.contains(&root1) {
        root = root2;
        if root < t_min || root > t_max {
            return None;
        }
     } */
     if root1_invalid && root2_invalid {
      return None;
     }
     let root = if root1_invalid {
         root2
     }else {
         root1
     };
      let t = root;
      let p = ray.at(t);
      let normal = ((p - self.centre)/self.radius).unwrap(); // Radius should not be zero
      Some(HitRecord::new(p, normal, t, &ray.direction(), self.material.clone(),) )
  }  
}
