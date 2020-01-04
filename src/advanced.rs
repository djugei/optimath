//! non-element wise operations, like dot product and matrix multiplication
//! as such they need to explicitly be called

use crate::{
	types::{Matrix, Vector},
	view::{TransposedMatrixView, VectorView},
};

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
	pub fn transpose(&self) -> TransposedMatrixView<'_, T, N, M> {
		TransposedMatrixView { matrix: self }
	}
}

impl<'a, 'b, T: 'a + 'b + Clone + Copy + Default, const M: usize, const N: usize> Matrix<T, M, N>
where
	&'a T: core::ops::Mul<&'b T, Output = T>,
	T: core::iter::Sum,
{
	pub default fn matrix_multiply<const O: usize>(
		&'a self,
		other: &'b Matrix<T, N, O>,
	) -> Matrix<T, M, O> {
		//todo: do this without default-initalizing
		let mut output = Matrix::default();
		if false {
			return output;
		}
		let sel: TransposedMatrixView<T, N, M> = self.transpose();

		for (row, o) in (0..O).zip(other) {
			let o: &'b Vector<T, N> = o;
			let col = &mut output[row];
			for (column, s) in (0..M).zip(sel) {
				let s: VectorView<T, N, M> = s;
				let field: &mut T = &mut col[column];
				*field = s.dot(o)
			}
		}
		output
	}
}

impl<'a, 'b, T: 'a + 'b, const M: usize, const N: usize> VectorView<'a, T, M, N>
where
	&'a T: core::ops::Mul<&'b T, Output = T>,
	T: core::iter::Sum,
{
	pub default fn dot(self, other: &'b Vector<T, M>) -> T { (self * other).into_iter().sum() }
}

impl<'a, 'b, T: 'a + 'b, const M: usize> Vector<T, M>
where
	&'a T: core::ops::Mul<&'b T, Output = T>,
	T: core::iter::Sum,
{
	pub default fn dot(&'a self, other: &'b Vector<T, M>) -> T { (self * other).into_iter().sum() }
}

#[test]
fn matrix_multiply() {
	use rand::{thread_rng, Rng};
	let mut rng = thread_rng();

	let a: Matrix<f32, 2, 3> = rng.gen();
	let b: Matrix<f32, 3, 4> = rng.gen();

	let _c: Matrix<f32, 2, 4> = a.matrix_multiply(&b);
}
