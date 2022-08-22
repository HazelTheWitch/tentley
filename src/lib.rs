#![feature(type_alias_impl_trait)]
#![warn(missing_docs)]

//! A linear algebra library written by Hazel Rella.
//! More of a learning opportunity than a real project, probably don't
//! use it in any real projects but if you do let me know!

/// Errors for Tentley
pub mod error;
/// Linear algebra module.
pub mod linalg;
/// Matrix, AugmentedMatrix, and SquareMatrix
pub mod matrix;
/// Traits relating to the elements of Matrices
pub mod scalar;

/// Everything you need to get started with Tentley.
pub mod prelude {
    pub use super::error::*;
    pub use super::matrix::{AugmentedMatrix, Matrix, SquareMatrix};

    pub use tentley_macros::{mat, row_vector, vector};
}
