use std::ops::{Add, Div, Mul, Neg};

use crate::{
    prelude::{SquareMatrix, TentleyError},
    scalar::{One, Scalar, Zero},
};

impl<
        T: Scalar + Mul<Output = T> + Add<Output = T> + Div<Output = T> + Neg<Output = T> + One + Zero,
        const N: usize,
    > SquareMatrix<T, N>
{

    /// Returns the lu decomposition of this [`SquareMatrix<T, N>`].
    /// 
    /// Finds two matrices, l and u in the form self = lu. Where l is a lower triangular matrix and u is an upper triangular matrix.
    ///
    /// # Errors
    ///
    /// - [`TentleyError::DivisionByZero`] if the matrix is singular
    pub fn lu_decomposition(&self) -> Result<(Self, Self), TentleyError> {
        let (mut l, mut u) = (Self::identity(), *self);

        for i in 0..N {
            let denominator = unsafe { *u.get_unchecked(i, i) };

            if denominator.is_zero() {
                return Err(TentleyError::DivisionByZero);
            }

            for j in i + 1..N {
                let factor = unsafe { *u.get_unchecked(j, i) } / denominator;

                unsafe {
                    *l.get_unchecked_mut(j, i) = factor;
                }

                u.add_rows(i, j, -factor)?;
            }
        }

        Ok((l, u))
    }
}
