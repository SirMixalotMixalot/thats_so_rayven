use crate::{ray::Ray, vec3::Vec3};
type Point = Vec3;
pub struct Camera {
    origin : Point,
    lower_left_corner : Point,
    horizontal : Vec3,
    vertical : Vec3,

    
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio : f64 = 16. / 9. ;

        let view_port_height = 2.;
        let view_port_width = aspect_ratio * view_port_height;
        let focal_length = 1.;

        let origin : Point = (0.,0.,0.).into();
        let horizontal : Vec3 = (view_port_width,0.,0.).into();
        let vertical : Vec3 = (0.,view_port_height,0.).into();
        let lower_left_corner : Point = origin - (horizontal/2.).unwrap()
            - (vertical/2.).unwrap() -
            (0.,0.,focal_length).into();
        Self {origin,lower_left_corner,horizontal,vertical}

    }
    pub fn get_ray(&self, u : f64, v : f64) -> Ray {
        let dir = self.lower_left_corner + self.horizontal * u + self.vertical * v
            - self.origin;
        Ray::new(self.origin,dir)
    }

}
