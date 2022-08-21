use std::{
    iter::{zip, Sum},
    mem::{transmute_copy, MaybeUninit},
    ops::{Add, Mul},
};

use crate::scalar::Scalar;

use super::Matrix;

impl<T: Scalar + Mul<Output = T> + Sum, const R: usize, const N: usize, const C: usize>
    Mul<Matrix<T, N, C>> for Matrix<T, R, N>
{
    type Output = Matrix<T, R, C>;

    fn mul(self, rhs: Matrix<T, N, C>) -> Self::Output {
        let mut data: [[MaybeUninit<T>; R]; C] = unsafe { MaybeUninit::uninit().assume_init() };

        for i in 0..R {
            for j in 0..C {
                data[i][j] = MaybeUninit::new(
                    unsafe { zip(self.get_row_unchecked(i), rhs.get_col_unchecked(j)) }
                        .map(|(a, b)| *a * *b)
                        .sum(),
                )
            }
        }

        Matrix::new(unsafe { transmute_copy(&data) })
    }
}

impl<T: Scalar + Add<Output = T>, const R: usize, const C: usize> Add for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data: [[MaybeUninit<T>; R]; C] = unsafe { MaybeUninit::uninit().assume_init() };

        for row in 0..R {
            for (col, (a, b)) in
                unsafe { zip(self.get_row_unchecked(row), rhs.get_row_unchecked(row)) }.enumerate()
            {
                data[col][row] = MaybeUninit::new(*a + *b);
            }
        }

        Matrix::new(unsafe { transmute_copy(&data) })
    }
}
