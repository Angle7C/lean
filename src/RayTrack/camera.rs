use super::{
    hit_record::Hittable,
    ray::Ray,
    vec3::{self, Vec3},
};
use nalgebra::{Matrix4, RowVector4, Vector4, Vector3};
use std::f64::consts::PI;
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner :Vec3,
    pub horizontal :Vec3,
    pub vertical :Vec3,

}
impl Camera {
    pub fn new(eye :Vec3,lookat :Vec3,up :Vec3,fov: f64, aspect: f64)->Self{
        let z=(eye-lookat).normalized();
        let x=up.cross(&z).normalized();
        let y=z.cross(&x).normalized();
        let ahpla=(fov/2.0*PI/180.0);
        let h=ahpla.tan();
        let w=aspect*h;
        let lower=eye-x*w-y*h-z;
        let hor=x*w*2.0;
        let ver=y*h*2.0;
        Self{
            origin:eye,
            lower_left_corner:lower,
            horizontal:hor,
            vertical:ver
        }
    }
    pub fn getRay(&self, u:f64,v :f64)->Ray{
        Ray::new(self.origin,self.lower_left_corner+self.horizontal*u+self.vertical*v-self.origin)
    }
}
