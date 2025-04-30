use crate::PuzzleBuilder;

/// A Rule for a puzzle
///
/// Rules are used to construct [`Puzzle`][crate::Puzzle]s
pub trait Rule {
    /// Apply a rule to the given puzzle builder
    ///
    /// This takes the requisite information about the puzzle
    /// and applies symbol sets, constraints, and techniques to
    /// the [`PuzzleBuilder`] so that the resulting [`Puzzle`][crate::Puzzle]
    /// can be solved.
    fn apply(&self, builder: &mut PuzzleBuilder);
}
