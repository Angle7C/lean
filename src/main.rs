mod lib;
fn main(){
    
}
#[cfg(test)]
mod  test{
    use lib::vec4;
    #[test]
    fn render_back(){
     const aspect_radio  : f64 = 16.0/9.0;
     const width:u32 =400;
     const height:u32=(width as f64/aspect_radio) as u32 ;
     
     let imag = image::ImageBuffer::new(width, height);

     let viewport_h=2.0;
     let viewport_w=aspect_radio*viewport_h;
     let focal_length=1.0;
     let origin=vec4::Point3::new_point(0.0,0.0,0.0);
    }
}