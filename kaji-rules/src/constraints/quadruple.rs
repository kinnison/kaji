//! Quadruple constraint (circle with digits in)
//!

use std::collections::HashMap;

use itertools::Itertools;
use kaji::{
    consts::DIFFICULTY_MEDIUM, CellIndex, Constraint, LogicalStep, SolveState, SymbolChoice,
    SymbolId, SymbolSetId,
};

#[derive(Debug)]
pub struct Quadruple {
    cells: Vec<CellIndex>,
    set: SymbolSetId,
    needed: Vec<SymbolId>,
}

#[derive(Debug)]
pub struct QuadState {
    choices: Vec<SymbolChoice>,
    unfilled: Vec<CellIndex>,
    left: Vec<SymbolId>,
}

impl Quadruple {
    pub fn new(
        cells: impl IntoIterator<Item = CellIndex>,
        needed: impl IntoIterator<Item = SymbolId>,
    ) -> Self {
        let cells = cells.into_iter().collect();
        let needed: Vec<SymbolId> = needed.into_iter().collect();

        assert!(needed
            .iter()
            .copied()
            .tuple_windows()
            .all(|(a, b)| a.symbol_set() == b.symbol_set()));

        let set = needed[0].symbol_set();

        Self { cells, set, needed }
    }

    fn qstate(&self, state: &SolveState) -> QuadState {
        let choices = self
            .cells
            .iter()
            .map(|&cell| state.choices(cell, self.set))
            .collect_vec();
        let unfilled = choices
            .iter()
            .enumerate()
            .filter_map(|(i, choice)| {
                if choice.solved() {
                    None
                } else {
                    Some(self.cells[i])
                }
            })
            .collect_vec();
        let mut left = self.needed.clone();

        for cell in self.cells.iter().copied() {
            let choice = state.choices(cell, self.set);
            if let Some(symbol) = choice.single_value() {
                if let Some((pos, _)) = left.iter().copied().find_position(|&s| s == symbol) {
                    left.remove(pos);
                }
            }
        }

        QuadState {
            choices,
            unfilled,
            left,
        }
    }

    fn prep_action(&self, state: &mut SolveState<'_>) -> LogicalStep {
        let mut action = LogicalStep::action("Quadruple at ");
        action.push_cells(self.cells.iter().copied().map(|cell| state.cell_info(cell)));
        action.push_str(": ");
        action
    }
}

impl QuadState {
    fn constrain(&self, quad: &Quadruple, state: &mut SolveState) -> LogicalStep {
        if self.left.is_empty() {
            return LogicalStep::NoAction;
        }
        let mut action = quad.prep_action(state);
        if self.left.len() == self.unfilled.len() {
            let left = self
                .left
                .iter()
                .copied()
                .map(|symbol| symbol.to_choice())
                .reduce(|a, b| a | b)
                .unwrap();
            let mut changed = vec![];
            for cell in self.unfilled.iter().copied() {
                if state.restrict(cell, left).changed() {
                    changed.push(cell);
                }
            }
            if !changed.is_empty() {
                action.push_str("restricted cells ");
                action.push_cells(changed.into_iter().map(|cell| state.cell_info(cell)));
                action.push_str(" to ");
                action.push_symbols(left.options().map(|symbol| state.symbol(symbol)));
                return action;
            }
        }

        LogicalStep::NoAction
    }
}

impl Constraint for Quadruple {
    fn prep_board(&self, state: &mut SolveState) {
        let qstate = self.qstate(state);

        qstate.constrain(self, state);
    }

    fn logical_step(&self, state: &mut SolveState) -> LogicalStep {
        let qstate = self.qstate(state);

        match qstate.constrain(self, state) {
            LogicalStep::NoAction => (),
            v => return v,
        }

        // Have a go at hidden single in quads
        let mut found: HashMap<SymbolId, Vec<CellIndex>> = HashMap::new();
        for cell in qstate.unfilled {
            for symbol in state.choices(cell, self.set).options() {
                if qstate.left.contains(&symbol) {
                    found.entry(symbol).or_default().push(cell);
                }
            }
        }
        for (symbol, cells) in found {
            if cells.len() == 1 {
                let mut action = self.prep_action(state);
                action.push_str("sets hidden single ");
                action.push_symbols(Some(state.symbol(symbol)));
                action.push_str(" in ");
                action.push_cells(Some(state.cell_info(cells[0])));
                state.set_symbol(cells[0], symbol);
                return action;
            }
        }

        LogicalStep::NoAction
    }

    fn difficulty(&self) -> u16 {
        DIFFICULTY_MEDIUM
    }
}
