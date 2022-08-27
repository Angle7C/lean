mod RayTrack;
fn main(){
    
}
#[cfg(test)]
mod  test{
    use image::Pixel;

    use crate::RayTrack::{vec4,ray};
    #[test]
    fn render_back(){
     const aspect_radio  : f64 = 16.0/9.0;
     const width:u32 =400;
     const height:u32=(width as f64/aspect_radio) as u32 ;
     
     let mut imag= image::RgbaImage::new(width, height);
     let viewport_h=2.0;
     let viewport_w=aspect_radio*viewport_h;
     let focal_length=1.0;
     let origin=vec4::Point3::new_point(0.0,0.0,0.0);
     let horizontal =vec4::Vec4::new_vector(viewport_w, 0.0, 0.0);
     let verital =vec4::Vec4::new_vector(0.0, viewport_h, 0.0);
     let lower_left_corner=origin-horizontal/2.0-verital/2.0-vec4::Vec4::new_vector(0.0, 0.0, focal_length);
     for i in 0..width{
        for j in 0..height{
            let u=i as f64/(width as f64-1.0);
            let v=j as f64/(width as f64-1.0);
            let ray =ray::Ray::new(origin, lower_left_corner+horizontal*u+verital*v-origin);
            let color = ray.ray_color();
            let pixel= imag.get_pixel_mut(i, j );
            // println!("{:?}",color.rgba());
            *pixel=color.rgba();
            
        }
     }
     imag.save("firstRender.png").unwrap();
    }
}