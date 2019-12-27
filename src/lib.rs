#![no_std]
#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(specialization)]
//#![feature(avx512_target_feature)]
//#![feature(sse4a_target_feature)]

use core::ops::*;
use core::iter::{FromIterator, IntoIterator};

struct Vector<const N: usize> {
    pub(crate) inner: [f32; N]
}

impl<const N: usize> FromIterator<f32> for Vector<N> {
    fn from_iter<I: IntoIterator<Item = f32>>(iter: I) -> Self {
        // todo: use fixed size iterator
        let mut new = Self::default();
        let mut entries = 0;
        for i in iter.into_iter().take(N) {
            new.inner[entries] = i;
            entries += 1;
        }
        new
    }
}

impl<const N: usize> Default for Vector<N> {
    fn default() -> Self {
        let mut x = core::mem::MaybeUninit::<[f32;N]>::uninit();
        let base = x.as_mut_ptr() as * mut f32;
        for offset in 0..N {
            unsafe {
                // N is x.len, can't shoot over
                base.add(offset).write(f32::default());
            }
        }
        // all N elements have been set to their default
        let inner = unsafe { x.assume_init() };
        Vector { inner }
    }
}


impl<const N: usize> Add for &Vector<N> {
    type Output = Vector<N>;
    fn add(self, other: Self) -> Vector<N> {
        self.inner.iter().zip(other.inner.iter())
            .map(|(s,o)| Add::add(s, o)).collect()
    }
}

impl<const N: usize> Sub for &Vector<N> {
    type Output = Vector<N>;
    fn sub(self, other: Self) -> Vector<N> {
        self.inner.iter().zip(other.inner.iter())
            .map(|(s,o)| Sub::sub(s, o)).collect()
    }
}

impl<const N: usize> Mul for &Vector<N> {
    type Output = Vector<N>;
    fn mul(self, other: Self) -> Vector<N> {
        self.inner.iter().zip(other.inner.iter())
            .map(|(s,o)| Mul::mul(s, o)).collect()
    }
}

impl<const N: usize> Div for &Vector<N> {
    type Output = Vector<N>;
    fn div(self, other: Self) -> Vector<N> {
        self.inner.iter().zip(other.inner.iter())
            .map(|(s,o)| Div::div(s, o)).collect()
    }
}

impl<const N: usize> Vector<N> {
    pub fn as_matrix(self) -> Matrix<1, {N}> {
        Matrix { inner: [self] }
    }
}

/// column major
struct Matrix<const N: usize, const M: usize> {
    inner: [Vector<M>; N]
}

impl<const N: usize, const M: usize> FromIterator<Vector<M>> for Matrix<N,M> {
    fn from_iter<I: IntoIterator<Item = Vector<M>>>(iter: I) -> Self {
        // todo: use fixed size iterator
        let mut new = Self::default();
        let mut entries = 0;
        for i in iter.into_iter().take(N) {
            new.inner[entries] = i;
            entries += 1;
        }
        new
    }
}

impl<const N: usize, const M: usize> Default for Matrix<N, M> {
    fn default() -> Self {
        let mut x = core::mem::MaybeUninit::<[Vector<M>;N]>::uninit();
        let base = x.as_mut_ptr() as * mut Vector<M>;
        for offset in 0..N {
            unsafe {
                // N is x.len, can't shoot over
                base.add(offset).write(Vector::default());
            }
        }
        // all N elements have been set to their default
        let inner = unsafe { x.assume_init() };
        Self { inner }
    }
}

impl<const N: usize, const M: usize> Add for &Matrix<N, M> {
    type Output = Matrix<N, M>;
    fn add(self, other: Self) -> Matrix<N, M> {
        self.inner.iter().zip(other.inner.iter())
            .map(|(s,o)| Add::add(s, o))
            .collect()
    }
}

impl<const N: usize, const M: usize, const O: usize> Mul<&Matrix<M, O>> for &Matrix<N, M> {
    type Output = Matrix<N, O>;
    fn mul(self, other: &Matrix<M, O>) -> Matrix<N, O> {
        todo!()
    }
}


#[cfg(test)]
const TESTLEN: usize = 777usize;

#[test]
fn default_is_default() {
    let m = Vector::<TESTLEN>::default();
    for i in 0..TESTLEN {
        assert_eq!(m.inner[i], f32::default());
    }
}
