use std::ops::{Add,Mul,Neg,Sub,Div,Index,IndexMut};
use std::fmt;
pub mod color;
use std::error::Error;

//====================Error Types==================
#[derive(Debug)]
pub struct DivisionByZeroError;

impl fmt::Display for DivisionByZeroError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"Cannot divide vector by 0")

    }
}
impl Error for DivisionByZeroError {}
//==================================================
#[derive(Debug,Clone,Copy)]
pub struct Vec3 {
    coords : [f64;3],
}

impl Vec3 {
    pub fn new() -> Self {
        Vec3 {coords : [0.0,0.0,0.0]}
    }
    pub fn with_points(x : f64, y : f64, z : f64) -> Self {
        Vec3 {coords : [x,y,z]}
    }
    pub fn x(&self) -> f64 {self.coords[0]} 
    pub fn y(&self) -> f64 {self.coords[1]}
    pub fn z(&self) -> f64 {self.coords[2]}

    pub fn cross(&self, other : &Vec3) -> Self {
        let x = self.y() * other.z() - self.z() * other.y();
        let y = self.z() * other.x() - self.x() * other.z();
        let z = self.x() * other.y() - self.y() * other.x();
        Vec3::with_points(x, y, z)
    }
    pub fn dot(&self, rhs: &Vec3) ->f64  {
        self.coords.iter().zip(rhs.coords.iter())
            .fold(0., |a,(&p1,&p2)| a + p1*p2)

    }
    pub fn len_squared(&self) -> f64 {
        self.dot(&self)
    }
    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }
    pub fn normalized(&self) -> Result<Self,DivisionByZeroError> {
        self / self.len()
    }
}
//=====================Ops=====================
impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::with_points(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}
impl Add for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &Vec3) -> Self::Output {
        (self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]).into()
    }
}
impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3::with_points(-self.x(),-self.y(), -self.z())
    }
}
impl Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        self * -1.
    }
}



impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, scaler: f64) -> Self::Output {
        Vec3::with_points( self.x() * scaler, self.y() * scaler, self.z() * scaler)
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, scaler: f64) -> Self::Output {
        Vec3::with_points( self.x() * scaler, self.y() * scaler, self.z() * scaler)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs : Vec3) -> Self::Output {
        self + (-rhs)
    }
}
impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs : &Vec3) -> Self::Output {
        self + &(-rhs)
    }
}
impl Div<f64> for Vec3 {
    type Output = Result<Self,DivisionByZeroError>;
    fn div(self, rhs:f64) -> Self::Output {
        if rhs == 0. {
            Err(DivisionByZeroError) 
        }
        else {Ok(self * (1./rhs))}
    }
}

impl Div<f64> for &Vec3 {
    type Output = Result<Vec3,DivisionByZeroError>;
    fn div(self, rhs:f64) -> Self::Output {
        if rhs == 0. {
            Err(DivisionByZeroError)
        }else {
            Ok(self * (1./rhs))
        }
    }
}
impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.coords[index]
    }
}
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.coords[index]
    }
}
//=================Utility=========================

impl From<[f64;3]> for Vec3 {
    fn from(coords : [f64;3]) ->Self {
        Vec3 {coords}
    }
}
impl From<(f64,f64,f64)> for Vec3 {
    fn from((x,y,z): (f64,f64,f64) ) -> Self {
        Vec3::with_points(x, y, z)
    }
}
impl Into<(f64,f64,f64)> for Vec3 {
    fn into(self) -> (f64,f64,f64) {
        (self.x(),self.y(),self.z())
    }
}
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"[{},{},{}]",self.x(),self.y(),self.z())
    }
}
