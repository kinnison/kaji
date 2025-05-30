//! Cell pair relationship rules

use std::{collections::HashSet, fmt};

use itertools::Itertools;
use kaji::{CellIndex, PuzzleBuilder, Rule};

use crate::constraints::{CellPairConstraint, DoubleCellPairConstraint};

/// Explicit relationship between a pair of cells
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CellPairRelationship {
    LessThan,
    LessEqual,
    Difference(i32),
    Sum(i32),
    Ratio(i32),
    DiffAtLeast(i32),
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
            DiffAtLeast(n) => write!(f, "Difference of at least {n}"),
        }
    }
}

impl CellPairRelationship {
    pub fn neg_name(&self) -> String {
        match self {
            CellPairRelationship::LessThan => "Greater-than-or-equal".into(),
            CellPairRelationship::LessEqual => "Greater-than".into(),
            CellPairRelationship::Difference(d) => {
                if *d == 1 {
                    "Non-consecutive".into()
                } else {
                    format!("Anti-difference of {d}")
                }
            }
            CellPairRelationship::Sum(n) => match *n {
                5 => "Anti-V".into(),
                10 => "Anti-X".into(),
                _ => format!("Anti-sum of {n}"),
            },
            CellPairRelationship::Ratio(r) => match *r {
                2 => "Anti-Black-Dot".into(),
                _ => format!("Anti-ratio of {r}"),
            },
            CellPairRelationship::DiffAtLeast(n) => format!("Difference of at most {}", *n - 1),
        }
    }
}

#[derive(Debug)]
pub struct CellPairsRule {
    cells: Vec<CellIndex>,
    rels: HashSet<(String, CellIndex, CellIndex, CellPairRelationship)>,
    negs: Vec<CellPairRelationship>,
}

impl CellPairsRule {
    pub fn new(
        cells: impl IntoIterator<Item = CellIndex>,
        rels: impl IntoIterator<Item = (String, CellIndex, CellIndex, CellPairRelationship)>,
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
        for (name, cell_a, cell_b, rel) in &self.rels {
            let (cell_a, cell_b, rel) = (*cell_a, *cell_b, *rel);
            used_pairs.insert((cell_a, cell_b));
            used_pairs.insert((cell_b, cell_a));
            builder.add_constraint(CellPairConstraint::new(name, cell_a, cell_b, rel, false));
        }
        let regions = builder.regions().collect_vec();
        for (first, second) in self.rels.iter().tuple_combinations() {
            if !(first.1 == second.1
                || first.1 == second.2
                || first.2 == second.1
                || first.2 == second.2)
            {
                // no overlap
                continue;
            }
            let overlap = if first.1 == second.1 || first.1 == second.2 {
                first.1
            } else {
                first.2
            };
            let other_a = if first.1 == overlap { first.2 } else { first.1 };
            let other_b = if second.1 == overlap {
                second.2
            } else {
                second.1
            };

            if !regions.iter().copied().any(|r| {
                let cells = builder.region(r).to_cells();
                cells.contains(&other_a) && cells.contains(&other_b)
            }) {
                // No overlap found
                continue;
            }

            let name = if first.0 == second.0 {
                first.0.clone()
            } else {
                format!("{n0} and {n1}", n0 = first.0, n1 = second.0)
            };

            builder.add_constraint(DoubleCellPairConstraint::new(
                name,
                first.1,
                first.2,
                first.3,
                false,
                second.1,
                second.2,
                second.3,
                false,
                overlap,
                other_a == other_b,
            ));
        }
        if self.negs.is_empty() {
            // Nothing more to do, so return
            return;
        }
        for (cell_a, cell_b) in builder.all_orthogonal_pairs(&self.cells) {
            if !used_pairs.contains(&(cell_a, cell_b)) {
                for neg in &self.negs {
                    builder.add_constraint(CellPairConstraint::new(
                        neg.neg_name(),
                        cell_a,
                        cell_b,
                        *neg,
                        true,
                    ));
                }
            }
        }
    }
}
