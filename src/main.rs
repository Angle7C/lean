mod Object;
mod Raytrack;
use Object::sphere::Sphere;
fn main() {}
#[cfg(test)]
mod test {
    use crate::Object::sphere::Sphere;
    use crate::Raytrack::ray::Ray;
    use crate::Raytrack::vec3::Vec3;
    use crate::Raytrack::hit_record::{Hittable, HittableList};
    #[test]
    fn draw_sphere() {
        let a=Sphere::new(Vec3::new(0.0, 100.0, -1.0), 99.5);
        let b=Sphere::new(Vec3::new(0.0,0.0,  -1.0), 0.5);
        let mut world=HittableList::new();
        world.add(Box::new(b));

        world.add(Box::new(a));

        let width = 800;
        let height = 800;
        let mut image = image::RgbaImage::new(width, height);
        for (_,_,item) in image.enumerate_pixels_mut(){
            *item=image::Rgba([127,127,127,255])
        }

        // return ;
        let lower_left_corner=Vec3::new(-4.0, -2.0, -1.0);
        let horizontal=Vec3::new(8.0, 0.0, 0.0);
        let vertical=Vec3::new(0.0, 4.0, 0.0);
        let origin=Vec3::new(0.0, 0.0, 0.0);
        for i in 0..width {
            for j in 0..height {
                let u = i as f64 / (width-1) as f64;
                let v = j as f64 / (height-1) as f64;
                let  r= Ray::new(origin, lower_left_corner + horizontal*u + vertical*v-origin);
                let pixel = image.get_pixel_mut(i, j);
                for i in 0..100{
                    *pixel=r.ray_color(&world).rgba();
                }
            }
        }
        image.save("sphere_test.png").unwrap();

    }
}
