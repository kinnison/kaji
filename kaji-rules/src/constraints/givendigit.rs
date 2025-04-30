use kaji::{consts::SYMBOL_SET_DIGITS, Constraint, SolveState};
use kaji_loader::raw::RawPuzzleData;

#[derive(Debug)]
pub struct GivenDigits {
    digits: Vec<(usize, usize, usize)>,
}

impl GivenDigits {
    pub fn new(raw: &RawPuzzleData) -> Self {
        let mut givens = vec![];
        for (rrow, row) in raw.cells.iter().enumerate() {
            for (rcol, cell) in row.iter().enumerate() {
                if let Some(value) = cell.value {
                    givens.push((rrow + 1, rcol + 1, value));
                }
            }
        }
        Self {
            digits: givens.to_vec(),
        }
    }

    pub fn from_pattern(pattern: &[&str]) -> Self {
        let mut digits = vec![];
        for (rownr, row) in pattern.iter().enumerate() {
            for (colnr, digitch) in row.chars().enumerate() {
                if let '0'..='9' = digitch {
                    digits.push(((rownr + 1), (colnr + 1), ((digitch as u8) - b'0') as usize))
                }
            }
        }
        Self { digits }
    }
}

impl Constraint for GivenDigits {
    fn prep_board(&self, state: &mut SolveState) {
        let digits = state
            .symbols_by_set_name(SYMBOL_SET_DIGITS)
            .collect::<Vec<_>>();
        for &(row, col, digit) in &self.digits {
            if let Some(cell) = state.cell_at(row, col) {
                assert!(digit > 0);
                assert!(digit <= digits.len());
                state.set_symbol(cell, digits[digit - 1]);
            }
        }
    }
}
