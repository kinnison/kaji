use std::collections::{HashMap, HashSet};

use kaji::{CellIndex, LogicalStep, SolveState, SymbolChoice, SymbolId, SymbolSetId, Technique};

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
            let region_cells = state
                .region(region)
                .to_cells()
                .into_iter()
                .collect::<HashSet<_>>();
            let mut choices: HashMap<SymbolChoice, HashSet<CellIndex>> = HashMap::new();
            for cell in region_cells.iter().copied() {
                let choice = state.choices(cell, self.set);
                if !choice.solved() {
                    choices.entry(choice).or_default().insert(cell);
                }
            }
            // TODO: Ideally we need to take groups of symbol choices
            // union them together, and check the cell count.
            // otherwise a triple of ab abc bc doesn't work.
            for (choice, cells) in choices {
                if choice.options().count() == cells.len() {
                    // The n cells in cells all share the same n options
                    let region = state.region(region);
                    let mut action = format!("{}-tuple found in {region} (", cells.len());
                    for cell in cells.iter().copied() {
                        let cell = state.cell_info(cell);
                        action.push_str(&format!("{cell}, "));
                    }
                    action.pop();
                    action.pop();
                    action.push_str(") eliminates ");
                    let mut changed = false;
                    for cell in region_cells.difference(&cells).copied() {
                        for option in choice.options() {
                            let did = state.eliminate(cell, option).changed();
                            if did {
                                let symbol = state.symbol(option);
                                let cell = state.cell_info(cell);
                                action.push_str(&format!("{symbol} from {cell}, "));
                                changed = true;
                            }
                        }
                    }
                    action.pop();
                    action.pop();
                    if changed {
                        return LogicalStep::Acted(action);
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
