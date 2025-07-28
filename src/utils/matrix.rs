use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Matrix<T, const N: usize, const M: usize> {
    pub data: [[T; M]; N],
}

impl<T, const N: usize, const M: usize> Matrix<T, N, M>
where
    T: Copy + Default,
{
    pub const fn new(data: [[T; M]; N]) -> Self {
        Self { data }
    }

    pub fn zero() -> Self {
        Self {
            data: [[T::default(); M]; N],
        }
    }

    pub fn transpose(&self) -> Matrix<T, M, N> {
        let mut result = [[T::default(); N]; M];
        for i in 0..N {
            for j in 0..M {
                result[j][i] = self.data[i][j];
            }
        }
        Matrix::new(result)
    }
}

impl<T, const N: usize> Matrix<T, N, N>
where
    T: Copy + Default + From<u8> + PartialEq,
{
    pub fn identity() -> Self {
        let mut m = Self::zero();
        let one = T::from(1);
        for i in 0..N {
            m.data[i][i] = one;
        }
        m
    }
}

// Index
impl<T, const N: usize, const M: usize> Index<usize> for Matrix<T, N, M> {
    type Output = [T; M];
    fn index(&self, idx: usize) -> &Self::Output {
        &self.data[idx]
    }
}

impl<T, const N: usize, const M: usize> IndexMut<usize> for Matrix<T, N, M> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.data[idx]
    }
}

// Display
impl<T: fmt::Display, const N: usize, const M: usize> fmt::Display for Matrix<T, N, M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..N {
            write!(f, "[")?;
            for j in 0..M {
                write!(f, "{:>8}", self.data[i][j])?;
                if j + 1 != M {
                    write!(f, ", ")?;
                }
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

// Element-wise addition
impl<T, const N: usize, const M: usize> Add for Matrix<T, N, M>
where
    T: Copy + Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut result = self;
        for i in 0..N {
            for j in 0..M {
                result.data[i][j] = result.data[i][j] + rhs.data[i][j];
            }
        }
        result
    }
}

impl<T, const N: usize, const M: usize> AddAssign for Matrix<T, N, M>
where
    T: Copy + AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..N {
            for j in 0..M {
                self.data[i][j] += rhs.data[i][j];
            }
        }
    }
}

// Element-wise subtraction
impl<T, const N: usize, const M: usize> Sub for Matrix<T, N, M>
where
    T: Copy + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut result = self;
        for i in 0..N {
            for j in 0..M {
                result.data[i][j] = result.data[i][j] - rhs.data[i][j];
            }
        }
        result
    }
}

impl<T, const N: usize, const M: usize> SubAssign for Matrix<T, N, M>
where
    T: Copy + SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..N {
            for j in 0..M {
                self.data[i][j] -= rhs.data[i][j];
            }
        }
    }
}

// Scalar multiplication
impl<T, const N: usize, const M: usize> Mul<T> for Matrix<T, N, M>
where
    T: Copy + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        let mut result = self;
        for i in 0..N {
            for j in 0..M {
                result.data[i][j] = result.data[i][j] * rhs;
            }
        }
        result
    }
}

impl<T, const N: usize, const M: usize> MulAssign<T> for Matrix<T, N, M>
where
    T: Copy + MulAssign,
{
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..N {
            for j in 0..M {
                self.data[i][j] *= rhs;
            }
        }
    }
}

// Scalar division
impl<T, const N: usize, const M: usize> Div<T> for Matrix<T, N, M>
where
    T: Copy + Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        let mut result = self;
        for i in 0..N {
            for j in 0..M {
                result.data[i][j] = result.data[i][j] / rhs;
            }
        }
        result
    }
}

impl<T, const N: usize, const M: usize> DivAssign<T> for Matrix<T, N, M>
where
    T: Copy + DivAssign,
{
    fn div_assign(&mut self, rhs: T) {
        for i in 0..N {
            for j in 0..M {
                self.data[i][j] /= rhs;
            }
        }
    }
}

// Negation
impl<T, const N: usize, const M: usize> Neg for Matrix<T, N, M>
where
    T: Copy + Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self {
        let mut result = self;
        for i in 0..N {
            for j in 0..M {
                result.data[i][j] = -result.data[i][j];
            }
        }
        result
    }
}

impl<T, const N: usize, const M: usize, const K: usize> Mul<Matrix<T, M, K>> for Matrix<T, N, M>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T>,
{
    type Output = Matrix<T, N, K>;

    fn mul(self, rhs: Matrix<T, M, K>) -> Self::Output {
        let mut result = Matrix::<T, N, K>::zero();
        for i in 0..N {
            for j in 0..K {
                let mut sum = T::default();
                for k in 0..M {
                    sum = sum + self.data[i][k] * rhs.data[k][j];
                }
                result.data[i][j] = sum;
            }
        }
        result
    }
}

use super::VecN;

impl<T, const N: usize, const M: usize> Mul<VecN<T, M>> for Matrix<T, N, M>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T>,
{
    type Output = VecN<T, N>;

    fn mul(self, rhs: VecN<T, M>) -> VecN<T, N> {
        let mut result = VecN {
            data: [T::default(); N],
        };

        for i in 0..N {
            let mut sum = T::default();
            for j in 0..M {
                sum = sum + self.data[i][j] * rhs.data[j];
            }
            result.data[i] = sum;
        }

        result
    }
}

impl<T, const N: usize, const M: usize> Mul<Matrix<T, N, M>> for VecN<T, N>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T>,
{
    type Output = VecN<T, M>;

    fn mul(self, rhs: Matrix<T, N, M>) -> VecN<T, M> {
        let mut result = VecN {
            data: [T::default(); M],
        };

        for j in 0..M {
            let mut sum = T::default();
            for i in 0..N {
                sum = sum + self.data[i] * rhs.data[i][j];
            }
            result.data[j] = sum;
        }

        result
    }
}
