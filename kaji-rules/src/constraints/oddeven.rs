//! Odd and Even cells
//!

use itertools::Itertools;
use kaji::{consts::DIFFICULTY_TRIVIAL, CellIndex, Constraint, LogicalStep, SolveState};

#[derive(Debug)]
pub struct OddEven {
    cell: CellIndex,
    is_odd: bool,
}

impl OddEven {
    pub fn new(cell: CellIndex, is_odd: bool) -> Self {
        Self { cell, is_odd }
    }
}

impl Constraint for OddEven {
    fn difficulty(&self) -> u16 {
        DIFFICULTY_TRIVIAL + 250
    }

    fn prep_board(&self, _state: &mut SolveState) {
        // Nothing
    }

    fn logical_step(&self, state: &mut SolveState) -> LogicalStep {
        let values = state.cell_values(self.cell).collect_vec();
        if values.len() < 2 {
            return LogicalStep::NoAction;
        }

        let mut action = LogicalStep::action(format!(
            "{} cell at ",
            if self.is_odd { "Odd" } else { "Even" }
        ));
        action.push_cells(Some(state.cell_info(self.cell)));
        action.push_str(" eliminates ");
        assert_eq!(
            values[0].symbols().len(),
            1,
            "For now, we only support odd/even when there are no modifiers"
        );
        let mut elims = vec![];
        for value in values {
            let is_odd = (value.value() & 1) == 1;
            let sym0 = value.symbols().pop().unwrap();
            if is_odd != self.is_odd {
                elims.push(sym0);
                state.eliminate(self.cell, sym0);
            }
        }
        if !elims.is_empty() {
            action.push_symbols(elims.into_iter().map(|symbol| state.symbol(symbol)));
            return action;
        }

        LogicalStep::NoAction
    }
}
