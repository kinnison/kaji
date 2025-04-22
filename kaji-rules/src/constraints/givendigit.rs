use kaji::{consts::SYMBOL_SET_DIGITS, Constraint, SolveState};

#[derive(Debug)]
pub struct GivenDigits {
    digits: Vec<(usize, usize, usize)>,
}

impl GivenDigits {
    pub fn new(givens: &[(usize, usize, usize)]) -> Self {
        Self {
            digits: givens.to_vec(),
        }
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
