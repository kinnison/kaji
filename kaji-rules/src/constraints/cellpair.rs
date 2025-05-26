//! CellPair relationships as a constraint

use itertools::Itertools;
use kaji::{consts::DIFFICULTY_MEDIUM, CellIndex, Constraint, LogicalStep, SolveState};

use crate::rules::cellpairs::CellPairRelationship;

#[derive(Debug)]
pub struct CellPairConstraint {
    name: String,
    cell_a: CellIndex,
    cell_b: CellIndex,
    rel: CellPairRelationship,
    neg: bool,
}

impl CellPairConstraint {
    pub fn new(
        name: impl Into<String>,
        cell_a: CellIndex,
        cell_b: CellIndex,
        rel: CellPairRelationship,
        neg: bool,
    ) -> Self {
        Self {
            name: name.into(),
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

        let mut action = LogicalStep::action(&self.name);
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

#[derive(Debug)]
pub struct DoubleCellPairConstraint {
    name: String,
    cell_a: CellIndex,
    cell_b: CellIndex,
    rel_ab: CellPairRelationship,
    neg_ab: bool,
    cell_c: CellIndex,
    cell_d: CellIndex,
    rel_cd: CellPairRelationship,
    neg_cd: bool,
    overlap: CellIndex,
}

impl DoubleCellPairConstraint {
    pub fn new(
        name: impl Into<String>,
        cell_a: CellIndex,
        cell_b: CellIndex,
        rel_ab: CellPairRelationship,
        neg_ab: bool,
        cell_c: CellIndex,
        cell_d: CellIndex,
        rel_cd: CellPairRelationship,
        neg_cd: bool,
        overlap: CellIndex,
    ) -> Self {
        assert!(cell_a == overlap || cell_b == overlap);
        assert!(cell_c == overlap || cell_d == overlap);
        Self {
            name: name.into(),
            cell_a,
            cell_b,
            rel_ab,
            neg_ab,
            cell_c,
            cell_d,
            rel_cd,
            neg_cd,
            overlap,
        }
    }
}

impl Constraint for DoubleCellPairConstraint {
    fn difficulty(&self) -> u16 {
        DIFFICULTY_MEDIUM + 1500
    }

    fn prep_board(&self, _state: &mut SolveState) {
        // Nothing to do for now
    }

    fn logical_step(&self, state: &mut SolveState) -> LogicalStep {
        // Essentially we have relationship ab and relationship cd.  We need to
        // determine a limitation based on the overlap such that the other cells
        // cannot be satisfied. then we can eliminate such.
        let values_a = state.cell_values(self.cell_a).collect_vec();
        let values_b = state.cell_values(self.cell_b).collect_vec();
        let values_c = state.cell_values(self.cell_c).collect_vec();
        let values_d = state.cell_values(self.cell_d).collect_vec();

        for overlap_value in state.cell_values(self.overlap) {
            let permitted_ab = if self.overlap == self.cell_a {
                values_b
                    .iter()
                    .filter(|v| {
                        self.rel_ab.satisfied(overlap_value.value(), v.value()) ^ self.neg_ab
                    })
                    .collect_vec()
            } else {
                values_a
                    .iter()
                    .filter(|v| {
                        self.rel_ab.satisfied(v.value(), overlap_value.value()) ^ self.neg_ab
                    })
                    .collect_vec()
            };
            let permitted_cd = if self.overlap == self.cell_c {
                values_d
                    .iter()
                    .filter(|v| {
                        self.rel_cd.satisfied(overlap_value.value(), v.value()) ^ self.neg_cd
                    })
                    .collect_vec()
            } else {
                values_c
                    .iter()
                    .filter(|v| {
                        self.rel_cd.satisfied(v.value(), overlap_value.value()) ^ self.neg_cd
                    })
                    .collect_vec()
            };
            // There must be at least one member of permitted_ab not in permitted_cd or vice-versa
            if permitted_ab.iter().all(|abv| permitted_cd.contains(abv))
                && permitted_cd.iter().all(|cdv| permitted_ab.contains(cdv))
                && permitted_ab.len() < 2
            {
                // effectively the same sets, so eliminate overlap_value from the overlap cell
                let mut action = LogicalStep::action(&self.name);
                action.push_str(" eliminates ");
                action.push_symbols(Some(state.symbol(overlap_value.symbols()[0])));
                action.push_str(" from ");
                action.push_cells(Some(state.cell_info(self.overlap)));
                if state
                    .eliminate(self.overlap, overlap_value.symbols()[0])
                    .changed()
                {
                    return action;
                }
            }
        }
        LogicalStep::NoAction
    }
}
