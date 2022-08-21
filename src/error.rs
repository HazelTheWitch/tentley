#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Axis {
    Row,
    Column,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TentleyError {
    UninitializedError { exptected: usize, actual: usize },
    IndexOutOfBounds { axis: Axis },
    DivisionByZero,
}