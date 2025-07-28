use crate::utils::Matrix;

/// Decomposição QR via Gram-Schmidt clássico (versão limpa)
/// Retorna matrizes Q (ortogonal) e R (triangular superior) tal que A = Q * R
pub fn qr_decomposicao_classica<const N: usize>(
    a: Matrix<f64, N, N>,
) -> (Matrix<f64, N, N>, Matrix<f64, N, N>) {
    let mut q = Matrix::<f64, N, N>::zero();
    let mut r = Matrix::<f64, N, N>::zero();

    for j in 0..N {
        let mut u = [0.0; N];
        for i in 0..N {
            u[i] = a[i][j];
        }

        // Projeta o vetor u sobre as colunas anteriores de Q
        for k in 0..j {
            let mut dot = 0.0;
            for i in 0..N {
                dot += q[i][k] * a[i][j]; // produto interno entre q_k e a_j
            }
            r[k][j] = dot;
            for i in 0..N {
                u[i] -= dot * q[i][k]; // subtrai projeção de u
            }
        }

        // Normaliza o vetor u para formar a j-ésima coluna de Q
        let norm = u.iter().map(|x| x * x).sum::<f64>().sqrt();
        r[j][j] = norm;
        for i in 0..N {
            q[i][j] = u[i] / norm;
        }
    }

    (q, r)
}

/// Método QR iterativo para encontrar autovalores e autovetores
/// A matriz A_k converge para uma forma quase diagonal
/// Os autovalores aparecem na diagonal de A_k, e as colunas da matriz P acumulada são os autovetores
pub fn metodo_qr<const N: usize>(
    a: Matrix<f64, N, N>,
    epsilon: f64,
    max_iter: usize,
) -> (Matrix<f64, N, N>, [f64; N]) {
    let mut a_k = a;                              // Matriz que será atualizada
    let mut p = Matrix::<f64, N, N>::identity();  // Acumulador dos Q_k
    let mut erro = 100.0;                         
    let mut iter = 0;                             

    // Itera até convergir (erro abaixo de epsilon) ou atingir o número máximo de iterações
    while erro > epsilon && iter < max_iter {
        let (q, r) = qr_decomposicao_classica(a_k); // Passo 1: Decomposição QR
        a_k = r * q;                                // Passo 2: Transformação de similaridade A_k = R * Q
        p = p * q;                                  // Passo 3: Acumula os autovetores: P_k = P_{k-1} * Q_k

        // Passo 4: Verifica convergência com a soma dos quadrados dos elementos abaixo da diagonal
        erro = 0.0;
        for i in 1..N {
            for j in 0..i {
                erro += a_k[i][j].powi(2);
            }
        }

        iter += 1;
    }

    // Os autovalores estão na diagonal de A_k
    let mut autovalores = [0.0; N];
    for i in 0..N {
        autovalores[i] = a_k[i][i];
    }

    // Retorna os autovetores acumulados (colunas de P) e autovalores
    (p, autovalores)
}
