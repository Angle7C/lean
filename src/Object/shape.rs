use nalgebra::Matrix3;
use nalgebra::Point2;
use nalgebra::Point3;
use nalgebra::Vector2;
use nalgebra::Vector3;
use obj::IndexTuple;
use obj::ObjData;

use crate::raytrack::boundbox::HitRecord;
use crate::raytrack::ray::Ray;
#[derive(Debug, Default)]
pub struct Triangle {
    pub point: [Vector3<f32>; 3],
    pub uv: [Vector2<f32>; 3],
    pub normal: [Vector3<f32>; 3],
}
pub trait tr {
     fn hit();
     fn new_position();
}
impl Triangle {
    #[allow(unused)]
    pub fn new() -> Self {
       todo!();
    }
    pub fn new_position(data :&ObjData,index :&Vec<IndexTuple>)->Self{
        let point=[
            Vector3::from_column_slice(&data.position[index[0].0]),
            Vector3::from_column_slice(&data.position[index[1].0]),
            Vector3::from_column_slice(&data.position[index[2].0]),
        ];
        let uv=[
            Vector2::from_column_slice(&data.texture[index[0].1.unwrap()]),
            Vector2::from_column_slice(&data.texture[index[1].1.unwrap()]),
            Vector2::from_column_slice(&data.texture[index[2].1.unwrap()]),
        ];
        let normal=[
            Vector3::from_column_slice(&data.normal[index[0].2.unwrap()]),
            Vector3::from_column_slice(&data.normal[index[1].2.unwrap()]),
            Vector3::from_column_slice(&data.normal[index[2].2.unwrap()]),
        ];
        Self { point: (point), uv: (uv), normal: (normal) }
        // Self { point: (point) }
    }
    #[allow(unused)]
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let e1 = self.point[1] - self.point[0];
        let e2 = self.point[2] - self.point[0];
        let p = ray.direction.cross(&e2);
        let d = e1.dot(&p);
        if d.abs() <= f32::EPSILON {
            return None;
        }
        let inv_det = 1.0 / d;
        let t = Vector3::new(ray.origin.x, ray.origin.y, ray.origin.z) - self.point[0];
        let u = inv_det * t.dot(&p);
        if u < 0.0 || u > 1.0 {
            return None;
        }
        let q = t.cross(&e1);
        let v = ray.direction.dot(&q) * inv_det;
        if v < 0.0 || v+u > 1.0 {
            return None;
        }
        let t = e2.dot(&q) * inv_det;
        if t < t_min || t_max < t||t<0.0 {
            return None;
        } else {
            let normal=(1.0-u-v)*self.normal[0]+u*self.normal[1]+self.normal[2]*v;
            return Some(HitRecord::new(normal.normalize(), ray.at(t), t));
        }
        // todo!();
    }
}
pub struct  Squart{
    pub point:  [Vector3<f32>; 4],
    pub uv:     [Vector2<f32>; 4],
    pub normal: [Vector3<f32>; 4],
}
impl Squart {
    pub fn new_position(data :&ObjData,index :&Vec<IndexTuple>)->Self{
        let point=[
            Vector3::from_column_slice(&data.position[index[0].0]),
            Vector3::from_column_slice(&data.position[index[1].0]),
            Vector3::from_column_slice(&data.position[index[2].0]),
            Vector3::from_column_slice(&data.position[index[3].0]),
        ];
        let uv=[
            Vector2::from_column_slice(&data.texture[index[0].1.unwrap()]),
            Vector2::from_column_slice(&data.texture[index[1].1.unwrap()]),
            Vector2::from_column_slice(&data.texture[index[2].1.unwrap()]),
            Vector2::from_column_slice(&data.texture[index[3].1.unwrap()]),
        ];
        let normal=[
            Vector3::from_column_slice(&data.normal[index[0].2.unwrap()]),
            Vector3::from_column_slice(&data.normal[index[1].2.unwrap()]),
            Vector3::from_column_slice(&data.normal[index[2].2.unwrap()]),
            Vector3::from_column_slice(&data.normal[index[3].2.unwrap()]),
        ];
        Self { point: (point), uv: (uv), normal: (normal) }
    }
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        
        let t = ( self.point[0] - ray.origin).dot(&self.normal[0]) / ray.direction.dot(&self.normal[0]);
        if t<0.0||t<t_min||t>t_max{
            return None;
        } 
        let p=ray.at(t);
        let p01=self.point[0]-self.point[1];
        let p12=self.point[1]-self.point[2];
        let p23=self.point[2]-self.point[3];
        let p30=self.point[3]-self.point[0];
        let p0p=self.point[0]-p;
        let p1p=self.point[1]-p;
        let p2p=self.point[2]-p;
        let p3p=self.point[3]-p;
        let a=p01.cross(&p0p);
        let b=p12.cross(&p1p);
        let c=p23.cross(&p2p);
        let d=p30.cross(&p3p);
        if (a.z<0.0&&b.z<0.0&&c.z<0.0&&d.z<0.0)||(a.z>0.0&&b.z>0.0&&c.z>0.0&&d.z>0.0){
            Some(HitRecord::new(self.normal[0],p,t))
        }else{
            None
        }
    }
}