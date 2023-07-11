use crate::{ray::hit::Hittable, vec3,ray::Ray};
use std::io;

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

pub fn ray_color(ray : Ray,object : &dyn Hittable, depth: u32) -> Color {
    if depth <= 0 {
        return Color::new();
    }
    if let Some(hr) = ray.hit_obj(object) {

       if let Some((attenuation, scattered)) = hr.material.scatter(&ray, hr.clone()) {
         return attenuation * ray_color(scattered, object, depth - 1); 

       }else {
        return Color::with_points(0.,0., 0.);
       }
    }
       
    let unit_dir = ray.direction().normalized();
    let t = 0.5 * (unit_dir.y() + 1.);
    let c1 : Color = (1.,1.,1.).into() ;
    let c2 : Color = (0.5,0.7,1.0).into() ;
    c1 * (1. - t) + c2 * t 



}

