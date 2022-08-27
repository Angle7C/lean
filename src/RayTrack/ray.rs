use super::vec4::{self, Vec4};
pub struct Ray{
    pub orig:vec4::Point3,
    pub dir :vec4::Vec4,
}
impl Ray{
    pub fn new(origin :vec4::Point3,direction:vec4::Vec4)->Ray{
        Ray { orig: (origin), dir: (direction.normalized()) }
    }
    pub fn at(&self,t :f64)->Vec4{
        self.orig+self.dir*t
    }
    pub fn ray_color(&self)->Vec4{
        let t=0.5*(self.dir.y+1.0);
        Vec4::new_vector(1.0, 1.0, 1.0)*(1.0-t)+Vec4::new_vector(0.4, 0.4, 1.0)*t
    }
}