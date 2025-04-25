//! Sudoku grids

use kaji::{consts::SYMBOL_SET_DIGITS, CellInfo, PuzzleBuilder, Region, Rule, Symbol};

use crate::techniques::{HiddenSingle, NakedSingle};

use super::regions::NonRepeatRegion;

pub struct SudokuGrid {
    size: usize,
}

impl SudokuGrid {
    pub fn new(size: usize) -> Self {
        assert!([4, 6, 8, 9].contains(&size));
        Self { size }
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

        let rows = rows
            .into_iter()
            .enumerate()
            .map(|(n, row)| builder.add_region(Region::new(format!("Row {}", n + 1), row)))
            .collect::<Vec<_>>();

        let cols = cols
            .into_iter()
            .enumerate()
            .map(|(n, col)| builder.add_region(Region::new(format!("Column {}", n + 1), col)))
            .collect::<Vec<_>>();

        let mut boxes = vec![];

        // 4=2x2 6=3x2 8=4x2, 9=3x3
        let boxw = if self.size == 9 { 3 } else { self.size >> 1 };
        let boxh = if self.size == 9 { 3 } else { 2 };

        let mut srow = 1;
        let mut scol = 1;
        for boxr in 1..=self.size {
            let mut boxcells = vec![];
            for brow in 0..boxh {
                for bcol in 0..boxw {
                    let cellrow = srow + brow;
                    let cellcol = scol + bcol;
                    let cellnr = ((cellrow - 1) * self.size) + cellcol - 1;
                    boxcells.push(cells[cellnr]);
                }
            }
            boxes.push(builder.add_region(Region::new(format!("Box {boxr}"), boxcells)));

            scol += boxw;
            if scol > self.size {
                scol = 1;
                srow += boxh;
            }
        }
        for region in rows
            .into_iter()
            .chain(cols.into_iter())
            .chain(boxes.into_iter())
        {
            NonRepeatRegion::new(region, digits).apply(builder);
        }

        builder.add_technique(NakedSingle::new(digits));
        builder.add_technique(HiddenSingle::new(digits));
    }
}
