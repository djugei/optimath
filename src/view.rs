//! views on underlying vectors
//!
//! basically move trough the data at different strides and offsets
//! currently only transposed matrices and contained flipped vectors
//!
//! the Index trait sucks hard
use crate::{consts::ConstIterator, types::Matrix};

#[derive(Debug)]
pub struct TransposedMatrixView<'a, T, const M: usize, const N: usize> {
	pub(crate) matrix: &'a Matrix<T, N, M>,
}

impl<'a, T, const M: usize, const N: usize> Copy for TransposedMatrixView<'a, T, M, N> {}
impl<'a, T, const M: usize, const N: usize> Clone for TransposedMatrixView<'a, T, M, N> {
	fn clone(&self) -> Self { *self }
}

impl<'a, T: 'a + Clone, const M: usize, const N: usize> TransposedMatrixView<'a, T, M, N> {
	pub fn materialize(self) -> Matrix<T, M, N> {
		self.into_iter()
			.map(IntoIterator::into_iter)
			.map(Iterator::cloned)
			.map(Iterator::collect)
			.collect()
	}
}

impl<'a, T, const M: usize, const N: usize> IntoIterator for TransposedMatrixView<'a, T, M, N> {
	type Item = VectorView<'a, T, M, N>;
	type IntoIter = ConstIterator<Self::Item, Self, N>;

	fn into_iter(self) -> Self::IntoIter { self.into() }
}

#[derive(Debug)]
pub struct VectorView<'a, T, const M: usize, const N: usize> {
	pub(crate) row: usize,
	pub(crate) matrix: &'a Matrix<T, N, M>,
}

impl<'a, T, const M: usize, const N: usize> Copy for VectorView<'a, T, M, N> {}
impl<'a, T, const M: usize, const N: usize> Clone for VectorView<'a, T, M, N> {
	fn clone(&self) -> Self { *self }
}

impl<'a, T, const M: usize, const N: usize> IntoIterator for VectorView<'a, T, M, N> {
	type Item = &'a T;
	type IntoIter = ConstIterator<&'a T, Self, M>;

	fn into_iter(self) -> Self::IntoIter { self.into() }
}

#[test]
fn transpose_bounds() {
	extern crate std;
	use std::println;
	let a: Matrix<f32, 2, 3> = (0..3)
		.map(|r| (0..2).map(|e| (e + (10 * r)) as f32).collect())
		.collect();
	println!("origin matrix: {:?}", a);
	let a2 = a.transpose().materialize().transpose().materialize();

	assert_eq!(a, a2);
}
