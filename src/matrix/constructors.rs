use crate::scalar::Scalar;

use super::Matrix;

impl<T: Scalar, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn new(data: [[T; C]; R]) -> Self {
        assert!(R > 0, "rows must be greater than 1");
        assert!(C > 0, "cols must be greater than 1");

        Self { data }
    }

    pub fn full(value: T) -> Self {
        Self::new([[value; C]; R])
    }
}