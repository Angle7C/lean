// use std::f64::consts::PI;
// use rand::Rng;
// use crate::object::traitable::Object;
// use crate::raytrack::ray::Ray;
// use crate::raytrack::hit_record::HitRecord;
// use crate::raytrack::vec3::{Vec3};
// use crate::raytrack::hit_record::Hittable;
// use crate::raytrack::aabb::AABB;
// #[derive(Debug)]
// pub struct Sphere{
//     pub center :Vec3,
//     pub radius :f64,
//     pub light:bool,
// }
// impl Hittable for Sphere{   
//     fn hit(&self,r :&Ray,t_min :f64, t_max:f64)->Option<HitRecord> {
//         let oc=r.orig-self.center;
//         let a=1 as f64;
//         let b  =oc.dot(&r.dir)*2.0;
//         let c=oc*oc-self.radius*self.radius;
//         let det=b*b-4.0*a*c;
//         match det>=0.0 {
//             true=>{
//                 let det=det.sqrt();
//                 let t=(-b-det)/(2.0*a);
//                 if t<t_min||t>t_max {
//                     let t=(-b+det)/(2.0*a);
//                     if t<t_min||t>t_max {
//                         None
//                     }else{
//                         let p=r.at(t);
//                         let mut rec=HitRecord::new((p-self.center).normalized(), p,t,false,self.light);
//                         rec.set_face_norm(&r);
//                         Some(rec)
                        
//                     }
//                 }else{
//                     let p=r.at(t);
//                     let mut rec=HitRecord::new((p-self.center).normalized(), p,t,false,self.light);
//                     rec.set_face_norm(&r);
//                     Some(rec)

//                 }
                
//             }
//             false=>None
//         }
        
//     }
//     fn boundBox(&self)->Option<AABB> {
//         let min=self.center-Vec3::new(self.radius,self.radius,self.radius);
//         let max=self.center+Vec3::new(self.radius,self.radius,self.radius);
//         Some(AABB::new(min, max))
//     }

// }
// impl Sphere{
//     pub fn rand_unit_sphere()->Vec3{
//         let mut rand = rand::thread_rng();
//         let alpha = rand.gen_range(0.0*PI..0.5*PI);
//         let cos_beta =rand.gen_range(0.0..1.0);
//         let sin_beta=(1.0-cos_beta*cos_beta) as f32;
//         let sin_beta=sin_beta.sqrt() as f64;
//         Vec3 { x: sin_beta*alpha.cos(), y: sin_beta*alpha.sin(), z: sin_beta }
//     }
//     pub fn new(center :Vec3,radius :f64)->Self{
//         Sphere{
//             center:(center),
//             radius:(radius),
//             light:false
//         }
//     }
//     pub fn new_light(center :Vec3,radius :f64)->Self{
//         Sphere{
//             center:(center),
//             radius:(radius),
//             light:true
//         }
//     }
// }
