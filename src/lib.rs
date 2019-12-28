#![no_std]
#![allow(incomplete_features)]
#![feature(const_generics)]
//#![feature(specialization)]
#![feature(trait_alias)]
#![feature(maybe_uninit_extra)]
//#![feature(avx512_target_feature)]
//#![feature(sse4a_target_feature)]

use core::iter::{FromIterator, IntoIterator};
use core::mem::MaybeUninit;
use core::ops::*;

pub trait Math =
    Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Sized;

pub trait FullMath = Math;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Vector<T: FullMath, const N: usize> {
    pub(crate) inner: [T; N],
}

// basic methods and traits

impl<T: FullMath, const N: usize> Vector<T, N> {
    fn uninit_inner() -> MaybeUninit<[T; N]> {
        MaybeUninit::uninit()
    }
    pub fn next_dimension(self) -> Vector<Self, 1> {
        Vector { inner: [self] }
    }
}

impl<T: FullMath, const N: usize> FromIterator<T> for Vector<T, N> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let mut inner = Self::uninit_inner();
        let base = inner.as_mut_ptr() as *mut T;
        for offset in 0..N {
            if let Some(element) = iter.next() {
                unsafe {
                    base.add(offset).write(element);
                }
            } else {
                panic!("not enough elements");
            }
        }
        iter.next().map(|_| panic!("too many elements"));
        let inner = unsafe { inner.assume_init() };
        Self { inner }
    }
}

// iter stuff just required cause impls on arrays are limited to 32 elements (for no reason)
pub struct IntoIter<T: FullMath, const N: usize> {
    pos: usize,
    data: [MaybeUninit<T>; N],
}

impl<T: FullMath, const N: usize> IntoIter<T, N> {
    fn new(vector: Vector<T, N>) -> Self {
        let data = unsafe {
            let data =
                core::ptr::read(&vector.inner as *const [T; N] as *const [MaybeUninit<T>; N]);
            core::mem::forget(vector);
            data
        };
        IntoIter { pos: 0, data }
    }
}

impl<T: FullMath, const N: usize> Iterator for IntoIter<T, N> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.pos == N {
            None
        } else {
            let out = unsafe { self.data.get_unchecked(self.pos).read() };
            self.pos += 1;
            Some(out)
        }
    }
}

impl<T: FullMath, const N: usize> Drop for IntoIter<T, N> {
    fn drop(&mut self) {
        let range = self.pos..N;
        for offset in range {
            self.pos = offset;
            unsafe { self.data.get_unchecked(self.pos).read() };
        }
    }
}

impl<T: FullMath, const N: usize> IntoIterator for Vector<T, N> {
    type Item = T;
    type IntoIter = IntoIter<T, { N }>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

impl<T: FullMath + Default, const N: usize> Default for Vector<T, N> {
    fn default() -> Self {
        let mut x = Self::uninit_inner();
        let base = x.as_mut_ptr() as *mut T;
        for offset in 0..N {
            unsafe {
                // N is x.len, can't shoot over
                base.add(offset).write(T::default());
            }
        }
        // all N elements have been set to their default
        let inner = unsafe { x.assume_init() };
        Vector { inner }
    }
}

// reference operations, getting recrursive problems on where clause

/*
impl<'a, T: FullMath, const N: usize> Add for &'a Vector<T, N>
where
    &'a T: Add,
{
    type Output = Vector<T, N>;
    fn add(self, other: Self) -> Vector<T, N> {
        self.inner
            .iter()
            .zip(other.inner.iter())
            .map(|(s, o)| s + o)
            .collect()
    }
}

impl<T: FullMath, const N: usize> Sub for &Vector<T, N> {
    type Output = Vector<T, N>;
    fn sub(self, other: Self) -> Vector<T, N> {
        self.inner
            .iter()
            .zip(other.inner.iter())
            .map(|(s, o)| s-o)
            .collect()
    }
}

impl<T: FullMath, const N: usize> Mul for &Vector<T, N> {
    type Output = Vector<T, N>;
    fn mul(self, other: Self) -> Vector<T, N> {
        self.inner
            .iter()
            .zip(other.inner.iter())
            .map(|(s, o)| s * o
            .collect()
    }
}

impl<T: FullMath, const N: usize> Div for &Vector<T, N> {
    type Output = Vector<T, N>;
    fn div(self, other: Self) -> Vector<T, N> {
        self.inner
            .iter()
            .zip(other.inner.iter())
            .map(|(s, o)| s / o)
            .collect()
    }
}
*/
// direct operations

impl<T: FullMath, const N: usize> Add for Vector<T, N> {
    type Output = Vector<T, N>;
    fn add(self, other: Self) -> Vector<T, N> {
        self.into_iter()
            .zip(other.into_iter())
            .map(|(s, o)| s + o)
            .collect()
    }
}

impl<T: FullMath, const N: usize> Sub for Vector<T, N> {
    type Output = Vector<T, N>;
    fn sub(self, other: Self) -> Vector<T, N> {
        self.into_iter()
            .zip(other.into_iter())
            .map(|(s, o)| s - o)
            .collect()
    }
}

impl<T: FullMath, const N: usize> Mul for Vector<T, N> {
    type Output = Vector<T, N>;
    fn mul(self, other: Self) -> Vector<T, N> {
        self.into_iter()
            .zip(other.into_iter())
            .map(|(s, o)| s * o)
            .collect()
    }
}

impl<T: FullMath, const N: usize> Div for Vector<T, N> {
    type Output = Vector<T, N>;
    fn div(self, other: Self) -> Vector<T, N> {
        self.into_iter()
            .zip(other.into_iter())
            .map(|(s, o)| s / o)
            .collect()
    }
}

pub type Matrix<T, const M: usize, const N: usize> = Vector<Vector<T, N>, M>;

#[cfg(test)]
const TESTLEN: usize = 777usize;

#[test]
fn default_is_default() {
    let m = Vector::<f32, TESTLEN>::default();
    for i in 0..TESTLEN {
        assert_eq!(m.inner[i], f32::default());
    }
}

#[test]
fn operations() {
    let a: Vector<f32, TESTLEN> = (0..TESTLEN).map(|x| x as f32).collect();
    let b: Vector<f32, TESTLEN> = (1..{ TESTLEN + 1 }).map(|x| x as f32).collect();

    let add = a + b;
    let sub = a - b;
    let mul = a * b;
    let div = a / b;

    for i in 0..TESTLEN {
        assert_eq!(a.inner[i] + b.inner[i], add.inner[i]);
        assert_eq!(a.inner[i] - b.inner[i], sub.inner[i]);
        assert_eq!(a.inner[i] * b.inner[i], mul.inner[i]);
        assert_eq!(a.inner[i] / b.inner[i], div.inner[i]);
    }
}
