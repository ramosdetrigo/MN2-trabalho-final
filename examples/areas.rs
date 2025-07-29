use trabalho_final::unidade2::areas::*;

fn main() {
    let area =
        area_superficie_gauss_legendre(-50.0, 50.0, -50.0, 50.0, &|x, y| 0.2 * (x * x - y * y));

    println!("Área da superfície z = 0.2(x² - y²): {:.2} m²", area);

    let at = area_triangulo(3.0, 5.0, 1000);
    println!("Área do triângulo: {:.10}", at);

    let trap = area_trapezio(3.0, 1.0, 5.0, 2.0, 1000);
    println!("Área do trapézio: {:.10}", trap);

    let circ = area_circulo(2.0, 1000);
    println!("Área do círculo: {:.10}", circ);

    let elip = area_elipse(3.0, 1.5, 1000);
    println!("Área da elipse: {:.10}", elip);

    let fxy = |x: f64, y: f64| {
        let gx = 0.4 * x;
        let gy = 0.4 * y;
        (gx * gx + gy * gy + 1.0).sqrt()
    };

    let sup_cart = area_superficie_cartesiana(&fxy);
    println!("Área da superfície (cartesiano): {:.2} m²", sup_cart);

    let sup_polar = area_superficie_polar(&fxy);
    println!("Área da superfície (polar): {:.2} m²", sup_polar);

    let sup_cart_elip = area_superficie_cartesiana_com_elipse(&fxy);
    println!(
        "Área da superfície (restrita à elipse): {:.2} m²",
        sup_cart_elip
    );
}
