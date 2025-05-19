//! Cell pair relationship rules

/// Explicit relationship between a pair of cells
#[derive(Debug, Clone, Copy)]
pub enum CellPairRelationship {
    LessThan,
    LessEqual,
    Difference(i32),
    Sum(i32),
    Ratio(i32),
}
