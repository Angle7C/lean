use super::vec4::{self, Vec4};
pub struct Ray{
    pub orig:vec4::Point3,
    pub dir :vec4::Vec4,
}
impl Ray{
    fn new(origin :vec4::Point3,direction:vec4::Vec4)->Ray{
        Ray { orig: (origin), dir: (direction) }
    }
    fn at(&self,t :f64)->Vec4{
        self.orig+self.dir*t
    }
}