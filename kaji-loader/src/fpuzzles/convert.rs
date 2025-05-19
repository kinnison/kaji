use std::num::NonZeroUsize;

use kaji::SymbolValue;
use kaji_rules::{
    puzzledata::{
        GridData, GridDataKind, PuzzleData, RawSudokuPairRelationship, SudokuGridData,
        SudokuGridRuleCloneData, SudokuGridRuleQuadrupleData, SymbolData, SymbolSetData,
    },
    rules::cellpairs::CellPairRelationship,
};

use super::FpuzzlesData;

impl From<FpuzzlesData> for PuzzleData {
    fn from(val: FpuzzlesData) -> Self {
        let size = val.grid.len();
        let mut puzzle = Self::default();
        let mut digits = SymbolSetData::new("digits");

        for n in 1..=size {
            digits.push(SymbolData::new(format!("{n}"), SymbolValue::Set(n as i32)));
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
                for (a, b) in line.iter().zip(line.iter().rev()).take(line.len() >> 1) {
                    grid.rules_mut().clone_pairs.push(SudokuGridRuleCloneData {
                        a: (a.row, a.col),
                        b: (b.row, b.col),
                    })
                }
            }
        }

        for odd in val.odd {
            grid.rules_mut()
                .odd_cells
                .push((odd.cell.row, odd.cell.col));
        }

        for even in val.even {
            grid.rules_mut()
                .even_cells
                .push((even.cell.row, even.cell.col));
        }

        grid.rules_mut().pair_relationships.nonconsecutive = val.nonconsecutive;

        for xvpair in val.xv {
            let rel = if xvpair.value == "X" {
                CellPairRelationship::Sum(10)
            } else {
                CellPairRelationship::Sum(5)
            };
            grid.rules_mut()
                .pair_relationships
                .relationships
                .push(RawSudokuPairRelationship {
                    cell_a: (xvpair.cells.0.row, xvpair.cells.0.col),
                    cell_b: (xvpair.cells.1.row, xvpair.cells.1.col),
                    relationship: rel,
                });
        }

        for diff in val.difference {
            grid.rules_mut()
                .pair_relationships
                .relationships
                .push(RawSudokuPairRelationship {
                    cell_a: (diff.cells.0.row, diff.cells.0.col),
                    cell_b: (diff.cells.1.row, diff.cells.1.col),
                    relationship: CellPairRelationship::Difference(diff.value.unwrap_or(1)),
                });
        }

        for ratio in val.ratio {
            grid.rules_mut()
                .pair_relationships
                .relationships
                .push(RawSudokuPairRelationship {
                    cell_a: (ratio.cells.0.row, ratio.cells.0.col),
                    cell_b: (ratio.cells.1.row, ratio.cells.1.col),
                    relationship: CellPairRelationship::Ratio(ratio.value.unwrap_or(2)),
                });
        }

        let grid = GridData::new(0, 0, GridDataKind::Sudoku(grid));

        puzzle.push_grid(grid);

        puzzle
    }
}
