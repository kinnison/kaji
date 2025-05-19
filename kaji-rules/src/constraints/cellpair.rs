//! CellPair relationships as a constraint

use itertools::Itertools;
use kaji::{consts::DIFFICULTY_MEDIUM, CellIndex, Constraint, LogicalStep, SolveState};

use crate::rules::cellpairs::CellPairRelationship;

#[derive(Debug)]
pub struct CellPairConstraint {
    cell_a: CellIndex,
    cell_b: CellIndex,
    rel: CellPairRelationship,
    neg: bool,
}

impl CellPairConstraint {
    pub fn new(cell_a: CellIndex, cell_b: CellIndex, rel: CellPairRelationship, neg: bool) -> Self {
        Self {
            cell_a,
            cell_b,
            rel,
            neg,
        }
    }
}

impl Constraint for CellPairConstraint {
    fn difficulty(&self) -> u16 {
        DIFFICULTY_MEDIUM + 1000
    }

    fn prep_board(&self, _state: &mut SolveState) {
        // Nothing for us to do here
    }

    fn logical_step(&self, state: &mut SolveState) -> LogicalStep {
        let values_a = state.cell_values(self.cell_a).collect_vec();
        let values_b = state.cell_values(self.cell_b).collect_vec();

        // Essentially what we want to do is:
        // For each value_a, if there is no value_b which satisfies the relationship
        // eliminate it if possible
        // Repeat value_b/value_a swapped

        let mut action = LogicalStep::action(format!(
            "{rel} {neg}between ",
            rel = self.rel,
            neg = if self.neg { "(negative) " } else { "" }
        ));
        action.push_cells(
            Some(state.cell_info(self.cell_a))
                .into_iter()
                .chain(Some(state.cell_info(self.cell_b))),
        );
        action.push_str(" eliminates ");

        let mut changed = false;

        for value_a in &values_a {
            if !values_b
                .iter()
                .any(|value_b| self.rel.satisfied(value_a.value(), value_b.value()) ^ self.neg)
            {
                // Nothing in values_b satisfies value_a
                assert_eq!(value_a.symbols().len(), 1);
                if state.eliminate(self.cell_a, value_a.symbols()[0]).changed() {
                    if changed {
                        action.push_str(", ");
                    }
                    changed = true;
                    action.push_symbols(Some(state.symbol(value_a.symbols()[0])));
                    action.push_str(" from ");
                    action.push_cells(Some(state.cell_info(self.cell_a)));
                }
            }
        }

        for value_b in &values_b {
            if !values_a
                .iter()
                .any(|value_a| self.rel.satisfied(value_a.value(), value_b.value()) ^ self.neg)
            {
                // Nothing in values_b satisfies value_a
                assert_eq!(value_b.symbols().len(), 1);
                if state.eliminate(self.cell_b, value_b.symbols()[0]).changed() {
                    if changed {
                        action.push_str(", ");
                    }
                    changed = true;
                    action.push_symbols(Some(state.symbol(value_b.symbols()[0])));
                    action.push_str(" from ");
                    action.push_cells(Some(state.cell_info(self.cell_b)));
                }
            }
        }

        if changed {
            return action;
        }

        LogicalStep::NoAction
    }
}

impl CellPairRelationship {
    fn satisfied(&self, a: i32, b: i32) -> bool {
        use CellPairRelationship::*;
        match self {
            LessThan => a < b,
            LessEqual => a <= b,
            Difference(n) => (a - b).abs() == *n,
            Sum(n) => (a + b) == *n,
            Ratio(n) => (a == (b * *n)) || (b == (a * *n)),
        }
    }
}
