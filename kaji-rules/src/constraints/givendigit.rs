use kaji::{Constraint, SolveState, SymbolSetId};

use crate::puzzledata::SudokuGridData;

#[derive(Debug)]
pub struct GivenDigits {
    symbols: SymbolSetId,
    digits: Vec<(usize, usize, usize)>,
}

impl GivenDigits {
    pub fn new(symbols: SymbolSetId, raw: &SudokuGridData) -> Self {
        Self {
            symbols,
            digits: raw.givens().to_vec(),
        }
    }

    pub fn from_pattern(symbols: SymbolSetId, pattern: &[&str]) -> Self {
        let mut digits = vec![];
        for (rownr, row) in pattern.iter().enumerate() {
            for (colnr, digitch) in row.chars().enumerate() {
                if let '0'..='9' = digitch {
                    digits.push(((rownr + 1), (colnr + 1), ((digitch as u8) - b'0') as usize))
                }
            }
        }
        Self { symbols, digits }
    }
}

impl Constraint for GivenDigits {
    fn prep_board(&self, state: &mut SolveState) {
        let digits = state.symbols(self.symbols).collect::<Vec<_>>();
        for &(row, col, digit) in &self.digits {
            if let Some(cell) = state.cell_at(row, col) {
                assert!(digit > 0);
                assert!(digit <= digits.len());
                state.set_symbol(cell, digits[digit - 1]);
            }
        }
    }

    fn difficulty(&self) -> u16 {
        0 // We don't do logical steps, so no difficulty
    }
}
