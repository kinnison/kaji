//! Indexer cells
//!

use itertools::Itertools;
use kaji::{consts::DIFFICULTY_EASY, CellIndex, Constraint, LogicalStep, RegionId, SolveState};

#[derive(Debug, Clone, Copy)]
pub enum SudokuIndexerKind {
    Row,
    Column,
    Box,
}

#[derive(Debug)]
pub struct Indexer {
    cell: CellIndex,
    region: RegionId,
}

impl Indexer {
    pub fn new(cell: CellIndex, region: RegionId) -> Self {
        Self { cell, region }
    }
}

impl Constraint for Indexer {
    fn difficulty(&self) -> u16 {
        DIFFICULTY_EASY + 1000
    }

    fn prep_board(&self, _state: &mut SolveState) {
        // Nothing to do here
    }

    fn logical_step(&self, state: &mut SolveState) -> LogicalStep {
        // Two things that we can do.
        // 1. If we are solved already then we can eliminate our index in the region from anything but
        // the cell we reference
        // 2. If we are not solved already then we can eliminate any value from us whose cell cannot be our index

        let our_values = state.cell_values(self.cell).collect_vec();
        let our_region = state.region(self.region).to_cells();
        let our_index = our_region
            .iter()
            .find_position(|c| **c == self.cell)
            .unwrap()
            .0 as i32;

        if our_values.len() == 1 {
            for (oci, other_cell) in our_region.iter().copied().enumerate() {
                if oci as i32 == our_values[0].value() - 1 {
                    // This is the cell we are referencing, so skip eliminations
                    continue;
                }
                let elim_values = state
                    .cell_values(other_cell)
                    .filter(|v| v.value() == our_index + 1)
                    .collect_vec();
                if elim_values.is_empty() {
                    // Nothing to do for this cell
                    continue;
                }
                assert_eq!(
                    elim_values[0].symbols().len(),
                    1,
                    "We only work with single symbol sets"
                );
                let mut action =
                    LogicalStep::action(format!("Indexer at {}", state.cell_info(self.cell)));
                action.push_str(" eliminates ");
                for elim in elim_values {
                    let sym_e = elim.symbols().pop().unwrap(); // 1 symbol only
                    if state.eliminate(other_cell, sym_e).changed() {
                        action.push_symbols(Some(state.symbol(sym_e)));
                        action.push_str(" from ");
                        action.push_cells(Some(state.cell_info(other_cell)));
                        return action;
                    }
                }
            }
        } else {
            // Instead of eliminating from elsewhere, we eliminate from ourselves based
            // on the possible values we might be able to be.
            for value in our_values {
                let can_eliminate =
                    if value.value() < 1 || value.value() as usize > our_region.len() {
                        true
                    } else {
                        let other_cell = our_region[value.value() as usize - 1];
                        let wanted_value = our_index + 1;
                        !state
                            .cell_values(other_cell)
                            .any(|v| v.value() == wanted_value)
                    };
                if can_eliminate {
                    let mut action =
                        LogicalStep::action(format!("Indexer at {}", state.cell_info(self.cell)));
                    action.push_str(" self eliminates ");
                    assert_eq!(
                        value.symbols().len(),
                        1,
                        "Can only operate on single symbol sets"
                    );
                    let sym_e = value.symbols().pop().unwrap();
                    action.push_symbols(Some(state.symbol(sym_e)));
                    if state.eliminate(self.cell, sym_e).changed() {
                        return action;
                    }
                }
            }
        }

        LogicalStep::NoAction
    }
}
