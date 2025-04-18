use crate::puzzle::{Board, Puzzle};

use super::Constraint;

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
    fn prep_board(&self, puzzle: &Puzzle, board: &mut Board) {
        let digits = puzzle.symbols(0).collect::<Vec<_>>();
        for &(row, col, digit) in &self.digits {
            if let Some(cell) = puzzle.cell_at(row, col) {
                assert!(digit > 0);
                assert!(digit <= digits.len());
                puzzle.set_symbol(board, cell, digits[digit - 1]);
            }
        }
    }
}
