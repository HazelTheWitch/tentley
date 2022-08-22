use crate::scalar::Scalar;

use super::Matrix;

impl<T: Scalar, const R: usize, const C: usize> Matrix<T, R, C> {
    /// Get a reference to a given element. Returns None if the index is out of bounds.
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.data.get(row)?.get(col)
    }

    /// Get a mutable reference to a given element. Returns None if the index is out of bounds.
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        self.data.get_mut(row)?.get_mut(col)
    }

    /// Get a reference to a given element.
    ///
    /// # Safety
    ///
    /// Safe if the index is in bounds.
    pub unsafe fn get_unchecked(&self, row: usize, col: usize) -> &T {
        self.data.get_unchecked(row).get_unchecked(col)
    }

    /// Get a mutable reference to a given element.
    ///
    /// # Safety
    ///
    /// Safe if the index is in bounds.
    pub unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut T {
        self.data.get_unchecked_mut(row).get_unchecked_mut(col)
    }

    /// Get a vector containing referecnes to a row of the matrix. Returns None if the row is out of bounds.
    pub fn get_row(&self, row: usize) -> Option<Vec<&T>> {
        Some(self.data.get(row)?.iter().collect())
    }

    /// Get a vector containing mutable referecnes to a row of the matrix. Returns None if the row is out of bounds.
    pub fn get_row_mut(&mut self, row: usize) -> Option<Vec<&mut T>> {
        Some(self.data.get_mut(row)?.iter_mut().collect())
    }

    /// Get a vector containing referecnes to a column of the matrix. Returns None if the column is out of bounds.
    pub fn get_col(&self, col: usize) -> Option<Vec<&T>> {
        if col >= C {
            return None;
        }

        Some(
            self.data
                .iter()
                // Safety: we have done a bounds check, so the index is in bounds.
                .map(|row| unsafe { row.get_unchecked(col) })
                .collect(),
        )
    }

    /// Get a vector containing mutable referecnes to a column of the matrix. Returns None if the column is out of bounds.
    pub fn get_col_mut(&mut self, col: usize) -> Option<Vec<&mut T>> {
        if col >= C {
            return None;
        }

        Some(
            self.data
                .iter_mut()
                // Safety: we have done a bounds check, so the index is in bounds.
                .map(|row| unsafe { row.get_unchecked_mut(col) })
                .collect(),
        )
    }

    /// Get a vector of references to a row.
    ///
    /// # Safety
    ///
    /// Safe when the row is in bounds.
    pub unsafe fn get_row_unchecked(&self, row: usize) -> Vec<&T> {
        self.data.get_unchecked(row).iter().collect()
    }

    // Get a vector of mutable references to a row.
    ///
    /// # Safety
    ///
    /// Safe when the row is in bounds.
    pub unsafe fn get_row_unchecked_mut(&mut self, row: usize) -> Vec<&mut T> {
        self.data.get_unchecked_mut(row).iter_mut().collect()
    }

    // Get a vector of references to a column.
    ///
    /// # Safety
    ///
    /// Safe when the column is in bounds.
    pub unsafe fn get_col_unchecked(&self, col: usize) -> Vec<&T> {
        self.data.iter().map(|row| row.get_unchecked(col)).collect()
    }

    // Get a vector of mutable references to a column.
    ///
    /// # Safety
    ///
    /// Safe when the column is in bounds.
    pub unsafe fn get_col_unchecked_mut(&mut self, col: usize) -> Vec<&mut T> {
        self.data
            .iter_mut()
            .map(|row| row.get_unchecked_mut(col))
            .collect()
    }
}
