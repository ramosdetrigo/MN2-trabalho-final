use sdl2::image::SaveSurface;
use trabalho_final::unidade1::imagem::*;

fn main() {
    let mut img = load_png("img1.png");

    let pixels = img.without_lock_mut().unwrap().chunks_mut(3);
    for pixel in pixels {
        if px_avg(pixel) < 128 {
            px_set(pixel, 0, 0,0 );
        }
    }

    img.save("output.png").unwrap();
}
