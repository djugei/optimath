use crate::types::{Matrix, Vector};
use core::ops::*;

#[derive(Clone, Copy)]
pub struct TransposedMatrixView<'a, T, const M: usize, const N: usize>{
    pub(crate) matrix: &'a Matrix<T, M, N>,
}

impl<'a, T: 'a, const M: usize, const N: usize> TransposedMatrixView<'a, T, M, N> {
    pub fn get(self, index: usize) -> VectorView<'a, T, M, N> {
        VectorView {
            row: index,
            matrix: self.matrix
        }
    }
}

#[derive(Copy, Clone)]
pub struct VectorView<'a, T, const M: usize, const N: usize> {
    row: usize,
    matrix: &'a Matrix<T, M, N>,
}

struct ViewIter<'a, T, const M: usize, const N: usize> {
    pos: usize,
    view: VectorView<'a, T, M, N>,
}

impl<'a, T, const M: usize, const N: usize> Iterator for ViewIter<'a, T, M, N> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        if self.pos == N {
            None
        } else {
            let out = Some(&self.view.matrix[self.pos][self.view.row]);
            self.pos += 1;
            out
        }
    }
}

impl<'a, 'b, T, const M: usize, const N: usize> Mul<&'b Vector<T, M>> for VectorView<'a, T, M, N>
where
    &'a T: Mul<&'b T, Output = T>,
{
    type Output = Vector<T, M>;
    fn mul(self, other: &'b Vector<T, M>) -> Vector<T, M> {
        let iter = ViewIter { pos: 0, view: self};
        iter.zip(other.inner.iter())
            .map(|(s, o)| s * o)
            .collect()
    }
}

#[cfg(test)]
use crate::base::TESTLEN;
