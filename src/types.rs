//! this module contains some non-math specific methods around Vectors as std-lib arrays are sadly
//! intentionally unusable above the size of 32. When that restriction gets removed most of this
//! module gets obsolete
use core::{
	iter::{FromIterator, IntoIterator},
	mem::MaybeUninit,
	ops::*,
};

/// a const-sized vector of elements, supports all math operations that T does on an
/// element-by-element basis.
///
/// can be iterated over using [.into_iter()](#method.into_iter) on Vector or &Vector
/// can be constructed from iterators using collect().
///
/// is repr(align(16)) for simd
#[repr(align(16))] // todo: choose alignment based on simd-width
#[derive(Copy, Clone)]
pub struct Vector<T, const N: usize> {
	pub(crate) inner: [T; N],
}

/// Matrix is just a type alias for Vector<Vector>.
///
/// Supports some matrix specific maths operations, namely matrix multiplication and transpose
///
/// A Vector<Vector<Vector<...>>> can also be considered a matrix and as such has those operations
/// defined too.
pub type Matrix<T, const M: usize, const N: usize> = Vector<Vector<T, M>, N>;

impl<T, const N: usize> Vector<T, N> {
	pub(crate) fn uninit_inner() -> MaybeUninit<[T; N]> { MaybeUninit::uninit() }
	pub fn ascend(self) -> Vector<Self, 1> { Vector { inner: [self] } }
}

/// now you might be asking: hey djugei, why isn't this function just implemented directly on
/// Vector?
///
/// and that is a very good question!
/// well the answer is that for some reason if i copy this exact code into the types impl and
/// delete the trait i get an infinite recursion error during compilation that i can't explain,
/// feels a lot like spooky action at a distance and that i would consider a compiler bug
pub trait Stupidity<T> {
	//todo: maybe add a try_build function
	fn build_with_fn<F: FnMut(usize) -> T>(f: F) -> Self;
}
impl<T, const N: usize> Stupidity<T> for Vector<T, N> {
	fn build_with_fn<F: FnMut(usize) -> T>(mut f: F) -> Self {
		let mut inner = Self::uninit_inner();
		let base = inner.as_mut_ptr() as *mut T;
		for offset in 0..N {
			let element = f(offset);
			unsafe {
				// can't overshoot cause N is const
				base.add(offset).write(element);
			}
		}

		// has to be initialized at this point because all N elements have been visited
		let inner = unsafe { inner.assume_init() };
		Self { inner }
	}
}

impl<T, const N: usize> Index<usize> for Vector<T, N> {
	type Output = T;
	fn index(&self, index: usize) -> &T { &self.inner[index] }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N> {
	fn index_mut(&mut self, index: usize) -> &mut T { &mut self.inner[index] }
}

impl<T, const N: usize> FromIterator<T> for Vector<T, N> {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		let mut iter = iter.into_iter();
		Self::build_with_fn(|_| iter.next().unwrap())
	}
}

// iter stuff just required cause impls on arrays are limited to 32 elements (for no reason)
pub struct IntoIter<T, const N: usize> {
	pos: usize,
	data: [MaybeUninit<T>; N],
}

impl<T, const N: usize> IntoIter<T, N> {
	// fixme: this is probably rly slow cause of all the copies
	// seems to be optimized out though
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

impl<T, const N: usize> Iterator for IntoIter<T, N> {
	type Item = T;
	fn next(&mut self) -> Option<T> {
		if self.pos == N {
			None
		} else {
			let out = unsafe { self.data.get_unchecked(self.pos).assume_init_read() };
			self.pos += 1;
			Some(out)
		}
	}
}

impl<T, const N: usize> Drop for IntoIter<T, N> {
	fn drop(&mut self) { for _item in self {} }
}

impl<T, const N: usize> IntoIterator for Vector<T, N> {
	type Item = T;
	type IntoIter = IntoIter<T, { N }>;

	fn into_iter(self) -> Self::IntoIter { IntoIter::new(self) }
}

use crate::consts::ConstIterator;

impl<'a, T, const N: usize> IntoIterator for &'a Vector<T, N> {
	type Item = &'a T;
	type IntoIter = ConstIterator<&'a T, &'a Vector<T, N>, N>;

	fn into_iter(self) -> Self::IntoIter { self.into() }
}

impl<T: Default, const N: usize> Default for Vector<T, N> {
	fn default() -> Self { Self::build_with_fn(|_| T::default()) }
}

impl<T: PartialEq, const N: usize> PartialEq for Vector<T, N> {
	fn eq(&self, other: &Self) -> bool { self.into_iter().zip(other).all(|(s, o)| s == o) }
}

impl<T: Eq, const N: usize> Eq for Vector<T, N> {}

use core::fmt::Debug;
impl<T: Debug, const N: usize> Debug for Vector<T, N> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
		f.write_str("Vector<")?;
		Debug::fmt(&N, f)?;
		f.write_str(">[")?;
		for i in self {
			i.fmt(f)?;
			f.write_str(", ")?;
		}
		f.write_str("]")?;
		Ok(())
	}
}

use core::fmt::Display;
impl<T: Display + Debug, const N: usize> Display for Vector<T, N> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
		if f.alternate() {
			Debug::fmt(self, f)?;
		} else {
			f.write_str("Vector[\n")?;
			for i in self {
				Display::fmt(i, f)?;
				f.write_str(",\n")?;
			}
			f.write_str("]")?;
		}
		Ok(())
	}
}
