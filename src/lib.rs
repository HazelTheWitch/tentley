#![feature(type_alias_impl_trait)]
#![warn(missing_docs)]

//! A linear algebra library written by Hazel Rella.
//! More of a learning opportunity than a real project, probably don't
//! use it in any real projects but if you do let me know!


pub mod error;
pub mod linalg;
pub mod matrix;
pub mod scalar_traits;

pub mod prelude {
    //! Everything you need to get started with Tentley.
    pub use super::error::*;
    pub use super::matrix::{AugmentedMatrix, Matrix, SquareMatrix};

    pub use tentley_macros::{mat, row_vector, vector};
}
