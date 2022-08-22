#![feature(type_alias_impl_trait)]

pub mod error;
pub mod linalg;
pub mod matrix;
pub mod scalar;

pub mod prelude {
    pub use super::error::*;
    pub use super::matrix::{AugmentedMatrix, Matrix, SquareMatrix};

    pub use tentley_macros::{mat, row_vector, vector};
}
