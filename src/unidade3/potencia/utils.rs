use crate::utils::{Matrix, VecN};

impl<const N: usize> Matrix<f64, N, N> {
    pub fn decomp_lu(&self) -> (Matrix<f64, N, N>, Matrix<f64, N, N>) {
        let mut l = Matrix::identity();
        let mut u = Matrix::zero();

        for i in 0..N {
            for j in i..N {
                let mut sum = 0.0;
                for k in 0..i {
                    sum += l.data[i][k] * u.data[k][j];
                }
                u.data[i][j] = self.data[i][j] - sum;
            }

            for j in i+1..N {
                let mut sum = 0.0;
                for k in 0..i {
                    sum += l.data[j][k] * u.data[k][i];
                }
                l.data[j][i] = (self.data[j][i] - sum) / u.data[i][i];
            }
        }

        (l, u)
    }

    pub fn solve_lu(&self, b: &VecN<f64, N>) -> VecN<f64, N> {
        let (l, u) = self.decomp_lu();

        // 1. Resolve Ly = b
        let mut y = [0.0; N];
        for i in 0..N {
            let mut sum = 0.0;
            for j in 0..i {
                sum += l.data[i][j] * y[j];
            }
            y[i] = b[i] - sum;
        }

        // 2. Resolve Ux = y
        let mut x = [0.0; N];
        for i in (0..N).rev() {
            let mut sum = 0.0;
            for j in i+1..N {
                sum += u.data[i][j] * x[j];
            }
            x[i] = (y[i] - sum) / u.data[i][i];
        }

        VecN::from_array(x)
    }
}
