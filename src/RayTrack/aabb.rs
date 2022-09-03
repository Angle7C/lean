use super::vec3::Vec3;
use super::ray::Ray;
use std::mem;
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct AABB {
    pub min :Vec3,
    pub max :Vec3,
}
impl AABB{
    pub fn new(start :Vec3,end :Vec3)->Self{
        AABB{min:start,max:end}
    }
    pub fn merage(&self,other :&AABB)->Self{
        let min= Vec3::new(match self.min.x<other.min.x{
            true=>self.min.x,
            false=>other.min.x,
        },match self.min.y<other.min.y{
            true=>self.min.y,
            false=>other.min.y,
        },match self.min.z<other.min.z{
            true=>self.min.z,
            false=>other.min.z,
        });
        let max= Vec3::new(match self.max.x>other.max.x{
            true=>self.max.x,
            false=>other.max.x,
        },match self.max.y>other.max.y{
            true=>self.max.y,
            false=>other.max.y,
        },match self.max.z>other.max.z{
            true=>self.max.z,
            false=>other.max.z,
        });
        AABB::new(min,max)
    }
    pub fn hit(&self,ray :&Ray,t_min: f64,t_max: f64)->bool{
        let mut tmin=t_min;
        let mut tmax=t_max;
        for a in 0..3{
             let inv=1.0/ray.dir[a];
             let mut t0=(self.min[a]-ray.orig[a])*inv;
             let mut t1=(self.max[a]-ray.orig[a])*inv;
             if inv<0.0{
                mem::swap(&mut t0,&mut t1)
             }
             tmin= if t0>tmin {t0} else {tmin};
             tmax= if t1<tmax {t1} else {tmax};
             if tmax<=tmin{
                return false;
             }
            };
            tmin>=0.0
    
    }
}