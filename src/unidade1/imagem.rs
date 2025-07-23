use sdl2::{image::ImageRWops, rwops::RWops, surface::Surface};

pub fn load_png(filename: &str) -> Surface<'static> {
    RWops::from_file(filename, "r").unwrap().load_png().unwrap()
}

pub fn get_pixel(img: &Surface<'static>, x: u32, y: u32) -> [u8; 3] {
    let index = (img.width() * y + x) as usize;
    let pixels = img.without_lock().unwrap();
    [pixels[index], pixels[index + 1], pixels[index + 2]]
}

pub fn set_pixel(img: &mut Surface<'static>, x: u32, y: u32, color: [u8; 3]) {
    let index = (img.width() * y + x) as usize;
    let pixels = img.without_lock_mut().unwrap();
    pixels[index] = color[0];
    pixels[index + 1] = color[1];
    pixels[index + 2] = color[2];
}

pub fn px_avg(pixel: &[u8]) -> u32 {
    (pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) / 3
}

pub fn px_set(pixel: &mut [u8], r: u8, g: u8, b: u8) {
    pixel[0] = r;
    pixel[1] = g;
    pixel[2] = b;
}