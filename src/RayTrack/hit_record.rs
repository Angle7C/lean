use rand::Rng;

use crate::Object::sphere::Sphere;

use super::{vec3::Vec3, ray::Ray};
pub struct HitRecord{
    pub normal :Vec3,
    pub p  :Vec3,
    pub t  :f64,
    pub is_face: bool,
}

pub trait Hittable {
     fn hit(&self,r :&Ray,t_min :f64, t_max:f64)->Option<HitRecord>;
}
impl HitRecord{

    pub fn new(normal :Vec3,p :Vec3, t:f64, is_face:bool)->Self{
        HitRecord{ normal: (normal), p: (p), t: (t), is_face}
    }
    pub fn set_face_norm(&mut self,r :&Ray){
        match  r.dir*self.normal>0.0{
            true=>  self.normal=self.normal,
            false=> self.normal=-self.normal,
        };
    }
}
pub struct HittableList{
     list :Vec<Box<dyn Hittable>>,
}
impl HittableList{
    pub fn new()->Self{
        HittableList{list:vec![]}
    }
    pub fn add(&mut self,value :Box<dyn Hittable>){
        self.list.push(value)
    }
    pub fn clear(&mut self){
        self.list.clear();
    }
}
impl Hittable for HittableList{
    fn hit(&self,r :&Ray,t_min :f64, t_max:f64)->Option<HitRecord> {
        let mut rand= rand::thread_rng();
        
        let mut colse_far_t=t_max;
        let mut ans:Option<HitRecord>=None;
        let mut temp:Option<HitRecord>=None;
        for item in &self.list{
            match item.hit(r, t_min, colse_far_t) {
                Some(v)=>{
                    colse_far_t=v.t;
                    temp=Some(v)
                },
                _=> temp=None
            };
            ans=match temp {
                Some(v)=>Some(v),
                None=>ans,
            }
        };
        ans
    }
}