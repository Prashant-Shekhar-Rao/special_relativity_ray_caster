use image;
use std::time::SystemTime;
pub fn output(to_image: Vec<Vec<Vec<u8>>>) {
    let Reso=crate::ini_reader ::read_lines();
    let imgx = Reso.3[0] as u32;
    let imgy = Reso.3[1] as u32;
    if to_image.len() as u32 != imgx || to_image[0].len() as u32 != imgy {
        panic!("Your vector was of incorrect size. to_image.len() as u32!=imgy ||to_image.[0].len() as u32!=imgx failed");
    }

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

  
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = ((to_image[(x) as usize][(y) as usize][0] as f64 / 255.0f64)
            .powf(1.0f64 / 1.5f64)
            * 255.0f64) as u8;
        let b = ((to_image[(x) as usize][(y) as usize][1] as f64 / 255.0f64)
            .powf(1.0f64 / 1.5f64)
            * 255.0f64) as u8;
        let g = ((to_image[(x) as usize][(y) as usize][2] as f64 / 255.0f64)
            .powf(1.0f64 / 1.5f64)
            * 255.0f64) as u8;
        *pixel = image::Rgb([r, g, b]);
    
    }
 
    let now=SystemTime::now();
    let s=format!{"special_realtivity_output  {}.png",now.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()};
   
    imgbuf.save(s).unwrap();
}
