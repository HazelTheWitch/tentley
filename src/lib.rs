// #![feature(generic_const_exprs)]

pub mod error;
pub mod linalg;
pub mod matrix;
pub mod scalar;

pub mod prelude {
    pub use super::error::*;
    pub use super::matrix::{Matrix, SquareMatrix, AugmentedMatrix};

    pub use tentley_macros::{mat, row_vector, vector};
}
