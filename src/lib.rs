pub mod vec3;
pub mod camera;
pub mod ray;
pub mod shapes;
#[cfg(test)]
mod tests {
    use vec3::Vec3;

    use super::*;
    
    #[test]
    fn test_vec_add() {
        let v = vec3::Vec3::with_points(1.,2.,3.);
        let v2 = vec3::Vec3::with_points(1.,2.,3.);
        let v3 = v + v2;
        assert_eq!(v3.x(),2.);
        assert_eq!(v3.y(),4.);
        assert_eq!(v3.z(),6.);

    }
    #[test]
    fn test_vec_neg() {
        let (x,y,z) = (1.,2.,3.);
        let v1 = vec3::Vec3::with_points(x, y, z);
        let v1 = -v1;
        assert_eq!(v1.x(),-x);
        assert_eq!(v1.y(),-y);
        assert_eq!(v1.z(),-z);

        
    }
    #[test]
    fn test_vec_mul_vec() {
        let (x,y,z) = (1.,2.,3.);
        let v1 = vec3::Vec3::with_points(x, y, z);
        let v2 = v1.clone();
        let dp = v1.dot(&v2);
        assert_eq!(dp,x*x + y*y + z*z);
    }
    #[test]
    fn test_vec_mul_f64() {
        let (x,y,z) = (1.,2.,3.);
        let v1 = vec3::Vec3::with_points(x, y, z);
        let v2 : Vec3 = v1 * 2.;
        assert_eq!(v2.x(), 2. * x);
        assert_eq!(v2.y(), 2.* y);
        assert_eq!(v2.z(), 2. * z);
    }
    #[test]
    fn test_vec_sub() {
        let (x,y,z) = (1.,2.,3.);
        let v1 = Vec3::with_points(x, y, z);
        let v2 = v1.clone();
        let v3 = v1 - v2;
        assert_eq!(v3.x(), 0.);
        assert_eq!(v3.y(), 0.);
        assert_eq!(v3.z(), 0.);
    }
    #[test]
    fn test_vec_div() {
         let (x,y,z) = (1.,2.,3.);
         let v1 = Vec3::with_points(x, y, z);
         let v2 = (v1 / 2.).unwrap();
         assert_eq!(v2.x(), 1./2.);
         assert_eq!(v2.y(), 2./2.);
         assert_eq!(v2.z(), 3./2.);

    }
    #[test]
    #[should_panic(expected = "Cannot divide vector by 0")]
    fn test_div_zero() {
     let (x,y,z) = (1.,2.,3.);
     let v1 = Vec3::with_points(x, y, z);
     let _v1 = match v1/0. {
        Err(e) => panic!("{}",e),
        Ok(v)  => v,
     };

    }
    #[test]
    fn test_vec_cross() {
        let i = Vec3::with_points(1., 0., 0.);
        let j = Vec3::with_points(0., 1., 0.);
        let k = i.cross(&j);
        assert_eq!(k.x(),0.);
        assert_eq!(k.y(),0.);
        assert_eq!(k.z(),1.);
    }
    #[test]
    fn vec_index_ref() {
        let i = Vec3::with_points(2., 0., 1.);
        assert_eq!(i[0],2.);
        assert_eq!(i[1],0.);
        assert_eq!(i[2],1.);
    }
    #[test]
    fn vec_index_mut_ref() {

        let mut i = Vec3::with_points(2., 0., 1.);
        let x = &mut i[0];
        *x = 4.;
        assert_eq!(i.x(),4.);
    }
    #[test]
    fn vec_from_arr() {
        let _i : Vec3 = [1.,2.,3.].into();

    }
}
