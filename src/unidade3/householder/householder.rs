use crate::utils::Matrix;

// Implementado de acordo com o PDF da aula 20
pub fn passo_i_householder<const N: usize>(a: &Matrix<f64, N, N>, i: usize) -> Matrix<f64, N, N> {
    let mut w = [0.0; N];
    let mut wp = [0.0; N];

    for j in (i + 1)..N {
        w[j] = a.data[j][i];
    }

    let lw = w.iter().map(|x| x * x).sum::<f64>().sqrt();
    wp[i + 1] = lw;

    let mut n = [0.0; N];
    for j in 0..N {
        n[j] = w[j] - wp[j];
    }

    let norm_n = n.iter().map(|x| x * x).sum::<f64>().sqrt();
    if norm_n == 0.0 {
        return Matrix::identity();
    }

    for j in 0..N {
        n[j] /= norm_n;
    }

    let mut h = Matrix::<f64, N, N>::identity();
    for r in 0..N {
        for c in 0..N {
            h.data[r][c] -= 2.0 * n[r] * n[c];
        }
    }

    h
}

pub fn loop_householder<const N: usize>(a: Matrix<f64, N, N>) -> (Matrix<f64, N, N>, Matrix<f64, N, N>) {
    let mut h_total = Matrix::<f64, N, N>::identity();
    let mut a_step = a;

    for i in 0..(N - 2) {
        let h_i = passo_i_householder(&a_step, i);

        // A_{i+1} = H_i A_i H_i
        a_step = (h_i * a_step) * h_i;

        // Acumula as transformações
        h_total = h_total * h_i;
    }

    (a_step, h_total)
}
