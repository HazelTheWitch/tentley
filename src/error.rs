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
}
