use std::ops::{Add, Mul};

use crate::{scalar::Scalar, prelude::{TentleyError, Axis}};

use super::Matrix;

impl<T: Scalar + Add<Output = T> + Mul<Output = T>, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn swap_rows(&mut self, row_0: usize, row_1: usize) -> Result<(), TentleyError> {
        if row_0 >= R || row_1 >= R {
            return Err(TentleyError::IndexOutOfBounds { axis: Axis::Row });
        }

        self.data.swap(row_0, row_1);

        Ok(())
    }

    pub fn add_rows(&mut self, source: usize, target: usize, coefficient: T) -> Result<(), TentleyError> {
        if source >= R || target >= R {
            return Err(TentleyError::IndexOutOfBounds { axis: Axis::Row });
        }

        for col in 0..C {
            unsafe {
                let source_element: *const T = self.get_unchecked(source, col);
                let target_element: *mut T = self.get_unchecked_mut(target, col);

                *target_element = *target_element + coefficient * *source_element;
            }
        }

        Ok(())
    }

    pub fn multiply_row(&mut self, row: usize, coefficient: T) -> Result<(), TentleyError> {
        if row >= R {
            return Err(TentleyError::IndexOutOfBounds { axis: Axis::Row });
        }

        unsafe {
            for element in self.get_row_unchecked_mut(row) {
                *element = *element * coefficient;
            }
        }

        Ok(())
    }
}