//! Sudoku grids

use kaji::{CellInfo, PuzzleBuilder, Region, Rule, SymbolSetId};

use crate::{
    constraints::GivenDigits,
    puzzledata::SudokuGridData,
    techniques::{Fish, HiddenSingle, HiddenTuple, NakedSingle, NakedTuple, PointingSymbol},
};

use super::{antioffset::AntiOffset, regions::NonRepeatRegion};

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

        let mut boxes = vec![vec![]; raw.size()];

        for (cellidx, rawregion) in raw.regions().iter().copied().enumerate() {
            boxes[rawregion - 1].push(cells[cellidx]);
        }

        let boxes = boxes
            .into_iter()
            .enumerate()
            .map(|(boxn, boxcells)| {
                let boxname = format!("Box {}", boxn + 1);
                builder.add_region(Region::new(boxname, boxcells))
            })
            .collect::<Vec<_>>();

        for region in region_rows
            .into_iter()
            .chain(cols.into_iter())
            .chain(boxes.into_iter())
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
