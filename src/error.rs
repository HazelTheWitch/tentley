#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Axis {
    Row,
    Column,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TentleyError {
    IndexOutOfBounds { axis: Axis },
    DivisionByZero,
}
