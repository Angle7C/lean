
use super::{vec3::{self, Vec3}, hit_record::{ Hittable}};
use crate::raytrack::hit_record::HitRecord;
use crate::object::sphere::Sphere;
pub struct Ray{
    pub orig:vec3::Point3,
    pub dir :vec3::Vec3,
}
impl Ray{
    pub fn new(origin :vec3::Point3,direction:vec3::Vec3)->Ray{
        Ray { orig: (origin), dir: (direction.normalized()) }
    }
    pub fn at(&self,t :f64)->Vec3{
        self.orig+self.dir*t
    }
    pub fn path_color(&self,rec :Option<HitRecord>,bvh :&Box<dyn Hittable>,dep :i32)->Vec3{
        if dep<=0 {
            return Vec3::zero()
        }
        if let Some(v)=rec{
                let s=v.p+v.normal+Sphere::rand_unit_sphere();
                let r = Ray::new(v.p, v.p-s);
                match bvh.hit(&r, 0.0000, 10000.0){
                    Some(v)=> r.path_color(Some(v), bvh,dep-1),
                    None=>{
                        let t=0.5*(r.dir.y+1.0);
                        Vec3::new(0.78, 1.0, 1.0)*(1.0-t)+Vec3::new(0.4, 0.7, 1.0)*t
                    }
                }
        }
        else{
            let t=0.5*(self.dir.y+1.0);
            Vec3::new(0.78, 1.0, 1.0)*(1.0-t)+Vec3::new(0.4, 0.7, 1.0)*t
        }
    }
   
}