use core::ops::*;
use core::mem::MaybeUninit;
use core::iter::{FromIterator, IntoIterator};

pub trait Math =
    Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Sized;

pub trait SelfMath = AddAssign + SubAssign + MulAssign + DivAssign + Sized;

pub trait FullMath = Math; // + SelfMath;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Vector<T: FullMath, const N: usize> {
    pub(crate) inner: [T; N],
}

/// Matrix is column-major
pub type Matrix<T, const M: usize, const N: usize> = Vector<Vector<T, N>, M>;

impl<T: FullMath, const N: usize> Vector<T, N> {
    pub(crate) fn uninit_inner() -> MaybeUninit<[T; N]> {
        MaybeUninit::uninit()
    }
    pub fn ascend(self) -> Vector<Self, 1> {
        Vector { inner: [self] }
    }
}

impl<T: FullMath, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        &self.inner[index]
    }
}

impl<T: FullMath, const N: usize> IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.inner[index]
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

