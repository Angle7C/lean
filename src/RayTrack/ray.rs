use rand::Rng;

use super::{vec3::{self, Vec3}, hit_record::{HittableList, Hittable}};
use crate::Raytrack::hit_record::HitRecord;
use crate::Object::sphere::Sphere;
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
    pub fn ray_color(&self,world :&HittableList)->Vec3{
        let mut rand=rand::thread_rng();
        if rand.gen_range(0.0..1.0)<0.1{
            return Vec3::new(0.0,0.0,0.0);
        }
        
        match world.hit(&self, 0.0001, 1000.0){
            Some(v)=>{
                let target=v.p+v.normal+Sphere::rand_unit_sphere();
                if let v = Ray::new(v.p, target-v.p).ray_color(world){
                    return v*0.5;
                }else{
                    let t=0.5*(self.dir.normalized().y+1.0);
                    Vec3::new(1.0, 1.0, 1.0)*(1.0-t)+Vec3::new(0.4, 0.4, 1.0)*t
                }
            }
            None=>{
                let t=0.5*(self.dir.normalized().y+1.0);
                Vec3::new(1.0, 1.0, 1.0)*(1.0-t)+Vec3::new(0.4, 0.4, 1.0)*t
            
            }    
        }
        }
}