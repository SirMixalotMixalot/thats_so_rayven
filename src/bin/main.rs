
use std::io::Write;
use std::rc::Rc;
use thats_so_rayven::shapes::Sphere;
use thats_so_rayven::camera::Camera;
use thats_so_rayven::vec3::color;
use color::{Color, ray_color};
use thats_so_rayven::ray::hit::HittableList;
use rand;
fn main() {
    //Image 
    const ASPECT_RATIO : f64 = 16. / 9. ;
    const IM_WIDTH : i32 = 400;
    const IM_HEIGHT : i32 = (IM_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL : usize = 100usize;

    let cam = Camera::new();

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new((0.,0.,-1.).into(),0.5)));
    world.add(Rc::new(Sphere::new((0.,-100.5,-1.).into(),100.)));
    print!("P3\n{} {}\n255\n",IM_WIDTH,IM_HEIGHT);
    
    for j in (0..IM_HEIGHT).rev(){
       eprint!("\rScanlines remaining : {}",j);
       std::io::stderr().flush().unwrap();

       for i in 0..IM_WIDTH{
           let mut color = Color::new();
           for _ in 0..SAMPLES_PER_PIXEL {
            let (u,v) =
            ( 
                    (i as f64 + rand::random::<f64>())  / (IM_WIDTH-1) as f64,
                    (j as f64 + rand::random::<f64>())/ (IM_HEIGHT-1) as f64,
            );
            let r = cam.get_ray(u,v);
            color = color + ray_color(r,&world); 
     
           }
           color::write_color(&mut std::io::stdout(), &color,SAMPLES_PER_PIXEL).unwrap()
       }
    }
    eprintln!("\nDone...");



}
