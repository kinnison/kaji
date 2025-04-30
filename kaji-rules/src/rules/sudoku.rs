//! Sudoku grids

use kaji::{consts::SYMBOL_SET_DIGITS, CellInfo, PuzzleBuilder, Region, Rule, Symbol};
use kaji_loader::raw::{RawPuzzleData, RawRowColPair};

use crate::techniques::{HiddenSingle, HiddenTuple, NakedSingle, NakedTuple, PointingSymbol};

use super::regions::NonRepeatRegion;

pub struct SudokuGrid {
    size: usize,
    regions: Vec<Vec<RawRowColPair>>,
}

impl SudokuGrid {
    pub fn new(raw: &RawPuzzleData) -> Self {
        let size = raw.cells.len();
        assert!([4, 6, 8, 9].contains(&size));
        Self {
            size,
            regions: raw.regions.clone(),
        }
    }
}

impl Rule for SudokuGrid {
    fn apply(&self, builder: &mut PuzzleBuilder) {
        let mut set = builder.new_symbol_set(SYMBOL_SET_DIGITS);
        (1..=self.size).for_each(|n| set.push(Symbol::new(format!("{n}"))));
        let digits = set.finish();
        let mut cells = vec![];
        let mut rows = vec![vec![]; self.size];
        let mut cols = vec![vec![]; self.size];

        (1..=self.size).for_each(|row| {
            (1..=self.size).for_each(|col| {
                let rc = builder.new_cell(CellInfo::new(format!("r{row}c{col}"), row, col));
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

        let mut boxes = vec![];

        for (rrno, rawregion) in self.regions.iter().enumerate() {
            let boxname = format!("Box {}", rrno + 1);
            let boxcells = rawregion
                .iter()
                .copied()
                .map(|RawRowColPair(rrow, rcol)| rows[rrow][rcol]);
            boxes.push(builder.add_region(Region::new(boxname, boxcells)));
        }

        for region in region_rows
            .into_iter()
            .chain(cols.into_iter())
            .chain(boxes.into_iter())
        {
            NonRepeatRegion::new(region, digits).apply(builder);
        }

        builder.add_technique(NakedSingle::new(digits));
        builder.add_technique(HiddenSingle::new(digits));
        builder.add_technique(NakedTuple::new(digits, 3));
        builder.add_technique(HiddenTuple::new(digits, 3));
        builder.add_technique(PointingSymbol::new(digits));
        builder.add_technique(NakedTuple::new(digits, self.size - 1));
        builder.add_technique(HiddenTuple::new(digits, self.size - 1));
    }
}
