//! Sudoku grids

use std::collections::HashSet;

use kaji::{CellInfo, PuzzleBuilder, Region, Rule, SymbolSetId};

use crate::{
    constraints::{CloneCell, GivenDigits, OddEven, Quadruple},
    puzzledata::SudokuGridData,
    techniques::{Fish, HiddenSingle, HiddenTuple, NakedSingle, NakedTuple, PointingSymbol},
};

use super::{
    antioffset::AntiOffset,
    cellpairs::{CellPairRelationship, CellPairsRule},
    regions::NonRepeatRegion,
};

pub struct SudokuGrid<'grid> {
    digits: SymbolSetId,
    rofs: usize,
    cofs: usize,
    raw: &'grid SudokuGridData,
}

impl<'grid> SudokuGrid<'grid> {
    pub fn new(digits: SymbolSetId, rofs: usize, cofs: usize, raw: &'grid SudokuGridData) -> Self {
        let size = raw.size();
        assert!([4, 6, 8, 9].contains(&size));
        Self {
            digits,
            rofs,
            cofs,
            raw,
        }
    }
}

impl Rule for SudokuGrid<'_> {
    fn apply(&self, builder: &mut PuzzleBuilder) {
        let raw = self.raw;
        let mut cells = vec![];
        let mut rows = vec![vec![]; raw.size()];
        let mut cols = vec![vec![]; raw.size()];

        (1..=raw.size()).for_each(|row| {
            (1..=raw.size()).for_each(|col| {
                let rc = builder.new_cell(CellInfo::new(
                    format!("r{row}c{col}"),
                    row + self.rofs,
                    col + self.cofs,
                ));
                rows[row - 1].push(rc);
                cols[col - 1].push(rc);
                cells.push(rc);
            });
        });

        let region_rows = rows
            .iter()
            .enumerate()
            .map(|(n, row)| {
                builder.add_region(Region::new(format!("Row {}", n + 1), row.iter().copied()))
            })
            .collect::<Vec<_>>();

        let cols = cols
            .into_iter()
            .enumerate()
            .map(|(n, col)| builder.add_region(Region::new(format!("Column {}", n + 1), col)))
            .collect::<Vec<_>>();

        let mut box_cells = vec![vec![]; raw.size()];

        for (cellidx, rawregion) in raw.regions().iter().copied().enumerate() {
            box_cells[rawregion - 1].push(cells[cellidx]);
        }

        let boxes = box_cells
            .iter()
            .enumerate()
            .map(|(boxn, boxcells)| {
                let boxname = format!("Box {}", boxn + 1);
                builder.add_region(Region::new(boxname, boxcells.iter().copied()))
            })
            .collect::<Vec<_>>();

        let mut extra_regions = vec![];
        if raw.rules().diagonal_n {
            extra_regions.push(builder.add_region(Region::new(
                "Negative Diagonal",
                (0..raw.size()).map(|n| rows[n][n]),
            )))
        }
        if raw.rules().diagonal_p {
            extra_regions.push(builder.add_region(Region::new(
                "Positive Diagonal",
                (0..raw.size()).map(|n| rows[raw.size() - n - 1][n]),
            )))
        }
        if raw.rules().disjoint_groups {
            for bidx in 0..raw.size() {
                extra_regions.push(builder.add_region(Region::new(
                    format!("Disjoint group {}", bidx + 1),
                    box_cells.iter().map(|b| b[bidx]),
                )));
            }
        }

        for region in region_rows
            .into_iter()
            .chain(cols.into_iter())
            .chain(boxes.into_iter())
            .chain(extra_regions.into_iter())
        {
            NonRepeatRegion::new(region, self.digits).apply(builder);
        }

        if raw.rules().antiking {
            AntiOffset::new(1, 1, self.digits, cells.clone()).apply(builder);
        }
        if raw.rules().antiknight {
            AntiOffset::new(1, 2, self.digits, cells.clone()).apply(builder);
        }

        builder.add_constraint(GivenDigits::new(self.digits, raw));

        if !raw.rules().quadruple.is_empty() {
            for quad in &raw.rules().quadruple {
                let cells = quad.cells.iter().map(|&(row, col)| rows[row - 1][col - 1]);
                let needed = quad.symbols.iter().map(|&i| {
                    builder
                        .symbols(self.digits)
                        .nth(i - 1)
                        .expect("Bad symbol index")
                });
                builder.add_constraint(Quadruple::new(cells, needed));
            }
        }

        for clone in &raw.rules().clone_pairs {
            let cell_a = rows[clone.a.0 - 1][clone.a.1 - 1];
            let cell_b = rows[clone.b.0 - 1][clone.b.1 - 1];
            builder.add_constraint(CloneCell::new(cell_a, cell_b));
        }

        for odd in &raw.rules().odd_cells {
            builder.add_constraint(OddEven::new(rows[odd.0 - 1][odd.1 - 1], true));
        }

        for even in &raw.rules().even_cells {
            builder.add_constraint(OddEven::new(rows[even.0 - 1][even.1 - 1], false));
        }

        // Cell pairs rules (eg. XV, et al)
        let mut neg_rels = vec![];
        let rels = &raw.rules().pair_relationships;
        if rels.nonconsecutive {
            neg_rels.push(CellPairRelationship::Difference(1));
        }
        if rels.anti_black_dot {
            neg_rels.push(CellPairRelationship::Ratio(2));
        }
        if rels.anti_v {
            neg_rels.push(CellPairRelationship::Sum(5));
        }
        if rels.anti_x {
            neg_rels.push(CellPairRelationship::Sum(10));
        }

        let pos_rels = rels.relationships.iter().map(|r| {
            (
                rows[r.cell_a.0 - 1][r.cell_a.1 - 1],
                rows[r.cell_b.0 - 1][r.cell_b.1 - 1],
                r.relationship,
            )
        });

        let minimum = raw
            .rules()
            .minimum
            .iter()
            .copied()
            .map(|(row, col)| rows[row - 1][col - 1])
            .collect::<HashSet<_>>();

        let maximum = raw
            .rules()
            .maximum
            .iter()
            .copied()
            .map(|(row, col)| rows[row - 1][col - 1])
            .collect::<HashSet<_>>();

        let mut minimax = vec![];

        for cell in minimum.iter().copied() {
            for other in builder.orthogonal_cells(cell) {
                if !minimum.contains(&other) {
                    minimax.push((cell, other, CellPairRelationship::LessThan));
                }
            }
        }

        for cell in maximum.iter().copied() {
            for other in builder.orthogonal_cells(cell) {
                if !maximum.contains(&other) {
                    minimax.push((other, cell, CellPairRelationship::LessThan));
                }
            }
        }

        CellPairsRule::new(cells.iter().copied(), pos_rels.chain(minimax), neg_rels).apply(builder);

        // Add Sudoku techniques

        builder.add_technique(NakedSingle::new(self.digits));
        builder.add_technique(HiddenSingle::new(self.digits));
        builder.add_technique(NakedTuple::new(self.digits, 3));
        builder.add_technique(HiddenTuple::new(self.digits, 3));
        builder.add_technique(PointingSymbol::new(self.digits));
        builder.add_technique(NakedTuple::new(self.digits, raw.size() - 1));
        builder.add_technique(HiddenTuple::new(self.digits, raw.size() - 1));
        builder.add_technique(Fish::new(2, self.digits));
        builder.add_technique(Fish::new(3, self.digits));
    }
}
