use super::Matrix;

impl<const R: usize, const C: usize> Matrix<f32, R, C> {
    /// Generates a [`Matrix<f32, R, C>`] with elements randomly in the range [0, 1).
    pub fn random() -> Self {
        Self::from_fn(|_, _| fastrand::f32())
    }

    /// Generates a [`Matrix<f32, R, C>`] with elements randomly in the range [min, max).
    pub fn uniform(min: f32, max: f32) -> Self {
        let span = max - min;
        Self::from_fn(|_, _| fastrand::f32() * span + min)
    }
}

impl<const R: usize, const C: usize> Matrix<f64, R, C> {
    /// Generates a [`Matrix<f64, R, C>`] with elements randomly in the range [0, 1).
    pub fn random() -> Self {
        Self::from_fn(|_, _| fastrand::f64())
    }

    /// Generates a [`Matrix<f64, R, C>`] with elements randomly in the range [min, max).
    pub fn uniform(min: f64, max: f64) -> Self {
        let span = max - min;
        Self::from_fn(|_, _| fastrand::f64() * span + min)
    }
}
