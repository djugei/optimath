//! This module defines traits that can be implemented by all vector representations
//! operations then only need to be defined between each vector representation and this trait(s)
//! instead of between all combinations of vector reperesentation
//! i.e. only N implementations instead of N*N
//! with GAT i could bring that down to 1

/// implement this on types that can be indexed into that have a size known at compile time
///
/// mutability: just implement this twice, with &E &mut E as T
///
/// unsafety: calling .i(x) with x < N must successfully return T
///
/// unsafety: .i(x) and .i(y) return different objects when x != y.
/// i.e. they do not alias.
pub unsafe trait ConstIndex<T, const N: usize> {
	fn i(self, index: usize) -> T;
}

use crate::Vector;
unsafe impl<'a, T, const N: usize> ConstIndex<&'a T, N> for &'a Vector<T, N> {
	fn i(self, index: usize) -> &'a T { &self.inner[index] }
}

// lets just hope for the optimizer. only added this for templatemetamath to be usable.
// could think about changing the Add impls so &Vector returning T is accepted
// and only impl ConstIndex for Copy types
unsafe impl<'a, T, const N: usize> ConstIndex<T, N> for Vector<T, N>
where
	T: Copy,
{
	fn i(self, index: usize) -> T { *&self.inner[index] }
}

unsafe impl<'a, T, const N: usize> ConstIndex<&'a mut T, N> for &'a mut Vector<T, N> {
	fn i(self, index: usize) -> &'a mut T { &mut self.inner[index] }
}

use crate::VectorView;
unsafe impl<'a, T, const M: usize, const N: usize> ConstIndex<&'a T, M>
	for VectorView<'a, T, M, N>
{
	fn i(self, index: usize) -> &'a T {
		let row = &self.matrix[index];
		&row[self.row]
	}
}

unsafe impl<'a, T, const M: usize, const N: usize> ConstIndex<VectorView<'a, T, M, N>, N>
	for crate::TransposedMatrixView<'a, T, M, N>
{
	fn i(self, index: usize) -> VectorView<'a, T, M, N> {
		debug_assert!(index <= M);
		VectorView {
			row: index,
			matrix: self.matrix,
		}
	}
}

pub struct ConstIterator<T, C: ConstIndex<T, N>, const N: usize> {
	pub(crate) pos: usize,
	pub(crate) content: C,
	pub(crate) marker: core::marker::PhantomData<T>,
}

impl<T, C: ConstIndex<T, N> + Copy, const N: usize> Iterator for ConstIterator<T, C, N> {
	type Item = T;
	fn next(&mut self) -> Option<T> {
		if self.pos < N {
			let ret = self.content.i(self.pos);
			self.pos += 1;
			Some(ret)
		} else {
			None
		}
	}
}

/* im reasonably sure this could be implemented with GAT
 * right now it tells me that T and N are unconstrained, because C is not generic over them
 * this is exactly what GAT provides
impl<C, T, const N: usize> IntoIterator for C
where
	C: ConstIndex<T, N> + Copy,
{
	type Item = T;
	type IntoIter = ConstIterator<T, Self, N>;

	fn into_iter(self) -> Self::IntoIter { ConstIterator { pos: 0, content: self, marker: Default::default() }
}
*/

impl<C, T, const N: usize> From<C> for ConstIterator<T, C, N>
where
	C: ConstIndex<T, N>,
{
	fn from(content: C) -> Self {
		Self {
			pos: 0,
			content,
			marker: Default::default(),
		}
	}
}

pub struct ConstIteratorMut<'a, T, C, const N: usize> {
	pos: usize,
	content: *mut C,
	marker: core::marker::PhantomData<&'a mut T>,
}

impl<'a, T: 'a, C: 'a, const N: usize> Iterator for ConstIteratorMut<'a, T, C, N>
where
	&'a mut C: ConstIndex<&'a mut T, N>,
{
	type Item = &'a mut T;
	fn next(&mut self) -> Option<&'a mut T> {
		if self.pos < N {
			// this is to work around lifetime issues (but its like legit)
			// we can't just do this the direct way with self.content being &'a mut C
			// because then content.i(x) would return &'a mut T
			// but we need &a mut T, i.e. living as long/not outliving as &mut self
			// can't express that concept though cause its an anonymous lifetime
			// and changing that would break the iterator api.
			// the problem that would be occuring is that calling .next() twice and storing
			// the result might return the same reference twice leading to mutable
			// aliasing. we can guarantee that not to happen though, because the unsafe
			// trait ConstIndex provides the method .i(x) which is guaranteed to not alias for
			// different indices. as we increment the index on each iteration we never
			// alias.
			//
			// additionally there are tests that are run with miri just to make sure
			let content: &mut C = unsafe { core::mem::transmute(self.content) };
			let ret = content.i(self.pos);
			self.pos += 1;
			Some(ret)
		} else {
			None
		}
	}
}

impl<'a, T, C: 'a, const N: usize> From<&'a mut C> for ConstIteratorMut<'a, T, C, N>
where
	&'a mut C: ConstIndex<&'a mut T, N>,
{
	fn from(content: &'a mut C) -> Self {
		Self {
			pos: 0,
			content: content as *mut _,
			marker: Default::default(),
		}
	}
}

#[test]
fn const_iter() {
	use crate::Vector;
	use rand::{thread_rng, Rng};
	let mut rng = thread_rng();
	let a: Vector<f32, 40> = rng.gen();
	let iter: ConstIterator<&f32, _, 40> = ConstIterator {
		pos: 0,
		content: &a,
		marker: Default::default(),
	};

	let _s: f32 = iter.sum();
}
