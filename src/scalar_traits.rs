//! Traits relating to the elements of Matrices

/// A trait for any type which can be stored in a Matrix, blanket implementation provided for any types which have the properties required.
pub trait Scalar: Copy {}

impl<T: Copy> Scalar for T {}