mod object;
mod raytrack;
fn main() {}
#[cfg(test)]
mod test {
    use crate::object::plane::Plane;
    use crate::object::sphere::Sphere;
    use crate::raytrack::bvh::BVHNode;
    use crate::raytrack::camera::Camera;
    use crate::raytrack::hit_record::Hittable;
    use crate::raytrack::vec3::Vec3;
    use rand::Rng;

    fn init() {
        log4rs::init_file("log.yaml", Default::default()).unwrap();
    }
    #[test]
    fn draw_sphere() {
        init();
        let ssp = 10.0;
        let s1 = Sphere::new(Vec3::new(0.0, 100.0, 0.0), 97.0);
        let s2 = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 3.0);
        let p1 = Plane::new(
            Vec3::new(-100.0, -100.0, 0.0),
            Vec3::new(100.0, 100.0, 0.0),
            Vec3::new(100.0, -100.0, 0.0),
            Vec3::new(-100.0, 100.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
            true,
        );
        let mut list: Vec<Box<dyn Hittable>> = vec![];
        list.push(Box::new(s1));
        list.push(Box::new(s2));
        list.push(Box::new(p1));
        let bvh_tree = BVHNode::new(list);
        let width = 400;
        let height = 400;
        let mut image = image::RgbImage::new(width, height);
        let mut rand = rand::thread_rng();
        let camera = Camera::new(
            Vec3::new(0.0, 0.0, -15.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0,
            1.0,
        );
        for i in 0..width {
            for j in 0..height {
                let mut sum = Vec3::new(0.0, 0.0, 0.0);
                let pixel = image.get_pixel_mut(i, j);
                for _i in 0..ssp as i32 {
                    let u = (i as f64 + rand.gen::<f64>()) / (width) as f64;
                    let v = (j as f64 + rand.gen::<f64>()) / (height) as f64;
                    let ray = camera.getRay(u, v);
                    let color = ray.path_color(bvh_tree.hit(&ray, 0.0000, 10000.0), &bvh_tree, 10);
                    sum = sum + color;
                }
                *pixel = sum.rgba(ssp);
            }
        }
        image.save("sphere_test.png").unwrap();
    }
}
