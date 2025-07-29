use trabalho_final::unidade2::areas::*;

fn main() {
    let area =
        area_superficie_gauss_legendre(-50.0, 50.0, -50.0, 50.0, &|x, y| 0.2 * (x * x - y * y));

    println!("Área da superfície z = 0.2(x² - y²): {:.2} m²", area);
}
