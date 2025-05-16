//! Clone cell constraint
//!

use kaji::{consts::DIFFICULTY_EASY, CellIndex, Constraint, LogicalStep, SolveState};

#[derive(Debug)]
pub struct CloneCell {
    cell_a: CellIndex,
    cell_b: CellIndex,
}

impl CloneCell {
    pub fn new(cell_a: CellIndex, cell_b: CellIndex) -> Self {
        Self { cell_a, cell_b }
    }
}

impl Constraint for CloneCell {
    fn difficulty(&self) -> u16 {
        DIFFICULTY_EASY + 500
    }

    fn prep_board(&self, _state: &mut SolveState) {
        // Nothing
    }

    fn logical_step(&self, state: &mut SolveState) -> LogicalStep {
        let mut action = LogicalStep::action("Clone pair (");
        action.push_cells(Some(state.cell_info(self.cell_a)));
        action.push_str(" / ");
        action.push_cells(Some(state.cell_info(self.cell_b)));
        action.push_str(") ");
        let mut changed = false;
        for symbol_set in state.symbol_sets() {
            let choice_a = state.choices(self.cell_a, symbol_set);
            let choice_b = state.choices(self.cell_b, symbol_set);
            if choice_a != choice_b {
                // Restricting...
                let new_choice = choice_a & choice_b;

                if state.restrict(self.cell_a, new_choice).changed()
                    | state.restrict(self.cell_b, new_choice).changed()
                {
                    changed = true;
                    action.push_str("restricted to ");
                    action.push_symbols(new_choice.options().map(|symbol| state.symbol(symbol)));
                }
            }
        }
        if changed {
            return action;
        }
        LogicalStep::NoAction
    }
}
