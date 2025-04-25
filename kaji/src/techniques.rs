//! Techniques are logical operations performed on a board
//! at runtime to try and make progress.

use crate::{LogicalStep, SolveState};

pub trait Technique: std::fmt::Debug {
    /// Execute a single logical action on the board.
    ///
    /// If the constraint has no logical steps that it can make,
    /// then this can be omitted, but the constraint should implement
    /// board preparation instead.
    ///
    /// If the constraint *could* perform a logical action, but is
    /// unable to given the current board state, it should return
    /// `LogicalStep::NoAction`.  If everything the constraint
    /// could do has been done, it should return `LogicalStep::Finished`.
    fn logical_step(&self, _state: &mut SolveState) -> LogicalStep;
}
