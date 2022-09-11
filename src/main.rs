use nalgebra::Point3;
use nalgebra::{clamp, Vector3};
use rand::Rng;
use std::time::Instant;

use crate::object::{camera::Camera};

mod object;
mod raytrack;
fn main() {
}

#[inline]
fn rgb(vec: Vector3<f32>, ssp: i32) -> image::Rgb<u8> {
    let sc = 1.0 / ssp as f32;
    let x = clamp((vec.x * sc).sqrt(), 0.0, 255.0);
    let y = clamp((vec.y * sc).sqrt(), 0.0, 255.0);
    let z = clamp((vec.z * sc).sqrt(), 0.0, 255.0);

    image::Rgb([
        (x * 255.0 as f32) as u8,
        (y * 255.0 as f32) as u8,
        (z * 255.0 as f32) as u8,
    ])
}
mod test {
    use nalgebra::{clamp, Vector3,Point3};
    use rand::Rng;
    use core::time;
    use std::thread;
    use std::time::Instant;
    use crate::object::object::ModelData;
    use crate::rgb;
    use crate::object::{camera::Camera};
    #[test]
    fn darw() {
        // let model= ModelData::new("obj/".to_string(),String::from("untitled.obj"));
        let model= ModelData::new("obj/".to_string(),String::from("five_obj.obj"));

        let camera = Camera::new(
            Vector3::<f32>::new(5.0,5.0, 5.0),
            Vector3::<f32>::new(0.0, 0.0, 0.0),
            90.0,
            1.0,
        );
        let width = 200;
        let hight = 200;
        let ssp = 1;
        let t_min = 0.00;
        let t_max = 10000.0;
        let mut rand = rand::thread_rng();
        let mut image = image::RgbImage::new(width, hight);
        let mut sum = Vector3::<f32>::zeros();
        let mut u=0.0;
        let mut v=0.0;
        for i in 0..width {
            let t1 = Instant::now();
            for j in 0..width {
                let pixel = image.get_pixel_mut(i, j);
                for _s in 0..ssp {
                    u = (i as f32 + rand.gen_range(0.0..1.0)) / width as f32;
                    v = (j as f32 + rand.gen_range(0.0..1.0)) / width as f32;
                    let ray = camera.get_ray(u, v);
                    let rec = model.hit(&ray, t_min, t_max);
                    if let Some(v) = rec {
                        sum = (v.normal);;
                    } else {
                        sum = {
                            let t = 0.5 * (ray.direction.y + 1.0);
                            Vector3::<f32>::new(1.0, 1.0, 1.0) * (1.0 - t)
                                + Vector3::<f32>::new(0.4, 0.7, 1.0) * t
                        }
                    }
                }
                
                *pixel = rgb(sum, ssp);
                sum=Vector3::<f32>::zeros();
            }
            // println!("{} ms",t1.elapsed().as_millis());
        }
        image.save("ans.png").unwrap();
    }
}
