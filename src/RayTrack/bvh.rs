use super::{aabb::AABB, hit_record::Hittable};
use super::hit_record::HitRecord;
pub struct BVHNode {
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
    boundBox: AABB,
}
impl BVHNode {
    pub fn new(list: Vec<Box<dyn Hittable>>)->Box<dyn Hittable>{
        Self::build(list)
    }   
    fn build(mut list: Vec<Box<dyn Hittable>>) -> Box<dyn Hittable> {
        match list.len() {
            0 => panic!("this Object list is empty"),
            1 => list.remove(0),
            _ => {
                list.sort_by(|a, b| {
                    a.boundBox()
                        .unwrap()
                        .min
                        .x
                        .partial_cmp(&b.boundBox().unwrap().min.x)
                        .unwrap()
                });
                let mut a=list;
                let b=a.split_off(a.len()/2);
                let left=Self::build(a);
                let right=Self::build(b);
                let bound_box=left.boundBox().unwrap().merage(&right.boundBox().unwrap());
                Box::new(Self{
                    left:left,
                    right:right,
                    boundBox:bound_box,
                })
            },
        }
    }
}
impl Hittable for BVHNode {
    fn boundBox(&self) -> Option<AABB> {
        Some(self.boundBox)
    }
    fn hit(
        &self,
        r: &super::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<HitRecord> {
        match self.boundBox.hit(r, t_min, t_max) {
            false=>None,
            true=>{
                match (self.left.hit(r, t_min, t_max),self.right.hit(r, t_min, t_max)) {
                    (None,None)=>None,
                    (None,Some(v))|(Some(v),None)=>Some(v),
                    (Some(a),Some(b))=>{
                        if a.t<b.t {
                            Some(a)
                        }else{
                            Some(b)
                        }
                    }

                }
            }  
        }
    }
}
mod test{
    use crate::object::sphere::Sphere;
    use crate::raytrack::hit_record::Hittable;
    use crate::raytrack::ray::Ray;
    use crate::raytrack::vec3::Vec3;
    use super::BVHNode;
    #[test]
    fn test_bvh(){
            let s1 = Sphere::new(Vec3::new(0.0,1.0,0.0), 1.0);
            let s2 = Sphere::new(Vec3::new(0.0,-1.0,0.0), 1.0);
            let list:Vec<Box<dyn Hittable>> =vec![Box::new(s1),Box::new(s2)];
            let BVH_tree = BVHNode::new(list);
            let ray=Ray::new(Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0,0.0, 1.0));
            let rec = BVH_tree.hit(&ray, 0.0, 100.0);
            rec.unwrap();
        }
}
