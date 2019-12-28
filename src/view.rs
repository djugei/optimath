use crate::types::{FullMath, Matrix, Vector};
use core::ops::*;

struct VectorView<'a, T: FullMath, const M: usize, const N: usize> {
    row: usize,
    matrix: &'a Matrix<T, M, N>,
}

impl<'a, T: FullMath, const M: usize, const N: usize> Index<usize> for VectorView<'a, T, M, N> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        &self.matrix[index][self.row]
    }
}

/*
struct TransposedMatrixView<'a, T: FullMath, const M: usize, const N: usize>{
    inner: Vector<VectorView<'a, T, M, N>, M>
}

impl<'a, T: FullMath, const M: usize, const N: usize> Index<usize> for TransposedMatrixView<'a, T, M, N> {
    type Output = VectorView<'a, T, M, N>;
    fn index(&self, index: usize) -> &VectorView<'a, T, M, N> {
        &self.matrix[index][self.row]
    }
}
*/

