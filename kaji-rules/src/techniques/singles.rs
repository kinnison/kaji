use kaji::{CellIndex, LogicalStep, SolveState, SymbolSetId, Technique};

#[derive(Debug)]
pub struct NakedSingle {
    set: SymbolSetId,
}

impl NakedSingle {
    pub fn new(set: SymbolSetId) -> Self {
        Self { set }
    }
}

impl Technique for NakedSingle {
    fn logical_step(&self, state: &mut SolveState) -> LogicalStep {
        for cell in state.all_cells() {
            let choice = state.choices(cell, self.set);
            if !choice.solved() {
                if let Some(value) = choice.single_value() {
                    // We're not solved and there's only one thing we can be
                    state.set_symbol(cell, value);
                    let cell = state.cell_info(cell);
                    let value = state.symbol(value);
                    return LogicalStep::Acted(format!("Naked single {value} at {cell}"));
                }
            }
        }

        LogicalStep::NoAction
    }
}

#[derive(Debug)]
pub struct HiddenSingle {
    set: SymbolSetId,
}

impl HiddenSingle {
    pub fn new(set: SymbolSetId) -> Self {
        Self { set }
    }
}

impl Technique for HiddenSingle {
    fn logical_step(&self, state: &mut SolveState) -> LogicalStep {
        #[derive(Clone)]
        enum SymbolState {
            Unknown,
            Single(CellIndex),
            Many,
        }
        use SymbolState::*;
        for region in state.regions() {
            let cells = state.region(region).to_cells();
            let symbolids = state.symbols(self.set).collect::<Vec<_>>();
            let mut symbols = vec![Unknown; symbolids.len()];
            'cells: for cell in cells.iter().copied() {
                let choice = state.choices(cell, self.set);
                if choice.solved() {
                    continue 'cells;
                }
                for symbolnr in choice.options() {
                    symbols[symbolnr.symbol_index()] = match symbols[symbolnr.symbol_index()] {
                        Unknown => Single(cell),
                        _ => Many,
                    };
                }
            }
            for (idx, symbol) in symbols.into_iter().enumerate() {
                if let Single(cell) = symbol {
                    let symbol = symbolids[idx];
                    state.set_symbol(cell, symbol);
                    let value = state.symbol(symbol);
                    let region = state.region(region);
                    let cell = state.cell_info(cell);
                    return LogicalStep::Acted(format!(
                        "Hidden single {value} at {cell} in {region}"
                    ));
                }
            }
        }

        LogicalStep::NoAction
    }
}
