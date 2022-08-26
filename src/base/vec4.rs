use std::f64::{MAX as f64_MAX, MIN as f64_MIN};
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};
#[derive(Eq,Debug)]
pub struct Vec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
///向量相加
impl Add for Vec4 {
    type Output = Vec4;
    fn add(&self, v: &Vec4) -> Vec4 {
        Vec4 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
            w: self.w + v.w,
        }
    }
}
///向量相减

impl Sub for Vec4 {
    type Output = Vec4;
    fn sub(&self, v: &Vec4) -> Vec4 {
        Vec4 {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
            w: self.w - v.w,
        }
    }
}
///向量相乘

impl Mul<f64> for Vec4 {
    type Output = Vec4;
    fn mul(&self, v: f64) -> Vec4 {
        Vec4 {
            x: self.x * v,
            y: self.y * v,
            z: self.z * v,
            w: self.w * v,
        }
    }
}
///向量相除

impl Div<f64> for Vec4 {
    type Output = Vec4;
    fn div(&self, v: f64) -> Vec4 {
        Vec4 {
            x: self.x / v,
            y: self.y / v,
            z: self.z / v,
            w: self.w / v,
        }
    }
}
///向量求负

impl Neg for Vec4 {
    type Output = Vec4;
    fn neg(&self) -> Vec4 {
        Vec4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}
///向量取值（不可变引用）

impl Index<usize> for Vec4 {
    type Output = Vec4;
    fn index(&self, idex: usize) -> &f64 {
        match idex {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic("Vec4 out of range"),
        }
    }
}
///向量取值 （可变引用）

impl IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, idx: usize) -> &mut f64 {
        match idex {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic("Vec4 out of range"),
        }
    }
}

impl Vec4{
    pub fn new_point(x :f64,y:f64, z : f64)->Vec4{
        Vec4{x,y,z,1}
    }
    pub fn new_vector(x :f64,y:f64, z : f64)->Vec4{
        Vec4{x,y,z,0}
    }
    pub fn zero()->Vec4{
        Vec4{0,0,0,0}
    }
    pub fn ones()->Vec4{
        Vec4{1,1,1,1}
    }
    pub fn dot(&self,v: &Vec4)->f64{
        self.x*v.x+self.y*v.y+self.z*v.z
    }
    pub fn cross(&self,v :&Vec4){
        Vec4{
            x:self.y*v.z-self.z*v.y,
            y:self.z*v.x-self.x*v.z,
            z:self.x*v.y-self.y*v.x,
        }
    }
    pub fn rgba(&self)->image::Rgba<u8>{
        image::Rgba([
            (self.x*256) as u8,
            (self.y*256) as u8,
            (self.z*256) as u8,
             255  
        ])
    }
    pub fn length(&self)->f64{
        self.dot(self).sqrt()
    }
    pub fn length2(&self)->f64{
        self.dot(self)
    }
    pub fn normalized(&self)->Vec4{
        let len=self.length();
        self/len
    }
    pub fn mul_elem(&self,v: Vec4)->Vec4{
        Vec4{
            x:self.x*v.x,
            y:self.y*v.y,
            z:self.z*v.z,
            w:self.w*v.w,
        }
    }
    pub fn sub_elem(&self,v: Vec4)->Vec4{
        Vec4{
            x:self.x-v.x,
            y:self.y-v.y,
            z:self.z-v.z,
            w:self.w-v.w,
        }
    }
}

#[cfg(test)]
mod Tests{
    #[test]
    use super::*;
    fn add(){
      let a= Vec4::new_vector( 1,  1, 1);
      let b= Vec4::new_vector( 1,  1, 1);
      let c= Vec4::new_vector( 2,  2, 2);
      assert_eq!(a+b,c);
      println!("{:?}",a)
    }
}