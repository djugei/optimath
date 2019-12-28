use crate::types::{FullMath, Matrix, Vector};

impl<T: FullMath + Default, const M: usize, const N: usize> Matrix<T, M, N> {
    fn matrix_multiply<const O: usize>(&self, other: &Matrix<T, N, O>) -> Matrix<T, M, O> {
        //todo: do this without default-initalizing
        let mut output = Matrix::default();
        if false {
            return output;
        }
        for column in 0..M {
            let col = &mut output[column];
            for row in 0..O {
                let field: &mut T = &mut col[row];
            }
        }

        todo!()
    }

    fn transposed_iter<'a>(&'a self, row: usize) -> TransposedIter<'a, T, M, N> {
        TransposedIter {
            row,
            pos: 0,
            data: self,
        }
    }
}

pub struct TransposedIter<'a, T: FullMath, const M: usize, const N: usize> {
    row: usize,
    pos: usize,
    data: &'a Matrix<T, M, N>,
}

impl<'a, T: FullMath, const M: usize, const N: usize> Iterator for TransposedIter<'a, T, M, N> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        debug_assert_eq!(N, self.data[0].inner.len());
        if self.pos == N {
            let item = &self.data[self.row][self.pos];
            self.pos += 1;
            Some(item)
        } else {
            None
        }
    }
}


#[test]
fn transpose_iter() {
    let v = Vector::<f32, TESTLEN>::default();
    let m = v.ascent();
    let mut i = v.transposed_iter(0);
    assert_eq!(i.next(), Some(0.));
    assert_eq!(i.next(), None);
}
