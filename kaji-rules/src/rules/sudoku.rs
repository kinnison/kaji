//! Sudoku grids

use std::collections::HashSet;

use itertools::Itertools;
use kaji::{CellInfo, PuzzleBuilder, Region, Rule, SymbolSetId};

use crate::{
    constraints::{CloneCell, GivenDigits, Indexer, OddEven, Quadruple, SudokuIndexerKind},
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

        let region_cols = cols
            .into_iter()
            .enumerate()
            .map(|(n, col)| builder.add_region(Region::new(format!("Column {}", n + 1), col)))
            .collect::<Vec<_>>();

        let mut box_cells = vec![vec![]; raw.size()];

        for (cellidx, rawregion) in raw.regions().iter().copied().enumerate() {
            box_cells[rawregion - 1].push(cells[cellidx]);
        }

        let region_boxes = box_cells
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

        for &region in region_rows
            .iter()
            .chain(region_cols.iter())
            .chain(region_boxes.iter())
            .chain(extra_regions.iter())
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
                r.name.clone(),
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
            let name = format!("Minimum cell at {}", builder.cell_info(cell));
            for other in builder.orthogonal_cells(cell) {
                if !minimum.contains(&other) {
                    minimax.push((name.clone(), cell, other, CellPairRelationship::LessThan));
                }
            }
        }

        for cell in maximum.iter().copied() {
            let name = format!("Maximum cell at {}", builder.cell_info(cell));
            for other in builder.orthogonal_cells(cell) {
                if !maximum.contains(&other) {
                    minimax.push((name.clone(), other, cell, CellPairRelationship::LessThan));
                }
            }
        }

        let mut thermo_pairs = vec![];

        for thermo in raw.rules().thermometer.iter() {
            let start = thermo[0];
            let finish = thermo.iter().last().copied().unwrap();
            let name = format!(
                "Thermometer from r{}c{} to r{}c{}",
                start.0, start.1, finish.0, finish.1
            );
            for (a, b) in thermo.iter().copied().tuple_windows() {
                thermo_pairs.push((
                    name.clone(),
                    rows[a.0 - 1][a.1 - 1],
                    rows[b.0 - 1][b.1 - 1],
                    CellPairRelationship::LessThan,
                ));
            }
        }

        let mut whisper_pairs = vec![];

        for (diff, line) in raw.rules().whispers.iter() {
            let name = match (*diff, raw.size()) {
                (3, 6) | (5, 9) => "German Whisper".to_string(),
                (2, 6) | (4, 9) => "Dutch Whisper".to_string(),
                _ => format!("Whisper {diff}"),
            };
            let start = line[0];
            let finish = line.iter().copied().last().unwrap();
            let name = format!(
                "{name} from r{}c{} to r{}c{}",
                start.0, start.1, finish.0, finish.1
            );
            for (cell_a, cell_b) in line.iter().copied().tuple_windows() {
                let cell_a = rows[cell_a.0 - 1][cell_a.1 - 1];
                let cell_b = rows[cell_b.0 - 1][cell_b.1 - 1];
                whisper_pairs.push((
                    name.clone(),
                    cell_a,
                    cell_b,
                    CellPairRelationship::DiffAtLeast(*diff),
                ));
            }
        }

        CellPairsRule::new(
            cells.iter().copied(),
            pos_rels
                .chain(minimax)
                .chain(thermo_pairs)
                .chain(whisper_pairs),
            neg_rels,
        )
        .apply(builder);

        // Now add all the indexers
        for &(r, c, kind) in raw.rules().indexers.iter() {
            let cell = rows[r - 1][c - 1];
            let region = match kind {
                SudokuIndexerKind::Row => region_cols[c - 1],
                SudokuIndexerKind::Column => region_rows[r - 1],
                SudokuIndexerKind::Box => {
                    todo!("Work out the way to find a box")
                }
            };
            builder.add_constraint(Indexer::new(cell, region));
        }

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
