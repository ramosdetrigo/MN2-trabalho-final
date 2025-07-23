use sdl2::{image::{ImageRWops, SaveSurface}, pixels::{self, PixelFormat, PixelFormatEnum}, rwops::{self, RWops}, surface::Surface};

fn main() {
    let mut img = RWops::from_file("img1.png", "r").unwrap().load_png().unwrap();

    img.convert_format(PixelFormatEnum::RGB888).unwrap();
    let pixels: &mut [u8] = img.without_lock_mut().unwrap();

    for pixel in pixels.chunks_mut(3) {
        let avg = (pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32)/3;
        if avg < 100 {
            pixel[0] = 0;
            pixel[1] = 0;
            pixel[2] = 0;
        }
    }

    img.save("output.png").unwrap();
}
