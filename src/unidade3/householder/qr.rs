use crate::utils::{Matrix, VecN};
use std::f64::consts::PI;

// implementado de acordo com o pdf da aula 22 de 2021

/// Cria uma matriz de rotação de Jacobi Jij para anular o elemento (i,j) da matriz A
fn matriz_jacobi<const N: usize>(a: &Matrix<f64, N, N>, i: usize, j: usize) -> Matrix<f64, N, N> {
    let mut jij = Matrix::<f64, N, N>::identity();
    let eps = 1e-6;

    let a_ij = a[i][j];
    let a_jj = a[j][j];

    if a_ij.abs() <= eps {
        return jij;
    }

    let theta = if a_jj.abs() <= eps {
        if a_ij < 0.0 {
            PI / 2.0
        } else {
            -PI / 2.0
        }
    } else {
        (-a_ij / a_jj).atan()
    };

    let c = theta.cos();
    let s = theta.sin();

    jij[i][i] = c;
    jij[j][j] = c;
    jij[i][j] = s;
    jij[j][i] = -s;

    jij
}

/// Decomposição QR usando rotações de Jacobi (transformações de similaridade)
pub fn decomposicao_qr_jacobi<const N: usize>(a: Matrix<f64, N, N>) -> (Matrix<f64, N, N>, Matrix<f64, N, N>) {
    let mut qt = Matrix::<f64, N, N>::identity(); // QT = J_n(n-1) * ... * J_21 * I
    let mut rvelha = a;

    for j in 0..(N - 1) {
        for i in (j + 1)..N {
            let jij = matriz_jacobi::<N>(&rvelha, i, j);
            let rnova = jij * rvelha;
            rvelha = rnova;
            qt = jij * qt;
        }
    }

    let q = qt.transpose();
    let r = rvelha;

    (q, r)
}

/// Soma dos quadrados dos elementos abaixo da diagonal
fn soma_quadrados_abaixo<const N: usize>(a: &Matrix<f64, N, N>) -> f64 {
    let mut soma = 0.0;
    for i in 1..N {
        for j in 0..i {
            soma += a[i][j].powi(2);
        }
    }
    soma
}

/// Método QR iterativo para autovalores/autovetores
pub fn metodo_qr<const N: usize>(
    mut a: Matrix<f64, N, N>,
    epsilon: f64,
) -> (Matrix<f64, N, N>, VecN<f64, N>) {
    let mut p = Matrix::<f64, N, N>::identity();
    let mut val = soma_quadrados_abaixo(&a);

    while val > epsilon {
        let (q, r) = decomposicao_qr_jacobi(a.clone());
        a = r * q;       // Aₖ₊₁ = R Q
        p = p * q;       // acumula vetores
        val = soma_quadrados_abaixo(&a);
    }

    let mut lamb = VecN::<f64, N>::zero();
    for i in 0..N {
        lamb[i] = a[i][i];
    }

    (p, lamb)
}