use std::collections::{HashMap, HashSet};

use kaji::{CellIndex, LogicalStep, SolveState, SymbolChoice, SymbolId, SymbolSetId, Technique};

use itertools::Itertools;

#[derive(Debug)]
pub struct NakedTuple {
    set: SymbolSetId,
}

impl NakedTuple {
    pub fn new(set: SymbolSetId) -> Self {
        Self { set }
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
            for tsize in 2..unsolved_cells.len() {
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
}

impl HiddenTuple {
    pub fn new(set: SymbolSetId) -> Self {
        Self { set }
    }
}

impl Technique for HiddenTuple {
    fn logical_step(&self, state: &mut SolveState) -> LogicalStep {
        let symbols = state.symbols(self.set).collect::<Vec<_>>();
        for region in state.regions() {
            let mut symmap: HashMap<SymbolId, HashSet<CellIndex>> = HashMap::new();
            let cells = state.region(region).to_cells();
            for cell in cells.iter().copied() {
                for option in state.choices(cell, self.set).options() {
                    symmap.entry(option).or_default().insert(cell);
                }
            }

            // TODO: HiddenTuple can only see pairs with this horrid code.
            // We should improve it.
            for symbol0 in 0..symbols.len() - 1 {
                for symbol1 in symbol0 + 1..symbols.len() {
                    let cells0 = &symmap[&symbols[symbol0]];
                    let cells1 = &symmap[&symbols[symbol1]];
                    if cells0.len() == 2
                        && cells1.len() == 2
                        && cells0 == cells1
                        && cells0
                            .iter()
                            .copied()
                            .any(|cell| state.choices(cell, self.set).options().count() > 2)
                    {
                        // Hidden pair
                        let cells = cells0.iter().copied().collect::<Vec<_>>();
                        let acted = format!(
                            "Hidden pair in {}: {} and {} eliminated all but {} and {}",
                            state.region(region),
                            state.cell_info(cells[0]),
                            state.cell_info(cells[1]),
                            state.symbol(symbols[symbol0]),
                            state.symbol(symbols[symbol1])
                        );
                        for cell in cells {
                            for option in state.choices(cell, self.set).options() {
                                if option != symbols[symbol0] && option != symbols[symbol1] {
                                    state.eliminate(cell, option);
                                }
                            }
                        }
                        return LogicalStep::Acted(acted);
                    }
                }
            }
        }
        LogicalStep::NoAction
    }
}
