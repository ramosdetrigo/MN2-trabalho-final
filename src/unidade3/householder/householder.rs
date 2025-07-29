use crate::utils::Matrix;

pub fn householder_coluna<const N: usize>(
    a: &Matrix<f64, N, N>,
    i: usize,
) -> Matrix<f64, N, N> {
    let mut w = [0.0; N];
    let mut w_prime = [0.0; N];

    // Step 2: Copia os elementos abaixo da diagonal da coluna i
    for k in i + 1..N {
        w[k] = a[k][i];
    }

    // Step 3: Norma de w
    let lw = w.iter().map(|x| x * x).sum::<f64>().sqrt();

    // Step 4: Sinal oposto ao w[i+1]
    let sinal_oposto = if w[i + 1] < 0.0 { 1.0 } else { -1.0 };
    w_prime[i + 1] = sinal_oposto * lw;

    // Step 5: N = w - w'
    let mut n = [0.0; N];
    for k in 0..N {
        n[k] = w[k] - w_prime[k];
    }

    // Step 6: Normaliza n
    let norm_n = n.iter().map(|x| x * x).sum::<f64>().sqrt();
    if norm_n == 0.0 {
        return Matrix::<f64, N, N>::identity();
    }
    for k in 0..N {
        n[k] /= norm_n;
    }

    // Step 7: H = I - 2 * n * n^T
    let mut h = Matrix::<f64, N, N>::identity();
    for r in 0..N {
        for c in 0..N {
            h[r][c] -= 2.0 * n[r] * n[c];
        }
    }

    h
}

pub fn loop_householder<const N: usize>(
    a: Matrix<f64, N, N>,
) -> (Matrix<f64, N, N>, Matrix<f64, N, N>) {
    let mut h_total = Matrix::<f64, N, N>::identity();
    let mut a_k = a;

    for i in 0..(N - 2) {
        let h_i = householder_coluna(&a_k, i);

        // A^(i+1) = H_i * A^(i) * H_i
        let a_next = h_i * a_k * h_i;

        // Atualiza A e acumula H
        a_k = a_next;
        h_total = h_total * h_i;
    }

    (a_k, h_total)
}
