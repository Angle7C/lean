use std::f32::consts::PI;

use nalgebra::{Point3, Vector3};
use rand::Rng;

use crate::object::object::ModelData;

use super::boundbox::HitRecord;
#[derive(Debug)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}
impl Ray {
    #[inline]
    fn unit_rand(normal: &Vector3<f32>) -> Vector3<f32> {
        let mut rang = rand::thread_rng();
        let b = rang.gen_range(0.0..2.0 * PI);
        let cos_a = rang.gen_range(-1.0..1.0);
        let sin_a = f32::sqrt(1.0 - cos_a * cos_a);
        let v = Vector3::<f32>::new(sin_a * b.cos(), sin_a * b.sin(), cos_a);
        if v.dot(&normal) > 0.0 {
            return v;
        } else {
            return -v;
        }
    }
    pub fn new(o: Vector3<f32>, dir: Vector3<f32>) -> Self {
        Self {
            origin: (o),
            direction: (dir.normalize()),
        }
    }
    #[allow(dead_code)]
    pub fn at(&self, t: f32) -> Vector3<f32> {
        self.origin + self.direction * t
    }
    pub fn ray_color(
        &self,
        model: &ModelData,
        t_min: f32,
        t_max: f32,
        rec: Option<HitRecord>,
    ) -> Vector3<f32> {
        if rand::thread_rng().gen_bool(0.1) {
            return Vector3::<f32>::zeros();
        }
        match rec {
            Some(v) => {
                let t = v.normal + Self::unit_rand(&v.normal);
                let ray = Ray::new(v.p, t);
                let hit = model.hit(&ray, t_min, t_max);
                let cos = v.normal.dot(&self.direction);
                ray.ray_color(&model, t_min, t_max, hit)*0.5 /0.9
            }
            None => {
                let t = 0.5 * (self.direction.y + 1.0);
                Vector3::<f32>::new(1.0, 1.0, 1.0) * (1.0 - t)/0.9
                    + Vector3::<f32>::new(0.4, 0.7, 1.0) * t/0.9
            }
        }
    }
}
