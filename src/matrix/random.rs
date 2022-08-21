use super::Matrix;

impl<const R: usize, const C: usize> Matrix<f32, R, C> {
    pub fn random() -> Self {
        Self::from_fn(|_, _| fastrand::f32())
    }

    pub fn uniform(min: f32, max: f32) -> Self {
        let span = max - min;
        Self::from_fn(|_, _| fastrand::f32() * span + min)
    }
}

impl<const R: usize, const C: usize> Matrix<f64, R, C> {
    pub fn random() -> Self {
        Self::from_fn(|_, _| fastrand::f64())
    }

    pub fn uniform(min: f64, max: f64) -> Self {
        let span = max - min;
        Self::from_fn(|_, _| fastrand::f64() * span + min)
    }
}