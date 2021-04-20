use crate::{ray::hit::Hittable, vec3};
use crate::ray::Ray;
use std::io;
pub type Color = vec3::Vec3;
pub fn write_color(f :&mut dyn io::Write, color : &Color, samples_per_pixel : usize) -> io::Result<()> 
{   
    let scale = 1./ samples_per_pixel as f64;
    let c = color * scale;
    let s = 255.999;
    let (min,max) = (0.,0.999);
    write!(f,"{} {} {}\n",(s * f64::clamp(c.x(),min,max)) as i32,
                          (s * f64::clamp(c.y(),min,max)) as i32,
                          (s * f64::clamp(c.z(),min,max)) as i32)

}
pub fn ray_color(ray : Ray,hittable : &dyn Hittable) -> Color {
    
    if let Some(hr) = ray.hit_obj(hittable) {
       
       return (Color::with_points( hr.normal[0] , hr.normal[1] ,hr.normal[2] ) + (1.,1.,1.).into()) * 0.5; 
    }
       
    let unit_dir = ray.direction().normalized().unwrap();
    let t = 0.5 * (unit_dir.y() + 1.);
    let c1 : Color = (1.,1.,1.).into() ;
    let c2 : Color = (0.5,0.7,1.0).into() ;
    c1 * (1. - t) + c2 * t 



}

