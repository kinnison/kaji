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
                let sym_a = value_a.symbols().pop().unwrap();
                if state.eliminate(self.cell_a, sym_a).changed() {
                    if changed {
                        action.push_str(", ");
                    }
                    changed = true;
                    action.push_symbols(Some(state.symbol(sym_a)));
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
                let sym_b = value_b.symbols().pop().unwrap();
                if state.eliminate(self.cell_b, sym_b).changed() {
                    if changed {
                        action.push_str(", ");
                    }
                    changed = true;
                    action.push_symbols(Some(state.symbol(sym_b)));
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
            DiffAtLeast(n) => (a - b).abs() >= *n,
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
    double_overlap: bool,
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
        double_overlap: bool,
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
            double_overlap,
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
            assert!(overlap_value.symbols().len() == 1);
            let sym_o = overlap_value.symbols().pop().unwrap();
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

            // In the double_overlap case, we need at least one of permitted_ab to be in
            // permitted_cd since that could be the other cell
            // If not double_overlap, there must be at least one member of permitted_ab not in permitted_cd or vice-versa

            if (self.double_overlap && !permitted_ab.iter().any(|abv| permitted_cd.contains(abv)))
                || (!self.double_overlap
                    && permitted_ab.iter().all(|abv| permitted_cd.contains(abv))
                    && permitted_cd.iter().all(|cdv| permitted_ab.contains(cdv))
                    && permitted_ab.len() < 2)
            {
                // So by whatever means, the overlap cell value cannot stand, eliminate it.
                let mut action = LogicalStep::action(&self.name);
                action.push_str(" eliminates ");
                action.push_symbols(Some(state.symbol(sym_o)));
                action.push_str(" from ");
                action.push_cells(Some(state.cell_info(self.overlap)));
                if state.eliminate(self.overlap, sym_o).changed() {
                    return action;
                }
            }
        }
        LogicalStep::NoAction
    }
}
