
use nalgebra::{Vector3,Point3, Point};

use super::ray::Ray;
#[derive(Debug,Clone, Copy)]
pub struct AABB{
    pub min :Vector3<f32>,
    pub max :Vector3<f32>,

}
#[derive(Debug)]
pub struct HitRecord{
    pub normal :Vector3<f32>,
    pub p      :Vector3<f32>,
    pub t      :f32,
}
impl HitRecord {
    #[allow(dead_code)]
    pub fn new(normal :Vector3<f32>,p :Vector3<f32>,t: f32)->Self{
        HitRecord { normal: (normal), p: (p), t: (t) }
    }
    #[inline]
    pub fn set_normal(&mut self,ray :&Ray){
        if self.normal.dot(&ray.direction)>0.0{
            self.normal=-self.normal;
        }
    }
}
pub trait BoundBox{
    fn bound_box(&self)->AABB;
    fn hit(&self,ray:&Ray,t_min :f32,t_max:f32)->Option<HitRecord>;
}
impl AABB{
    #[allow(dead_code)]
    pub fn new(min :Vector3<f32>,max:Vector3<f32>)->Self{
        Self { min: (min)-Vector3::new(0.02, 0.02,0.02), max: (max+Vector3::new(0.02, 0.02,0.02)) }
    }
    pub fn zero()->Self{
        Self { min: (Vector3::<f32>::zeros()), max: (Vector3::<f32>::zeros()) }
    }
    pub fn merge(&self,other :&AABB)->Self{
        let min=Vector3::<f32>::new(self.min.x.min(other.min.x),self.min.y.min(other.min.y),self.min.z.min(other.min.z));
        let max=Vector3::<f32>::new(self.max.x.max(other.max.x),self.max.y.max(other.max.y),self.max.z.max(other.max.z));
        Self{min,max}
    }
    #[allow(dead_code)]
    pub fn area(&self)->f32{
        let det=self.max-self.min;
        det.x*det.y*2.0+det.x*det.z*2.0+det.y*det.z*2.0
    }
    pub fn hit(&self,ray :&Ray,t_min :f32,t_max:f32)->bool {
        let mut tmin=t_min;
        let mut tmax=t_max;
        let mut t0=0.0;
        let mut t1=0.0;
        for i in 0..3{
            let inv=1.0/ray.direction[i];
            t0=(self.min[i]-ray.origin[i])*inv;
            t1=(self.max[i]-ray.origin[i])*inv;
            if inv<0.0{
                std::mem::swap(&mut t0,&mut t1);
            }
            tmin = t0.min(tmin);
            tmax = t1.max(tmax);
            if tmax<tmin||t0<0.0||t1<0.0{
                return false
            }
        }
        return true;
    }
}
