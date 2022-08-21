use crate::scalar::Scalar;

use super::Matrix;

impl<T: Scalar, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.data.get(row)?.get(col)
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        self.data.get_mut(row)?.get_mut(col)
    }

    pub unsafe fn get_unchecked(&self, row: usize, col: usize) -> &T {
        self.data.get_unchecked(row).get_unchecked(col)
    }

    pub unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut T {
        self.data.get_unchecked_mut(row).get_unchecked_mut(col)
    }

    pub fn get_row(&self, row: usize) -> Option<Vec<&T>> {
        Some(self.data.get(row)?.into_iter().collect())
    }

    pub fn get_row_mut(&mut self, row: usize) -> Option<Vec<&mut T>> {
        Some(self.data.get_mut(row)?.into_iter().collect())
    }

    pub fn get_col(&self, col: usize) -> Option<Vec<&T>> {
        if col >= C {
            return None;
        }

        Some(self.data.iter().map(|row| unsafe { row.get_unchecked(col) }).collect())
    }

    pub fn get_col_mut(&mut self, col: usize) -> Option<Vec<&mut T>> {
        if col >= C {
            return None;
        }

        Some(self.data.iter_mut().map(|row| unsafe { row.get_unchecked_mut(col) }).collect())
    }

    pub unsafe fn get_row_unchecked(&self, row: usize) -> Vec<&T> {
        self.data.get_unchecked(row).into_iter().collect()
    }

    pub unsafe  fn get_row_unchecked_mut(&mut self, row: usize) -> Vec<&mut T> {
        self.data.get_unchecked_mut(row).into_iter().collect()
    }

    pub unsafe  fn get_col_unchecked(&self, col: usize) -> Vec<&T> {
        self.data.iter().map(|row| row.get_unchecked(col)).collect()
    }

    pub unsafe fn get_col_unchecked_mut(&mut self, col: usize) -> Vec<&mut T> {
        self.data.iter_mut().map(|row| row.get_unchecked_mut(col)).collect()
    }
}