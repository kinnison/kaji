//! Techniques are logical operations performed on a board
//! at runtime to try and make progress.

use crate::{LogicalStep, SolveState};

/// A solving technique
///
/// Each solving technique is a logical step along the way
/// from an unsolved puzzle to a solved puzzle.  Techniques
/// may be as simple as "This cell can only be X, so it's X"
/// for as complex as chaining inferences through multiple steps.
pub trait Technique: std::fmt::Debug {
    /// How hard is this constraint to make logical deductions with?
    ///
    /// You may use the constants from the [`kaji`][crate] crate,
    /// for example [`DIFFICULTY_EASY`][crate::consts::DIFFICULTY_EASY]
    fn difficulty(&self) -> u16;

    /// Execute a single logical action on the board.
    ///
    /// If the constraint has no logical steps that it can make,
    /// then this can be omitted, but the constraint should implement
    /// board preparation instead.
    ///
    /// If the constraint *could* perform a logical action, but is
    /// unable to given the current board state, it should return
    /// `LogicalStep::NoAction`.  If everything the constraint
    /// could do has been done, it should return [`LogicalStep::Finished`].
    fn logical_step(&self, _state: &mut SolveState) -> LogicalStep;
}
