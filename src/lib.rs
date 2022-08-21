pub mod error;
pub mod scalar;
pub mod matrix;

pub mod prelude {
    pub use super::matrix::{Matrix, SquareMatrix};
    pub use tentley_macros::mat;
    pub use super::error::TentleyError;
}