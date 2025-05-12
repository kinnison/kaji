use std::num::NonZeroUsize;

use kaji_rules::puzzledata::{
    GridData, GridDataKind, PuzzleData, SudokuGridData, SymbolData, SymbolSetData,
};

use super::FpuzzlesData;

impl From<FpuzzlesData> for PuzzleData {
    fn from(val: FpuzzlesData) -> Self {
        let size = val.grid.len();
        let mut puzzle = Self::default();
        let mut digits = SymbolSetData::new("digits");

        for n in 1..=size {
            digits.push(SymbolData::new(format!("{n}")));
        }

        puzzle.push_symbols(digits);

        let mut regions = SudokuGridData::default_regions(size);

        for rnum in 0..size {
            for cnum in 0..size {
                if let Some(rawregion) = val.grid[rnum][cnum].region {
                    regions[(rnum * size) + cnum] = rawregion + 1;
                }
            }
        }

        let mut grid = SudokuGridData::new("digits", NonZeroUsize::new(size).unwrap(), regions);
        grid.rules_mut().antiking = val.antiking;
        grid.rules_mut().antiknight = val.antiknight;

        for (rnum, row) in val.grid.into_iter().enumerate() {
            for (cnum, cell) in row.into_iter().enumerate() {
                if let Some(given) = cell.value {
                    grid.push_given(rnum + 1, cnum + 1, given.get());
                }
            }
        }

        if let Some(solution) = &val.solution {
            let solution = solution
                .iter()
                .map(|n| b'0' + (n.get() as u8))
                .map(|ch| ch as char)
                .collect::<String>();
            grid.set_solution_(&solution);
        }

        let grid = GridData::new(0, 0, GridDataKind::Sudoku(grid));

        puzzle.push_grid(grid);

        puzzle
    }
}
