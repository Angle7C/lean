use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
pub type Point3 = Vec3;
pub type Color = Vec3;

///向量相加
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}
///向量相减

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }
}
///向量相乘

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, v: f64) -> Vec3 {
        Vec3 {
            x: self.x * v,
            y: self.y * v,
            z: self.z * v,
        }
    }
}
///向量相除
impl Mul<Vec3> for Vec3 {
    type Output = f64;
    fn mul(self, rhs: Vec3) -> Self::Output {
        self.dot(&rhs)
    }
}
impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, v: f64) -> Vec3 {
        Vec3 {
            x: self.x / v,
            y: self.y / v,
            z: self.z / v,
        }
    }
}
///向量求负

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
///向量取值（不可变引用）

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, idex: usize) -> &f64 {
        match idex {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vec3 out of range"),
        }
    }
}
///向量取值 （可变引用）

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, idx: usize) -> &mut f64 {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Vec3 out of range"),
        }
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.,
            y: 0.,
            z: 0.0,
        }
    }
    pub fn ones() -> Vec3 {
        Vec3 {
            x: 1.,
            y: 1.,
            z: 1.,
        }
    }
    pub fn dot(&self, v: &Vec3) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }
    pub fn rgba(&self, ssp: f64) -> image::Rgb<u8> {
        let sc = 1.0/ssp ;
        image::Rgb([
            (255.0 * clamp((self.x * sc).sqrt(), 0.0, 1.0)) as u8,
            (255.0 * clamp((self.y * sc).sqrt(), 0.0, 1.0)) as u8,
            (255.0 * clamp((self.z * sc).sqrt(), 0.0, 1.0)) as u8
            
        ])
    }
    pub fn length(&self) -> f64 {
        self.dot(self).sqrt()
    }
    pub fn length2(&self) -> f64 {
        self.dot(self)
    }
    pub fn normalized(&self) -> Vec3 {
        let len = self.length();
        self.div(len)
    }
    pub fn mul_elem(&self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z,
        }
    }
    pub fn sub_elem(&self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }
}
fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value > min && max > value {
        return value;
    } else if value<min {
        return min;
    } else {
        return max;
    }
}
#[cfg(test)]
mod Tests {
    use super::*;
    #[test]
    fn add() {
        let a = Vec3::new(1., 1., 1.);
        let b = Vec3::new(1., 1., 1.);
        let c = Vec3::new(2., 2., 2.);
        assert_eq!(a + b, c);
        println!("{:?}", a)
    }
}
