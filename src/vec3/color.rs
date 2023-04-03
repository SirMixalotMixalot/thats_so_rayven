use crate::{ray::hit::Hittable, vec3,ray::Ray};
use std::io;

use super::Vec3;
pub type Color = vec3::Vec3;
pub fn write_color(f :&mut dyn io::Write, color : &Color, samples_per_pixel : usize) -> io::Result<()> 
{   
    let scale = 1./ samples_per_pixel as f64;
    let c = color * scale;
    let (r,g, b) = c.into();
    
    let s = 256.;
    
    let (min,max) = (0.,0.999);
    write!(f,"{} {} {}\n",(s * f64::clamp(r.sqrt(),min,max)) as i32,
                          (s * f64::clamp(g.sqrt(),min,max)) as i32,
                          (s * f64::clamp(b.sqrt(),min,max)) as i32)

}
fn random_in_unit() -> Vec3 {
    loop {
        let p = Vec3::rand_with_range(-1.,1.);
        if p.len_squared() - 1. <= f64::EPSILON  {
            break p;
        }

    }
}
pub fn ray_color(ray : Ray,object : &dyn Hittable, depth: u32) -> Color {
    if depth <= 0 {
        return Color::new();
    }
    if let Some(hr) = ray.hit_obj(object) {
       let target = hr.p + hr.normal + random_in_unit();

       return ray_color(Ray::new(hr.p, target - hr.p), object, depth - 1) * 0.5; 
    }
       
    let unit_dir = ray.direction().normalized().unwrap();
    let t = 0.5 * (unit_dir.y() + 1.);
    let c1 : Color = (1.,1.,1.).into() ;
    let c2 : Color = (0.5,0.7,1.0).into() ;
    c1 * (1. - t) + c2 * t 



}

