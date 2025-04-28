use crate::{puzzle::SolveState, CellIndex, CellInfo, Symbol};

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
    Acted(String),
    /// Nothing was able to be done
    #[default]
    NoAction,
    /// Nothing more is possible
    Finished,
}

impl LogicalStep {
    pub fn action(desc: impl Into<String>) -> Self {
        Self::Acted(desc.into())
    }

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

    pub fn push_str(&mut self, s: &str) {
        let Self::Acted(body) = self else {
            panic!("Attempted to push_cells() to a non-action logical step");
        };
        body.push_str(s);
    }

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
