pub enum TentleyError {
    UninitializedError { exptected: usize, actual: usize }
}