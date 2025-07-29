use trabalho_final::unidade3::householder::householder::*;
use trabalho_final::unidade3::householder::qr::*;
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

    println!("====== HOUSEHOLDER ======");
    // matriz acumulada H e matriz T
    let (t, h) = loop_householder(c);
    println!("Matriz H\n{h}\nMatriz T\n{t}");

    println!("====== MÉTODO QR ======");
    let (x, a) = metodo_qr(c, 1e-10);
    println!("Matriz X:\n{x}\nAutovalores (diagonal): \n{a}");

    println!("====== DECOMPOSIÇÃO ESPECTRAL ======");
    // Construir matriz (diagonal com autovalores)
    let diagonal = {
        let mut m = Matrix::<f64, 5, 5>::zero();
        for i in 0..5 {
            m[i][i] = a[i];
        }
        m
    };

    // X * A(diagonal) * X^T
    let c_reconstruida = x * diagonal * x.transpose();
    println!("Matriz C original:\n{c}");
    println!("Matriz reconstruída (X_C * Λ * X_C^T):\n{c_reconstruida}");
}