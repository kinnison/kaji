//! Cell pair relationship rules

use std::{collections::HashSet, fmt};

use kaji::{CellIndex, PuzzleBuilder, Rule};

use crate::constraints::CellPairConstraint;

/// Explicit relationship between a pair of cells
#[derive(Debug, Clone, Copy)]
pub enum CellPairRelationship {
    LessThan,
    LessEqual,
    Difference(i32),
    Sum(i32),
    Ratio(i32),
}

impl fmt::Display for CellPairRelationship {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CellPairRelationship::*;
        match self {
            LessThan => f.write_str("Less-than"),
            LessEqual => f.write_str("Less-than-or-equal"),
            Difference(n) => write!(f, "Difference of {n}"),
            Sum(n) => write!(f, "Sum to {n}"),
            Ratio(n) => write!(f, "Ratio of {n}"),
        }
    }
}

#[derive(Debug)]
pub struct CellPairsRule {
    cells: Vec<CellIndex>,
    rels: Vec<(CellIndex, CellIndex, CellPairRelationship)>,
    negs: Vec<CellPairRelationship>,
}

impl CellPairsRule {
    pub fn new(
        cells: impl IntoIterator<Item = CellIndex>,
        rels: impl IntoIterator<Item = (CellIndex, CellIndex, CellPairRelationship)>,
        negs: impl IntoIterator<Item = CellPairRelationship>,
    ) -> Self {
        Self {
            cells: cells.into_iter().collect(),
            rels: rels.into_iter().collect(),
            negs: negs.into_iter().collect(),
        }
    }
}

impl Rule for CellPairsRule {
    fn apply(&self, builder: &mut PuzzleBuilder) {
        // Step one, create constraints for all the rels
        let mut used_pairs = HashSet::new();
        for &(cell_a, cell_b, rel) in &self.rels {
            used_pairs.insert((cell_a, cell_b));
            used_pairs.insert((cell_b, cell_a));
            builder.add_constraint(CellPairConstraint::new(cell_a, cell_b, rel, false));
        }
        if self.negs.is_empty() {
            // Nothing more to do, so return
            return;
        }
        for (cell_a, cell_b) in builder.all_orthogonal_pairs(&self.cells) {
            if !used_pairs.contains(&(cell_a, cell_b)) {
                for neg in &self.negs {
                    builder.add_constraint(CellPairConstraint::new(cell_a, cell_b, *neg, true));
                }
            }
        }
    }
}
