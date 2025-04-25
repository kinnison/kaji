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
    fn prep_board(&self, _state: &mut SolveState);
}

/// The result of running a logical step in our solver
pub enum LogicalStep {
    Acted(String),
    /// Nothing was able to be done
    NoAction,
    /// Nothing more is possible
    Finished,
}
