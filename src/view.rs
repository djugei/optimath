//! views on underlying vectors
//!
//! basically move trough the data at different strides and offsets
//! currently only transposed matrices and contained flipped vectors
//!
//! the Index trait sucks hard
use crate::types::Matrix;

#[derive(Clone, Copy)]
pub struct TransposedMatrixView<'a, T, const M: usize, const N: usize> {
	pub(crate) matrix: &'a Matrix<T, N, M>,
}

impl<'a, T: 'a, const M: usize, const N: usize> TransposedMatrixView<'a, T, M, N> {
	pub fn get(&self, index: usize) -> VectorView<'a, T, M, N> {
		debug_assert!(index <= M);
		VectorView {
			row: index,
			matrix: self.matrix,
		}
	}
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
	type IntoIter = TransIter<'a, T, M, N>;

	fn into_iter(self) -> Self::IntoIter {
		TransIter {
			pos: 0,
			trans: self,
		}
	}
}

pub struct TransIter<'a, T, const M: usize, const N: usize> {
	pos: usize,
	trans: TransposedMatrixView<'a, T, M, N>,
}

impl<'a, T, const M: usize, const N: usize> Iterator for TransIter<'a, T, M, N> {
	type Item = VectorView<'a, T, M, N>;
	fn next(&mut self) -> Option<VectorView<'a, T, M, N>> {
		if self.pos == N {
			None
		} else {
			//println!("pos: {}, M: {}, N: {}", self.pos, M, N);
			let out = self.trans.get(self.pos);
			self.pos += 1;
			Some(out)
		}
	}
}

#[derive(Copy, Clone)]
pub struct VectorView<'a, T, const M: usize, const N: usize> {
	pub(crate) row: usize,
	pub(crate) matrix: &'a Matrix<T, N, M>,
}

impl<'a, T, const M: usize, const N: usize> IntoIterator for VectorView<'a, T, M, N> {
	type Item = &'a T;
	type IntoIter = ViewIter<'a, T, M, N>;

	fn into_iter(self) -> Self::IntoIter { ViewIter { pos: 0, view: self } }
}

pub struct ViewIter<'a, T, const M: usize, const N: usize> {
	pos: usize,
	view: VectorView<'a, T, M, N>,
}

impl<'a, T, const M: usize, const N: usize> Iterator for ViewIter<'a, T, M, N> {
	type Item = &'a T;
	fn next(&mut self) -> Option<&'a T> {
		if self.pos == M {
			None
		} else {
			//println!("row: {}, pos: {}, M: {}, N: {}", self.view.row, self.pos, M, N);
			let row = &self.view.matrix[self.pos];
			let element = &row[self.view.row];
			self.pos += 1;
			Some(element)
		}
	}
}

#[test]
fn transmute_bounds_1() {
	use rand::{thread_rng, Rng};
	let mut rng = thread_rng();
	let a: Matrix<f32, 1, 2> = rng.gen();
	let b = a.transpose().materialize();
	let a2 = b.transpose().materialize();

	assert_eq!(a, a2);
}
