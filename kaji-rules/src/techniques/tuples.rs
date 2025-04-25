use std::collections::{HashMap, HashSet};

use kaji::{CellIndex, LogicalStep, SolveState, SymbolChoice, SymbolSetId, Technique};

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
