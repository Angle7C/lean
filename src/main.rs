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
    let x = clamp((vec.x * sc).sqrt(), 0.0, 1.0);
    let y = clamp((vec.y * sc).sqrt(), 0.0, 1.0);
    let z = clamp((vec.z * sc).sqrt(), 0.0, 1.0);

    image::Rgb([
        (x * 255.0 as f32) as u8,
        (y * 255.0 as f32) as u8,
        (z * 255.0 as f32) as u8,
    ])
}
mod test {
    use image::Progress;
    use indicatif::ProgressBar;
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
        // let model= ModelData::new("obj/".to_string(),String::from("Cube.obj"));
        let t1 = Instant::now();
        let model= ModelData::new("obj/".to_string(),String::from("Cube.obj"));
        println!("{} ms",t1.elapsed().as_millis());

        let camera = Camera::new(
            Vector3::<f32>::new(25.0,25.0, 25.0),
            Vector3::<f32>::new(0.0, 00.0, 0.0),
            90.0,
            1.0,
        );
        let width = 300;
        let hight = 300;
        let ssp = 10;
        let t_min = 0.000;
        let t_max = 10000.0;
        let mut rand = rand::thread_rng();
        let mut image = image::RgbImage::new(width, hight);
        let mut sum = Vector3::<f32>::zeros();
        let mut u=0.0;
        let mut v=0.0;
        let pb=ProgressBar::new(width as u64);
        for i in 0..width {
       
            for j in 0..hight {
                let pixel = image.get_pixel_mut(i, j);
                for _s in 0..ssp {
                    u = (i as f32 + rand.gen_range(0.0..1.0)) / width as f32;
                    v = (j as f32 + rand.gen_range(0.0..1.0)) / hight as f32;
                    let ray = camera.get_ray(u, v);
                    let rec = model.hit(&ray, t_min, t_max);
                    let color=ray.ray_color(&model, t_min, t_max, rec);
                    // if let Some(v) = rec {
                    //     sum = (v.normal+Vector3::<f32>::new(1.0, 1.0, 1.0))*0.5;
                    // } else {
                    //     sum = {
                    //         let t = 0.5 * (ray.direction.y + 1.0);
                    //         Vector3::<f32>::new(1.0, 1.0, 1.0) * (1.0 - t)
                    //             + Vector3::<f32>::new(0.4, 0.7, 1.0) * t
                    //     }
                    // }
                    sum=sum+color;
                }
                
                *pixel = rgb(sum, ssp);
                sum=Vector3::<f32>::zeros();
            }
            pb.inc(1);
            
        }
        image.save("ans.png").unwrap();
        pb.finish();
    }
}
