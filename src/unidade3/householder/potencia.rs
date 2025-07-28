#![allow(dead_code)]

use crate::utils::{Matrix, VecN};

pub fn potencia_regular<const N: usize>(
    a: Matrix<f64, N, N>,
    v0: VecN<f64, N>,
    tolerancia: f64,
) -> (f64, VecN<f64, N>) {
    let mut lambda = 0.0;
    let mut v = v0;

    loop {
        // calcula o valor de x1 (aproximação normalizada)
        let x = v.normalized();
        let v_new = a * x;

        // calcula a aproximação do lambda
        let lambda_new = x.dot(&v_new);

        // checa convergência
        if (lambda_new - lambda).abs() / lambda_new.abs() <= tolerancia {
            return (lambda_new, x);
        }

        // passa pra próxima iteração
        lambda = lambda_new;
        v = v_new;
    }
}

pub fn potencia_inversa<const N: usize>(
    a: Matrix<f64, N, N>,
    v0: VecN<f64, N>,
    tolerancia: f64,
) -> (f64, VecN<f64, N>) {
    let mut lambda = 0.0;
    let mut v = v0;

    loop {
        // calcula o valor de x1 (aproximação normalizada)
        let x = v.normalized();

        // Calcula v_k novo de acordo com a decomposição L,U da matriz A
        // (tudo implementado dentro do solve_lu)
        let v_new = a.solve_lu(&x);

        let lambda_new = x.dot(&v_new);

        if (lambda_new - lambda).abs() / lambda_new.abs() <= tolerancia {
            return (1.0 / lambda_new, x);
        }

        lambda = lambda_new;
        v = v_new;
    }
}

pub fn potencia_deslocamento<const N: usize>(
    a: Matrix<f64, N, N>,
    v0: VecN<f64, N>,
    tolerancia: f64,
    deslocamento: f64
) -> (f64, VecN<f64, N>) {
    // calcula a matriz Â (representada aq por a2)
    let a2 = a - (Matrix::identity() * deslocamento);
    // autovalor e autovetor de Â por potencia inversa
    let (li, xi) = potencia_inversa(a2, v0, tolerancia);

    // Calcula o lambda mais próximo do deslocamento
    let lambda = li + deslocamento;
    let x = xi;

    (lambda, x)
}
