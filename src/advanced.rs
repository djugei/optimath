use crate::types::{Matrix};
use crate::view::{TransposedMatrixView};

impl<T: Default, const M: usize, const N: usize> Matrix<T, M, N> {
    fn matrix_multiply<const O: usize>(
        &self,
        other: &Matrix<T, O, M>
    ) -> Matrix<T, O, N> {
        //todo: do this without default-initalizing
        let mut output = Matrix::default();
        if false {
            return output;
        }
        for column in 0..M {
            let col = &mut output[column];
            for row in 0..O {
                //fixme: calculate self.transpose() before the loop
                // TransposedMatrixView is not Clone for some reason
                // ...
                let s = self.transpose().get(column);
                let o = &other[column];
                let f = s * o;
                let field: &mut T = &mut col[row];
                
            }
        }

        todo!()
    }
    fn transpose<'a>(&'a self) -> TransposedMatrixView<'a, T, M, N> {
        TransposedMatrixView {
            matrix: self
        }
    }
}
