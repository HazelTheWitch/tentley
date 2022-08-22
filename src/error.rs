//! Errors for Tentley

/// Enum representing rows or columns
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Axis {
    /// Row axis
    Row,
    /// Column axis
    Column,
}

/// Enum representing an error
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TentleyError {
    /// An out of bounds lookup along a given axis.
    IndexOutOfBounds {
        /// The axis of the out of bounds error 
        axis: Axis
    },
    /// Division by zero error
    DivisionByZero,
    /// Returned when an operation was performed on a complex type which did not exist in that type.
    /// ```rust
    /// use tentley::complex::*;
    /// 
    /// let x = 1.2;
    /// let c = ComplexF32::new(3.4, 6.0);
    /// 
    /// let r = x + c;
    /// 
    /// assert!(r.is_err());
    /// ```
    IncompatibleComplexType,
}
