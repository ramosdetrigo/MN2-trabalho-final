use sdl2::{image::ImageRWops, pixels::PixelFormatEnum, rwops::RWops, surface::Surface};

pub fn load_png(filename: &str) -> Surface<'static> {
    RWops::from_file(filename, "r").unwrap().load_png().unwrap()
}

pub fn get_pixelf(img: &Surface<'static>, x: u32, y: u32) -> (f64, f64, f64) {
    let index = (img.width() * y + x) as usize * 3;
    let pixels = img.without_lock().unwrap();
    (
        pixels[index] as f64 / 255.0,
        pixels[index + 1] as f64 / 255.0,
        pixels[index + 2] as f64 / 255.0,
    )
}

pub fn set_pixelf(img: &mut Surface<'static>, x: u32, y: u32, color: (f64, f64, f64)) {
    let index = (img.width() * y + x) as usize * 3;
    let pixels = img.without_lock_mut().unwrap();
    pixels[index] = (color.0 * 255.0) as u8;
    pixels[index + 1] = (color.1 * 255.0) as u8;
    pixels[index + 2] = (color.2 * 255.0) as u8;
}

// Aplicamos um kernel de tamanho K x K em cada pixel da matriz.
// Ignoramos os pixels das bordas, já que a matriz do kernel indexaria um pixel
// fora da imagem, que não existe.
pub fn convolution(image: &Surface<'static>, kernel: &[Vec<f64>]) -> Surface<'static> {
    let width = image.width();
    let height = image.height();
    let kernel_size = kernel.len();
    let k_center = kernel_size as i32 / 2;

    let mut output = Surface::new(width, height, PixelFormatEnum::RGB24).unwrap();

    // Iterando em cada pixel x,y da matriz
    for y in k_center..(height as i32 - k_center) {
        for x in k_center..(width as i32 - k_center) {
            let mut r = 0.0;
            let mut g = 0.0;
            let mut b = 0.0;

            // Aplica o kernel pro pixel (pega os pixels nos arredores e multiplica-os
            // pelos valores da matriz kernel, somando-os)
            for j in 0..kernel_size {
                for i in 0..kernel_size {
                    let px = x + i as i32 - k_center;
                    let py = y + j as i32 - k_center;

                    let (pr, pg, pb) = get_pixelf(image, px as u32, py as u32);
                    let k = kernel[j][i];

                    r += pr * k;
                    g += pg * k;
                    b += pb * k;
                }
            }

            set_pixelf(&mut output, x as u32, y as u32, (
                r.clamp(0.0, 1.0),
                g.clamp(0.0, 1.0),
                b.clamp(0.0, 1.0),
            ));
        }
    }

    output
}
