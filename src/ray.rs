use crate::vec3::Vec3;
use self::hit::{HitRecord, Hittable};
pub mod hit;
pub type Point = Vec3;
pub struct Ray {
    origin : Point,
    direction : Vec3,
}

impl Ray {
    pub fn new(origin : Point, direction : Vec3) -> Self {
        Self {origin,direction}
    }
    pub fn origin(&self) -> Point {self.origin}
    pub fn direction(&self) -> Vec3 {self.direction}
    pub fn at(&self, t : f64) -> Point {
        self.origin + (self.direction * t)
    }
    
    pub fn hit_obj(&self, obj : &dyn Hittable) -> Option<HitRecord> {
        obj.hit(&self,0.,f64::INFINITY)
    }
}
