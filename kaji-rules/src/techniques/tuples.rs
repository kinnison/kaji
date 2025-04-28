use std::collections::{HashMap, HashSet};

use kaji::{CellIndex, LogicalStep, SolveState, SymbolChoice, SymbolId, SymbolSetId, Technique};

use itertools::Itertools;

#[derive(Debug)]
pub struct NakedTuple {
    set: SymbolSetId,
    max_n: usize,
}

impl NakedTuple {
    pub fn new(set: SymbolSetId, max_n: usize) -> Self {
        Self { set, max_n }
    }
}

impl Technique for NakedTuple {
    fn logical_step(&self, state: &mut SolveState) -> LogicalStep {
        for region in state.regions() {
            let unsolved_cells = state
                .region(region)
                .to_cells()
                .into_iter()
                .filter(|cell| !state.choices(*cell, self.set).solved())
                .collect::<Vec<_>>();
            if unsolved_cells.len() < 3 {
                continue;
            }
            for tsize in 2..(self.max_n + 1).min(unsolved_cells.len()) {
                for cells in unsolved_cells.iter().copied().combinations(tsize) {
                    let found_tuple = cells
                        .iter()
                        .copied()
                        .map(|cell| state.choices(cell, self.set))
                        .reduce(|acc, choice| acc | choice)
                        .expect("Unable to reduce symbol choice set");
                    if found_tuple.options().count() != tsize {
                        continue;
                    }
                    let region = state.region(region);
                    let mut acted =
                        LogicalStep::action(format!("Naked {tsize}-tuple in {region}: "));
                    acted.push_cells(cells.iter().copied().map(|cell| state.cell_info(cell)));
                    acted.push_str("; removed ");
                    acted.push_symbols(found_tuple.options().map(|sym| state.symbol(sym)));
                    acted.push_str(" from ");
                    let mut changed = HashSet::new();
                    for other_cell in unsolved_cells
                        .iter()
                        .copied()
                        .filter(|cell| !cells.contains(cell))
                    {
                        for elim in found_tuple.options() {
                            if state.eliminate(other_cell, elim).changed() {
                                changed.insert(other_cell);
                            }
                        }
                    }
                    if !changed.is_empty() {
                        acted.push_cells(changed.into_iter().map(|cell| state.cell_info(cell)));
                        return acted;
                    }
                }
            }
        }
        LogicalStep::NoAction
    }
}

#[derive(Debug)]
pub struct HiddenTuple {
    set: SymbolSetId,
    max_n: usize,
}

impl HiddenTuple {
    pub fn new(set: SymbolSetId, max_n: usize) -> Self {
        Self { set, max_n }
    }
}

impl Technique for HiddenTuple {
    fn logical_step(&self, state: &mut SolveState) -> LogicalStep {
        for region in state.regions() {
            let unsolved_cells = state
                .region(region)
                .to_cells()
                .into_iter()
                .filter(|cell| !state.choices(*cell, self.set).solved())
                .collect::<HashSet<_>>();
            if unsolved_cells.len() < 3 {
                continue;
            }
            for tsize in 2..(self.max_n + 1).min(unsolved_cells.len()) {
                'cell_tuple: for cells in unsolved_cells.iter().copied().combinations(tsize) {
                    let cells = cells.into_iter().collect::<HashSet<_>>();
                    let found_tuple = cells
                        .iter()
                        .copied()
                        .map(|cell| state.choices(cell, self.set))
                        .reduce(|acc, choice| acc & choice)
                        .expect("Unable to reduce symbol choice set");
                    if found_tuple.options().count() != cells.len() {
                        continue;
                    }
                    // We have found_tuple present in cells
                    // We now check that none of it is present in other unsolved cells
                    for cell in unsolved_cells.difference(&cells).copied() {
                        if (state.choices(cell, self.set) & found_tuple)
                            .options()
                            .next()
                            .is_some()
                        {
                            // Some of our found_tuple is present outside of cells
                            continue 'cell_tuple;
                        }
                    }
                    // We should eliminate anything not in that tuple from those cells
                    let region = state.region(region);
                    let mut acted =
                        LogicalStep::action(format!("Hidden {tsize}-tuple in {region}: "));
                    acted.push_cells(cells.iter().map(|cell| state.cell_info(*cell)));
                    acted.push_str(" removed all candidates other than ");
                    acted.push_symbols(found_tuple.options().map(|symbol| state.symbol(symbol)));
                    let mut changed = false;
                    for cell in cells {
                        changed |= state.restrict(cell, found_tuple).changed();
                    }
                    if changed {
                        return acted;
                    }
                }
            }
        }
        LogicalStep::NoAction
    }
}
