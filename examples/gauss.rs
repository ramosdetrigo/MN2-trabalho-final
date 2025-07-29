use trabalho_final::unidade2::gauss::*;

fn main() {
    let f_legendre = |x: f64| x.sin();
    let f_outros = |x: f64| x * x;

    let resultado_legendre = gauss_legendre(1.0, 3.0, 3, &f_legendre);
    let resultado_hermite = gauss_hermite(3, &f_outros);
    let resultado_laguerre = gauss_laguerre(3, &f_outros);
    let resultado_chebyshev = gauss_chebyshev(4, &f_outros);

    println!("Gauss-Legendre:  {:.8}", resultado_legendre);
    println!("Gauss-Hermite:   {:.8}", resultado_hermite);
    println!("Gauss-Laguerre:  {:.8}", resultado_laguerre);
    println!("Gauss-Chebyshev: {:.8}", resultado_chebyshev);
}
