use crate::puzzle::{Board, CellIndex, Puzzle};

use super::{Constraint, LogicalStep};

#[derive(Debug)]
pub struct NakedSingle;

impl Constraint for NakedSingle {
    fn logical_step(&self, puzzle: &Puzzle, board: &mut Board) -> LogicalStep {
        for cell in puzzle.all_cells() {
            for set in puzzle.symbol_sets() {
                let choice = board.choices(cell).nth(set).unwrap();
                if !choice.solved() {
                    if let Some(value) = choice.single_value() {
                        // We're not solved and there's only one thing we can be
                        puzzle.set_symbol(board, cell, puzzle.symbol_set(set).id(set, value));
                        let cell = puzzle.cell_info(cell);
                        let value = &puzzle.symbol_set(set)[value];
                        return LogicalStep::Acted(format!("Naked single {value} at {cell}"));
                    }
                }
            }
        }

        LogicalStep::NoAction
    }
}

#[derive(Debug)]
pub struct HiddenSingle;

impl Constraint for HiddenSingle {
    fn logical_step(&self, puzzle: &Puzzle, board: &mut Board) -> LogicalStep {
        #[derive(Clone)]
        enum SymbolState {
            Unknown,
            Single(CellIndex),
            Many,
        }
        use SymbolState::*;
        for region in puzzle.regions() {
            let cells = puzzle.region(region).to_cells();
            for set in puzzle.symbol_sets() {
                let nsymbols = puzzle.symbol_set(set).len();
                let mut symbols = vec![Unknown; nsymbols];
                'cells: for cell in cells.iter().copied() {
                    let choice = board.choice_set(cell, set);
                    if choice.solved() {
                        continue 'cells;
                    }
                    for symbolnr in choice.options() {
                        symbols[symbolnr] = match symbols[symbolnr] {
                            Unknown => Single(cell),
                            _ => Many,
                        };
                    }
                }
                for (idx, symbol) in symbols.into_iter().enumerate() {
                    if let Single(cell) = symbol {
                        let symbol = puzzle.symbol_set(set).id(set, idx);
                        puzzle.set_symbol(board, cell, symbol);
                        let value = &puzzle.symbol_set(set)[idx];
                        let region = puzzle.region(region);
                        let cell = puzzle.cell_info(cell);
                        return LogicalStep::Acted(format!(
                            "Hidden single {value} at {cell} in {region}"
                        ));
                    }
                }
            }
        }

        LogicalStep::NoAction
    }
}
