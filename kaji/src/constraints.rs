use crate::{puzzle::SolveState, CellInfo, Symbol};

/// A puzzle constraint
///
/// Constraints limit the possibilities of cell values
/// within the board.  Constraints are defined at puzzle build time
/// and apply their limitations at the start of solving, and whenever
/// the board changes in areas they are interested in.
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
#[derive(Default)]
pub enum LogicalStep {
    /// During this step in the solve, something happened
    Acted(String),
    /// Nothing was able to be done
    #[default]
    NoAction,
    /// Nothing more is possible
    Finished,
}

impl LogicalStep {
    /// Create a new [`LogicalStep`] based on the given description.
    ///
    /// ```
    /// # use kaji::LogicalStep;
    /// let step = LogicalStep::action("Hello");
    /// assert!(matches!(step, LogicalStep::Acted(_)));
    /// ```
    pub fn action(desc: impl Into<String>) -> Self {
        Self::Acted(desc.into())
    }

    /// Push a description of a set of cells into a [`LogicalStep`]
    ///
    /// This takes an iterator over [`&CellInfo`][CellInfo] and
    /// renders the cells as their rXcY representation into the
    /// logical step's description.
    ///
    /// For example if the iterator yielded cells r2c3 and r3c4
    /// then the string "r2c3, r3c4" would be appended.
    ///
    /// # Panics
    ///
    /// The step **must** be a [`LogicalStep::Acted`] otherwise this
    /// will panic.
    pub fn push_cells<'a>(&mut self, cells: impl IntoIterator<Item = &'a CellInfo>) {
        let Self::Acted(body) = self else {
            panic!("Attempted to push_cells() to a non-action logical step");
        };
        let mut divider = "";
        for cell in cells {
            body.push_str(divider);
            body.push_str(&format!("{cell}"));
            divider = ", ";
        }
    }

    /// Push a string into a [`LogicalStep`]
    ///
    /// This simply appends the given string argument to the step's
    /// description.
    ///
    /// # Panics
    ///
    /// The step **must** be a [`LogicalStep::Acted`] otherwise this
    /// will panic.
    pub fn push_str(&mut self, s: &str) {
        let Self::Acted(body) = self else {
            panic!("Attempted to push_cells() to a non-action logical step");
        };
        body.push_str(s);
    }

    /// Push a set of symbols into a [`LogicalStep`]
    ///
    /// This takes an iterator which yields [`&Symbol`][`Symbol`]
    /// and renders the symbols as their display names.
    ///
    /// For example, this may append the string "5, 4, 2"
    ///
    /// # Panics
    ///
    /// The step **must** be a [`LogicalStep::Acted`] otherwise this
    /// will panic.
    pub fn push_symbols<'a>(&mut self, symbols: impl IntoIterator<Item = &'a Symbol>) {
        let Self::Acted(body) = self else {
            panic!("Attempted to push_cells() to a non-action logical step");
        };
        let mut divider = "";
        for sym in symbols {
            body.push_str(divider);
            body.push_str(&format!("{sym}"));
            divider = ", ";
        }
    }
}
