
use super::{vec3::Vec3, ray::Ray, aabb::AABB};

#[derive(Debug)]
pub struct HitRecord{
    pub normal :Vec3,
    pub p  :Vec3,
    pub t  :f64,
    pub is_face: bool,
    pub is_light:bool,
}
pub trait Hittable {
     fn hit(&self,r :&Ray,t_min :f64, t_max:f64)->Option<HitRecord>;
     fn boundBox(&self)->Option<AABB>;
     

}
impl HitRecord{
    pub fn new(normal :Vec3,p :Vec3, t:f64, is_face:bool,is_light:bool)->Self{
        HitRecord{ normal: (normal), p: (p), t: (t), is_face,is_light}
    }
    pub fn set_face_norm(&mut self,r :&Ray){
        match  r.dir*self.normal>0.0{
            true=>  self.normal=self.normal,
            false=> self.normal=-self.normal,
        };
    }
}
