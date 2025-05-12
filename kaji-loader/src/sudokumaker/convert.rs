use std::num::NonZeroUsize;

use kaji_rules::puzzledata::{
    GridData, GridDataKind, PuzzleData, SudokuGridData, SymbolData, SymbolSetData,
};

use super::RawPuzzleData;

impl From<RawPuzzleData> for PuzzleData {
    fn from(val: RawPuzzleData) -> Self {
        let size = val.cells.len();
        let mut puzzle = Self::default();
        let mut digits = SymbolSetData::new("digits");

        for n in 1..=size {
            digits.push(SymbolData::new(format!("{n}")));
        }

        puzzle.push_symbols(digits);

        let mut regions = SudokuGridData::default_regions(size);
        for (rnum, rcells) in val.regions.into_iter().enumerate() {
            for cell in rcells {
                let cindex = (cell.0 * size) + cell.1;
                regions[cindex] = rnum + 1;
            }
        }

        let mut grid = SudokuGridData::new("digits", NonZeroUsize::new(size).unwrap(), regions);
        grid.rules_mut().antiking = val.metadata.antiking;
        grid.rules_mut().antiknight = val.metadata.antiknight;

        for (rnum, row) in val.cells.into_iter().enumerate() {
            for (cnum, cell) in row.into_iter().enumerate() {
                if let Some(given) = cell.value {
                    grid.push_given(rnum + 1, cnum + 1, given);
                }
            }
        }

        if let Some(solution) = &val.metadata.solution {
            grid.set_solution_(solution);
        }

        let grid = GridData::new(0, 0, GridDataKind::Sudoku(grid));

        puzzle.push_grid(grid);

        puzzle
    }
}
