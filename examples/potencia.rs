use trabalho_final::unidade3::potencia::potencia::*;
use trabalho_final::utils::*;

fn main() {
    // matriz B^-1
    let b_1: Matrix<f64, 5, 5> = Matrix::new([
        [1.0 / 2.0, 0.0, 0.0, 0.0, 0.0],
        [0.0, 1.0 / 3.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0 / 4.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 1.0 / 5.0, 0.0],
        [0.0, 0.0, 0.0, 0.0, 1.0 / 6.0],
    ]);

    // matriz A
    let a: Matrix<f64, 5, 5> = Matrix::new([
        [8.0, 4.0, 2.0, 6.0, 10.0],
        [6.0, 27.0, 9.0, 21.0, 24.0],
        [4.0, 12.0, 44.0, 24.0, 16.0],
        [15.0, 35.0, 30.0, 70.0, 35.0],
        [30.0, 48.0, 24.0, 42.0, 60.0],
    ]);

    // matriz C (B^-1 * A)
    let c = b_1 * a;

    // Vetor inicial pro método da potência
    let v0_5: VecN<f64, 5> = VecN::splat(1.0);

    println!("========== POTÊNCIA REGULAR ==========");
    let (l0, x0) = potencia_regular(c, v0_5, 1e-6);
    println!("Autovalor: {l0:.4}\nAutovetor: {x0}");

    println!("\n========== POTÊNCIA INVERSA ==========");
    let (l4, x4) = potencia_inversa(c, v0_5, 1e-6);
    println!("Autovalor: {l4:.4}\nAutovetor: {x4}");

    println!("\n========== POTÊNCIA COM DESLOCAMENTO ==========");
    let deslocamentos = [2.5, 5.0, 10.0];

    for u in deslocamentos {
        let (li, xi) = potencia_deslocamento(c, v0_5, 1e-6, u);
        println!("u = {u}\nAutovalor: {li:.4}\nAutovetor: {xi}\n",);
    }//30.3097, 8.8122, 5.2981, 3.6858, -0.1058

    let b: Matrix<f64, 5, 5> = Matrix::new([
        [2.0,0.0,0.0,0.0,0.0],
        [0.0,3.0,0.0,0.0,0.0],
        [0.0,0.0,4.0,0.0,0.0],
        [0.0,0.0,0.0,5.0,0.0],
        [0.0,0.0,0.0,0.0,6.0],
    ]);

    // calcula e printa Ax
    println!("Ax:  {}", a * x0);
    // calcula e printa lBx (é pros dois serem iguais ou mt próximos)
    println!("lBx: {}", x0 * b * l0);
}