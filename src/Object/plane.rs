use super::super::raytrack::ray::Ray;
use crate::raytrack::aabb::AABB;
use crate::raytrack::hit_record::HitRecord;
use crate::raytrack::hit_record::Hittable;
use crate::raytrack::vec3::Vec3;
pub struct Plane {
    pub normal: Vec3,
    pub p1: Vec3,
    pub p2: Vec3,
    pub p3: Vec3,
    pub p4: Vec3,
    pub light: bool,
}
impl Plane {
    pub fn new(p1: Vec3, p2: Vec3,p3 :Vec3,p4 :Vec3, normal: Vec3, light: bool) -> Self {
        Self {
            normal: (normal.normalized()),
            p1: (p1),
            p2: (p2),
            p3: (p3),
            p4: (p4),
            light: (light),
        }
    }
}
impl Hittable for Plane {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        //    todo!();
        let op = self.p1-r.orig;
        let t = (-r.dir * self.normal) / (op * self.normal);
        if t < t_min || t > t_max {
            return None;
        }
        let p = r.at(t);
        if p.x < self.p1.x.min(self.p2.x.min(self.p3.x.min(self.p4.x))) || p.x >  self.p1.x.max(self.p2.x.max(self.p3.x.max(self.p4.x))) {
            return None;
        }
        if p.y < self.p1.y.min(self.p2.y.min(self.p3.y.min(self.p4.y))) || p.y >  self.p1.y.max(self.p2.y.max(self.p3.y.max(self.p4.y))) {
            return None;
        }
        if p.z < self.p1.z.min(self.p2.z.min(self.p3.z.min(self.p4.z))) || p.y >  self.p1.z.max(self.p2.z.max(self.p3.z.max(self.p4.z))) {
            return None;
        }
        let mut rec = HitRecord::new(self.normal, p, t, true, self.light);
        rec.set_face_norm(r);
        Some(rec)
    }
    fn boundBox(&self) -> Option<AABB> {
        let min=Vec3::new(
            self.p1.x.min(self.p2.x.min(self.p3.x.min(self.p4.x))),self.p1.y.min(self.p2.y.min(self.p3.y.min(self.p4.y))),self.p1.z.min(self.p2.z.min(self.p3.z.min(self.p4.z)))
        );
        let max =Vec3::new(
            self.p1.x.max(self.p2.x.max(self.p3.x.max(self.p4.x))),self.p1.y.max(self.p2.y.max(self.p3.y.max(self.p4.y))),self.p1.z.max(self.p2.z.max(self.p3.z.max(self.p4.z)))
        );
        Some(AABB::new(min,max))
    }
}
