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
pub unsafe trait ConstIndex<T, const N: usize> {
	fn i(self, index: usize) -> T;
}

use crate::Vector;
unsafe impl<'a, T, const N: usize> ConstIndex<&'a T, N> for &'a Vector<T, N> {
	fn i(self, index: usize) -> &'a T { &self.inner[index] }
}

unsafe impl<'a, T, const N: usize> ConstIndex<&'a mut T, N> for &'a mut Vector<T, N> {
	fn i(self, index: usize) -> &'a mut T { &mut self.inner[index] }
}

use crate::VectorView;
unsafe impl<'a, T, const M: usize, const N: usize> ConstIndex<&'a T, N>
	for VectorView<'a, T, M, N>
{
	fn i(self, index: usize) -> &'a T {
		let row = &self.matrix[index];
		&row[self.row]
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

/* this produces some weird error about mismatching lifetimes
 *   note: expected  consts::ConstIndex<&'a mut T, N>
 *            found  consts::ConstIndex<&'a mut T, N>
 *  which, at least to me, seem to be the exact same thing
pub struct ConstIteratorMut<'a, T, C, const N: usize> {
	pos: usize,
	content: &'a mut C,
	marker: core::marker::PhantomData<T>,
}

impl<'a, T: 'a, C, const N: usize> Iterator for ConstIteratorMut<'a, T, C, N>
where
	&'a mut C: ConstIndex<&'a mut T, N>,
{
	type Item = &'a mut T;
	fn next(&mut self) -> Option<&mut T> {
		if self.pos < N {
			let ret = self.content.i(self.pos);
			self.pos += 1;
			Some(ret)
		} else {
			None
		}
	}
}
*/

#[test]
fn const_iter() {
	use crate::Vector;
	use rand::{thread_rng, Rng};
	let mut rng = thread_rng();
	let a: Vector<f32, 40> = rng.gen();
	let iter = ConstIterator {
		pos: 0,
		content: &a,
		marker: Default::default(),
	};

	let _s: f32 = iter.sum();
}
