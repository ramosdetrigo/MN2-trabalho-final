use sdl2::{image::SaveSurface, surface::Surface};
use trabalho_final::unidade1::imagem::*;

fn main() {
    let img = load_png("boats.png");

    // Suavizando a imagem com um kernel gaussiano normalizado
    let gaussian_kernel = [
        vec![1.0 / 16.0, 2.0 / 16.0, 1.0 / 16.0],
        vec![2.0 / 16.0, 4.0 / 16.0, 2.0 / 16.0],
        vec![1.0 / 16.0, 2.0 / 16.0, 1.0 / 16.0],
    ];
    let img = convolution(&img, &gaussian_kernel);

    // Kernel Laplaciano 3x3
    let laplace_kernel = [
        vec![1.0,  4.0, 1.0],
        vec![1.0, -20.0, 4.0],
        vec![1.0,  4.0, 4.0],
    ];

    // Aplica o filtro Laplaciano
    let img_a = convolution(&img, &laplace_kernel); // Imagem A

    // Cria a imagem B com threshold de tolerância
    let mut img_b = Surface::new(img.width(), img.height(), img.pixel_format_enum()).unwrap();
    for x in 0..img.width() {
        for y in 0..img.height() {
            let pixel_a = get_pixelf(&img_a, x, y);
            let pixel_a_avg = (pixel_a.0 + pixel_a.1 + pixel_a.2) / 3.0;

            // Verifica se o pixel é "zero"
            // (usei o threshold 0.25 pois foi o que melhor funcionou com a maior parte das imagens)
            if pixel_a_avg < 0.25 {
                set_pixelf(&mut img_b, x, y, (0.0, 0.0, 0.0)); // Preto
            } else {
                set_pixelf(&mut img_b, x, y, (1.0, 1.0, 1.0)); // Branco
            }
        }
    }

    // Salva as imagens
    img.save("Loutput_gaussian.png").unwrap();
    img_a.save("LoutputA.png").unwrap();
    img_b.save("LoutputB.png").unwrap();
}
