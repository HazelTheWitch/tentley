// #![feature(generic_const_exprs)]

pub mod linalg;
pub mod error;
pub mod scalar;
pub mod matrix;

pub mod prelude {
    pub use super::matrix::{Matrix, SquareMatrix};
    pub use super::error::*;

    pub use tentley_macros::{mat, vector, row_vector};
}