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
		let element = &row[self.row];
		element
	}
}
