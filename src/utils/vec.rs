use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A fixed-length vector of `N` components of type `T`
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct VecN<T, const N: usize> {
    pub data: [T; N],
}

impl<T, const N: usize> VecN<T, N> {
    /// Construct from array
    #[inline]
    pub const fn from_array(data: [T; N]) -> Self {
        Self { data }
    }
}

//
// Constructors for numeric types
//
impl<T, const N: usize> VecN<T, N>
where
    T: Copy + Default,
{
    /// Zero vector
    #[inline]
    pub fn zero() -> Self {
        Self {
            data: [T::default(); N],
        }
    }
}

impl<T, const N: usize> VecN<T, N>
where
    T: Copy,
{
    /// Broadcast constructor: all components = `x`
    #[inline]
    pub fn splat(x: T) -> Self {
        Self { data: [x; N] }
    }
}

//
// Indexing
//
impl<T, const N: usize> std::ops::Index<usize> for VecN<T, N> {
    type Output = T;
    fn index(&self, idx: usize) -> &T {
        &self.data[idx]
    }
}

impl<T, const N: usize> std::ops::IndexMut<usize> for VecN<T, N> {
    fn index_mut(&mut self, idx: usize) -> &mut T {
        &mut self.data[idx]
    }
}

//
// Element-wise Add/Sub
//
impl<T, const N: usize> Add for VecN<T, N>
where
    T: Copy + Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut out = self;
        for i in 0..N {
            out.data[i] = out.data[i] + rhs.data[i];
        }
        out
    }
}

impl<T, const N: usize> AddAssign for VecN<T, N>
where
    T: Copy + AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self.data[i] += rhs.data[i];
        }
    }
}

impl<T, const N: usize> Sub for VecN<T, N>
where
    T: Copy + Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut out = self;
        for i in 0..N {
            out.data[i] = out.data[i] - rhs.data[i];
        }
        out
    }
}

impl<T, const N: usize> SubAssign for VecN<T, N>
where
    T: Copy + SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self.data[i] -= rhs.data[i];
        }
    }
}

//
// Scalar mul/div
//
impl<T, const N: usize> Mul<T> for VecN<T, N>
where
    T: Copy + Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        let mut out = self;
        for i in 0..N {
            out.data[i] = out.data[i] * rhs;
        }
        out
    }
}

impl<T, const N: usize> MulAssign<T> for VecN<T, N>
where
    T: Copy + MulAssign,
{
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..N {
            self.data[i] *= rhs;
        }
    }
}

impl<T, const N: usize> Div<T> for VecN<T, N>
where
    T: Copy + Div<Output = T>,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        let mut out = self;
        for i in 0..N {
            out.data[i] = out.data[i] / rhs;
        }
        out
    }
}

impl<T, const N: usize> DivAssign<T> for VecN<T, N>
where
    T: Copy + DivAssign,
{
    fn div_assign(&mut self, rhs: T) {
        for i in 0..N {
            self.data[i] /= rhs;
        }
    }
}

//
// Negation
//
impl<T, const N: usize> Neg for VecN<T, N>
where
    T: Copy + Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self {
        let mut out = self;
        for i in 0..N {
            out.data[i] = -out.data[i];
        }
        out
    }
}

//
// Dot product, length, normalization
//
impl<T, const N: usize> VecN<T, N>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T> + Into<f64> + From<f64>,
{
    /// Dot product ∑ self[i] * rhs[i]
    pub fn dot(&self, rhs: &Self) -> T {
        let mut sum = T::default();
        for i in 0..N {
            sum = sum + (self.data[i] * rhs.data[i]);
        }
        sum
    }

    /// Squared length, as T
    pub fn length_squared(&self) -> T {
        self.dot(self)
    }

    /// Euclidean length, as f64
    pub fn length(&self) -> f64 {
        let sq_t: T = self.length_squared();
        let sq_f: f64 = sq_t.into();
        sq_f.sqrt()
    }

    /// Normalized vector (requires T::from(f64))
    pub fn normalized(&self) -> Self {
        let len = self.length();
        let mut out = *self;
        for i in 0..N {
            // convert component → f64, divide, convert back → T
            let comp_f: f64 = self.data[i].into();
            out.data[i] = T::from(comp_f / len);
        }
        out
    }
}

//
// Display
//
impl<T: fmt::Display, const N: usize> fmt::Display for VecN<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..N {
            write!(f, "{}", self.data[i])?;
            if i + 1 != N {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}
