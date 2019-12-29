use crate::{
	types::{Matrix, Vector},
	view::{TransposedMatrixView, VectorView},
};

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
	pub fn transpose<'a>(&'a self) -> TransposedMatrixView<'a, T, N, M> {
		TransposedMatrixView { matrix: self }
	}
}

impl<'a, 'b, T: 'a + 'b + Clone + Copy + Default, const M: usize, const N: usize> Matrix<T, M, N>
where
	&'a T: core::ops::Mul<&'b T, Output = T>,
	T: core::iter::Sum,
{
	pub fn matrix_multiply<const O: usize>(&'a self, other: &'b Matrix<T, N, O>) -> Matrix<T, M, O> {
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
				*field = (s * o).into_iter().sum()
            }
        }
		output
	}
}

#[test]
fn matrix_multiply() {
	let a: Matrix<f32, 2, 3> = Default::default();
	let b: Matrix<f32, 3, 4> = Default::default();
	
    let c: Matrix<f32, 2, 4> = a.matrix_multiply(&b);
}
