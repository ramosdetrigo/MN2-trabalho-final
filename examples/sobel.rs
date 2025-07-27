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

    // Kernel sobel 3x3 (retirado do vídeo)
    let sobel_x = [
        vec![-1.0, 0.0, 1.0],
        vec![-2.0, 0.0, 2.0],
        vec![-1.0, 0.0, 1.0],
    ];
    let sobel_y = [
        vec![1.0, 2.0, 1.0],
        vec![0.0, 0.0, 0.0],
        vec![-1.0, -2.0, -1.0],
    ];

    // Realizamos uma convolução com os kernels de sobel para pegar as derivadas X e Y
    let img_a = convolution(&img, &sobel_x); // dx
    let img_b = convolution(&img, &sobel_y); // dy

    // Imagem C (gradiente da imagem original)
    let mut img_c = Surface::new(img.width(), img.height(), img.pixel_format_enum()).unwrap();

    // A raiz dos quadrados de cada pixel (clamp entre 0 e 1) nos dá o gradiente
    for x in 0..img.width() {
        for y in 0..img.height() {
            let pixel_a = get_pixelf(&img_a, x, y);
            let pixel_b = get_pixelf(&img_b, x, y);

            // eleva os pixels ao quadrado, soma eles, e tira a raiz
            let pixel_c = (
                (pixel_a.0.powi(2) + pixel_b.0.powi(2)).sqrt().min(1.0),
                (pixel_a.1.powi(2) + pixel_b.1.powi(2)).sqrt().min(1.0),
                (pixel_a.2.powi(2) + pixel_b.2.powi(2)).sqrt().min(1.0),
            );

            set_pixelf(&mut img_c, x, y, pixel_c);
        }
    }

    // Imagem D (imagem final com threshold)
    let mut img_d = Surface::new(img.width(), img.height(), img.pixel_format_enum()).unwrap();
    for x in 0..img.width() {
        for y in 0..img.height() {
            let pixel_c = get_pixelf(&img_c, x, y);

            // Aplicando um threshold de 0.5 (colore o pixel de preto se estiver abaixo de 0.5, e de branco se acima)
            // OBS: Como as imagens estão em preto e branco, podemos assumir que
            // o valor do pixel é qualquer um dos 3 valores (r, g, ou b)
            if pixel_c.0 < 0.5 {
                set_pixelf(&mut img_d, x, y, (0.0, 0.0, 0.0));
            } else {
                set_pixelf(&mut img_d, x, y, (1.0, 1.0, 1.0));
            }
        }
    }

    // Salvando as imagens
    img.save("Soutput_gaussian.png").unwrap();
    img_a.save("SoutputA.png").unwrap();
    img_b.save("SoutputB.png").unwrap();
    img_c.save("SoutputC.png").unwrap();
    img_d.save("SoutputD.png").unwrap();
}
