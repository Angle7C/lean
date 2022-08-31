use super::vec3::Vec3;

pub struct Camera{
    pub origin: Vec3,
    pub up: Vec3,
    pub fov: f64,
    pub near: f64,
    pub far: f64,
}

