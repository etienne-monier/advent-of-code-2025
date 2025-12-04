extern crate nalgebra as na;
use na::DMatrix;

use std::fmt;

// WRAP DMatrix in a Display implementation (idiomatic)

pub struct DisplayMatrix<'a, T>(pub &'a DMatrix<T>);

impl<'a, T> fmt::Display for DisplayMatrix<'a, T>
where
    T: fmt::Display + PartialEq + From<u8>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let m = self.0;
        for i in 0..m.nrows() {
            for j in 0..m.ncols() {
                if m[(i, j)] == T::from(0u8) {
                    write!(f, ". ")?;
                } else {
                    write!(f, "{} ", m[(i, j)])?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// Convolve 2D matrices (loose size)
pub fn convolve2d(matrix: &DMatrix<i32>, kernel: &DMatrix<i32>) -> DMatrix<i32> {
    let (m_rows, m_cols) = (matrix.nrows(), matrix.ncols());
    let (k_rows, k_cols) = (kernel.nrows(), kernel.ncols());

    let out_rows = m_rows - k_rows + 1;
    let out_cols = m_cols - k_cols + 1;

    let mut result = DMatrix::<i32>::zeros(out_rows, out_cols);

    for i in 0..out_rows {
        for j in 0..out_cols {
            let mut sum = 0;
            for ki in 0..k_rows {
                for kj in 0..k_cols {
                    sum += matrix[(i + ki, j + kj)] * kernel[(ki, kj)];
                }
            }
            result[(i, j)] = sum;
        }
    }

    result
}

// Pad mattrix with 0 (size provided with pad)
pub fn pad_matrix(matrix: &DMatrix<i32>, pad: usize) -> DMatrix<i32> {
    let new_rows = matrix.nrows() + 2 * pad;
    let new_cols = matrix.ncols() + 2 * pad;
    let mut padded = DMatrix::<i32>::zeros(new_rows, new_cols);

    for i in 0..matrix.nrows() {
        for j in 0..matrix.ncols() {
            padded[(i + pad, j + pad)] = matrix[(i, j)];
        }
    }

    padded
}
