mod givendigit;
mod singles;

pub use givendigit::GivenDigits;
pub use singles::HiddenSingle;
pub use singles::NakedSingle;

use crate::puzzle::SolveState;

pub trait Constraint: std::fmt::Debug {
    /// Prepare a board ready for "playing".
    ///
    /// This is called exactly once and is used to apply
    /// constraint information to the initial board created
    /// during solving.
    ///
    /// If the constraint has no preparation to do, this can be
    /// omitted.  However if omitted, the constraint must provide
    /// a logical step
    fn prep_board(&self, _state: &mut SolveState) {}

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
    fn logical_step(&self, _state: &mut SolveState) -> LogicalStep {
        LogicalStep::Finished
    }
}

/// The result of running a logical step in our solver
pub enum LogicalStep {
    Acted(String),
    /// Nothing was able to be done
    NoAction,
    /// Nothing more is possible
    Finished,
}
