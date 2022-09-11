use crate::object::{object::ModelData, shape::{Triangle, Squart}};
use image::math;
use nalgebra::{Point3};
use obj::IndexTuple;
use crate::raytrack::ray::Ray;
use super::boundbox::{ AABB, HitRecord};
static mut D: f32 = 0.0;
#[derive(Debug)]
pub struct BVHNode {
    pub bound_box: AABB,
    pub next: Child,
}
#[derive(Debug)]
pub enum Child {
    Leaft((usize,usize,usize)),
    Node(Box<BVHNode>,Box<BVHNode>),
}
impl BVHNode {
    #[allow(unused)]
    pub fn new(model: &ModelData) -> Self {
        let (info, index) = GeomInfo::build_info(model);
        Self::build(info.into())
        // todo!();
    }
    #[allow(unused)]
    fn build(mut info: Vec<GeomInfo>)->Self {

        const  n :usize= 4;
        let mut min = Point3::<f32>::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut max = Point3::<f32>::new(-f32::INFINITY, -f32::INFINITY, -f32::INFINITY);
        info.iter().for_each(|a| {
            min.x = min.x.min(a.center.x);
            min.y = min.y.min(a.center.y);
            min.z = min.z.min(a.center.z);
            max.x = max.x.max(a.center.x);
            max.y = max.y.max(a.center.y);
            max.z = max.z.max(a.center.z);
        });

        let bound = AABB::new(min, max);
        if info.len()==1{
            return Self{
                bound_box: bound,
                next:Child::Leaft((info[0].index)),
            };
        }else if info.len()==2{
             let left=Self{
                bound_box: info[0].bound,
                next:Child::Leaft((info[0].index)),
            };
            let right=Self{
                bound_box:info[1].bound,
                next:Child::Leaft(info[1].index),
            };
            return Self{
                bound_box:bound,
                next:Child::Node(Box::new(left), Box::new(right)),
            }
        }
        let det = max - min;
        let area_s = det.x * det.y * 2.0 + det.x * det.z * 2.0 + det.y * det.z * 2.0;
        let mut index = 0;
        match det.argmax() {
            (0, v) => {
                unsafe {
                    D = v / n as f32;
                };
                index = 0;
                info.sort_by(|a, b| a.center.x.partial_cmp(&b.center.x).unwrap())
            }
            (1, v) => {
                unsafe {
                    D = v / n as f32;
                };
                index = 1;
                info.sort_by(|a, b| a.center.y.partial_cmp(&b.center.y).unwrap())
            }
            (2, v) => {
                unsafe {
                    D = v / n as f32;
                };
                index = 2;
                info.sort_by(|a, b| a.center.z.partial_cmp(&b.center.z).unwrap())
            }
            (_, _) => {
                panic!("this axis is None")
            }
        };
        let mut bounck: [AABB; n] = [AABB::zero(); n];
        //初始化bounck
        
        info.iter().for_each(|x| {

            let i =unsafe {
                let mut t=((x.center[index] - min[index]-0.001)/D) as usize;
                // if t>=n{
                //     t=n-1;
                // }
                t
            };
            // println!("i: {}",i);
            bounck[i] = AABB::new(x.center, x.center);
        });
        let mut left = AABB::new(info.first().unwrap().center, info.first().unwrap().center);
        let mut right = AABB::new(info.last().unwrap().center, info.last().unwrap().center);
        let mut l = 0.0;
        let mut r = 0.0;
        let mut min_value = f32::INFINITY;
        let mut spilt_n=0;
        for i in 0..n {
            for item in info.iter() {
                if unsafe { item.center[index] < min.x + i as f32 * D / n as f32 } {
                    left = left.merge(&item.bound);
                    l += 1.0;
                } else {
                    right = right.merge(&item.bound);
                    r += 1.0;
                }
            }
            if min_value > 10.0 + left.area() / area_s * l + right.area() / area_s * r {
                min_value = 10.0 + left.area() / area_s * l*10.0 + right.area() / area_s * r*10.0; 
                               spilt_n = i;
            }
        }
        let len=info.len()*spilt_n/n;
        let (l_node, r_node) = info.split_at(len);
        let left=Self::build( l_node.into());
        let right=Self::build(r_node.into());
        return Self{
            bound_box:bound,
            next:Child::Node(Box::new(left), Box::new(right)),
        }
    }
    #[allow(unused)]
    pub fn hit(&self,ray:&Ray,model :&ModelData, t_min:f32,t_max:f32)->Option<HitRecord>{
        if self.bound_box.hit(ray, t_min, t_max){
            match &self.next {
                Child::Leaft((i,j,k))=>{
                        let shape = model.as_shape(*i).unwrap();
                        let poly = shape.get(*j ).unwrap().polys.get(*k).unwrap();
                        let p=&poly.0;
                        if p.len()==3{
                            let triangle=Triangle::new_position(&model.data,p);
                            return triangle.hit(ray, t_min, t_max)
                        }else{
                            //多边形
                            let squart=Squart::new_position(&model.data,p);
                            return squart.hit(ray, t_min, t_max)
                        }
                    },
                Child::Node(a,b)=>{
                    let l=a.hit(ray, model, t_min, t_max);
                    let r=b.hit(ray, model, t_min, t_max);
                    match (l,r) {
                        (None,None)=>return None,
                        (Some(v),None)|(None,Some(v))=> return Some(v),
                        (Some(a),Some(b))=>{
                            if a.t<b.t{
                                return Some(a);
                            }else{
                                return Some(b);
                            };
                        },
                    };
                    
                }
            }
        }else{
            return None;
        }
    }
}
impl BVHNode {}
#[derive(Debug,Clone)]
pub struct GeomInfo {
    pub center: Point3<f32>,
    pub bound: AABB,
    //第i个obj，第j个mesh，第k个图元
    pub index: (usize, usize, usize),
}
impl GeomInfo {
    pub fn build_info(set: &ModelData) -> (Vec<GeomInfo>, Vec<usize>) {
        let data = &set.data;
        let n = data.objects.len();
        let vetex = &data.position;
        let mut obj_num = Vec::<usize>::new();
        let mut info = Vec::<GeomInfo>::new();
        for i in 0..n {
            if let Some(v) = set.as_shape(i) {
                //对不同模型下的所有mesh
                for (j, item) in v.iter().enumerate() {
                    //对一个模型下的不同类型mesh
                    for (k, t) in item.polys.iter().enumerate() {
                        //对多边形进行编辑
                        let shape = &t.0;
                        if shape.len() == 4 {
                            let (v, bound) = Self::shape_message_squart(shape, vetex);
                            let index = (i, j, k);
                            info.push(GeomInfo {
                                center: (v),
                                bound: (bound),
                                index: (index),
                            });
                        }
                        //对三角形进行编辑
                        if shape.len() == 3 {
                            let (v, bound) = Self::shape_message_triangle(shape, vetex);
                            let index = (i, j, k);
                            info.push(GeomInfo {
                                center: (v),
                                bound: (bound),
                                index: (index),
                            });
                        }
                    }
                }
                obj_num.push(v.len());
            }
        }
        (info, obj_num)
    }
    fn shape_message_triangle(
        shape_message: &Vec<IndexTuple>,
        vetex: &Vec<[f32; 3]>,
    ) -> (Point3<f32>, AABB) {
        let (ref a, ref b, ref c) = (shape_message[0], shape_message[1], shape_message[2]);
        let (pa, pb, pc) = (&vetex[a.0], &vetex[b.0], &vetex[c.0]);
        let center = Point3::<f32>::new(
            pa[0] + pb[0] + pc[0],
            pa[1] + pb[1] + pc[1],
            pa[1] + pb[1] + pc[1],
        ) / 3.0;
        let min = Point3::<f32>::new(
            pa[0].min(pb[0]).min(pc[0]),
            pa[1].min(pb[1]).min(pc[1]),
            pa[2].min(pb[2]).min(pc[2]),
        );
        let max = Point3::<f32>::new(
            pa[0].max(pb[0]).max(pc[0]),
            pa[1].max(pb[1]).max(pc[1]),
            pa[2].max(pb[2]).max(pc[2]),
        );
        let aabb = AABB::new(min, max);
        (center, aabb)
    }
    fn shape_message_squart(
        shape_message: &Vec<IndexTuple>,
        vetex: &Vec<[f32; 3]>,
    ) -> (Point3<f32>, AABB) {
        let (ref a, ref b, ref c,ref d) = (shape_message[0], shape_message[1], shape_message[2],shape_message[3]);
        let (pa, pb, pc,pd) = (&vetex[a.0], &vetex[b.0], &vetex[c.0],&vetex[d.0]);
        let center = Point3::<f32>::new(
            pa[0] + pb[0] + pc[0]+pd[0],
            pa[1] + pb[1] + pc[1]+pd[0],
            pa[1] + pb[1] + pc[1]+pd[0],
        ) / 4.0;
        let min = Point3::<f32>::new(
            pa[0].min(pb[0]).min(pc[0]).min(pd[0]),
            pa[1].min(pb[1]).min(pc[1]).min(pd[1]),
            pa[2].min(pb[2]).min(pc[2]).min(pd[2]),
        );
        let max = Point3::<f32>::new(
            pa[0].max(pb[0]).max(pc[0]).max(pd[0]),
            pa[1].max(pb[1]).max(pc[1]).max(pd[1]),
            pa[2].max(pb[2]).max(pc[2]).max(pd[2]),
        );
        let aabb = AABB::new(min, max);
        (center, aabb)
    }
}
