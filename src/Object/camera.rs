use nalgebra::{Matrix4, Vector3, Point3};

use std::f32::consts::PI;

use crate::raytrack::ray::Ray;

pub struct Camera {
    pub origin: Vector3<f32>,
    pub lower_left_corner :Vector3<f32>,
    pub horizontal :Vector3<f32>,
    pub vertical :Vector3<f32>,
    pub view_matrix: Matrix4<f32>,
}
impl Camera {
    #[allow(dead_code)]
    pub fn new(eye :Vector3<f32>,target :Vector3<f32>,fov: f32, aspect: f32)->Self{
        let z=(eye-target).normalize();
        let x=Vector3::y().cross(&z).normalize();
        let y=z.cross(&x).normalize();
        let ahpla= fov/2.0*PI/180.0;
        let h=ahpla.tan();
        let w=aspect*h;
        let lower=eye-x*w-y*h-z;
        let hor=x*w*2.0;
        let ver=y*h*2.0;
        Self{
            origin:eye,
            lower_left_corner:lower,
            horizontal:hor,
            vertical:ver,
            view_matrix:Matrix4::<f32>::zeros()
        }
    }
    pub fn get_ray(&self,u:f32,v:f32)->Ray{
        Ray::new(self.origin, (self.lower_left_corner+u*self.horizontal+v*self.vertical-self.origin))
    }
}
