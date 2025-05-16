use std::num::NonZeroUsize;

use kaji_rules::puzzledata::{
    GridData, GridDataKind, PuzzleData, SudokuGridData, SudokuGridRuleCloneData,
    SudokuGridRuleQuadrupleData, SymbolData, SymbolSetData,
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
        grid.rules_mut().diagonal_n = val.diagonal_n;
        grid.rules_mut().diagonal_p = val.diagonal_p;
        grid.rules_mut().disjoint_groups = val.disjointgroups;

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

        grid.rules_mut().quadruple = val
            .quadruple
            .into_iter()
            .map(|fpquad| {
                let cells = fpquad.cells.into_iter().map(|c| (c.row, c.col)).collect();
                let symbols = fpquad.values.into_iter().map(|v| v.get()).collect();
                SudokuGridRuleQuadrupleData { cells, symbols }
            })
            .collect();

        for clone_group in val.clone {
            assert_eq!(clone_group.cells.len(), clone_group.clone_cells.len());
            for (a, b) in clone_group
                .cells
                .into_iter()
                .zip(clone_group.clone_cells.into_iter())
            {
                grid.rules_mut().clone_pairs.push(SudokuGridRuleCloneData {
                    a: (a.row, a.col),
                    b: (b.row, b.col),
                })
            }
        }

        for palindrome in val.palindrome {
            for line in palindrome.lines {
                // a,b,c,d,e,f
                // -> (a,f) (b,e) (c,d)
                // a,b,c
                // -> (a,c)
                for (a, b) in line.iter().zip(line.iter().rev()) {
                    grid.rules_mut().clone_pairs.push(SudokuGridRuleCloneData {
                        a: (a.row, a.col),
                        b: (b.row, b.col),
                    })
                }
            }
        }

        let grid = GridData::new(0, 0, GridDataKind::Sudoku(grid));

        puzzle.push_grid(grid);

        puzzle
    }
}
