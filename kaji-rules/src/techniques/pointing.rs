//! Pointing symbols

use std::collections::HashSet;

use kaji::{consts::DIFFICULTY_EASY, LogicalStep, RegionId, SolveState, SymbolSetId, Technique};

#[derive(Debug)]
pub struct PointingSymbol {
    set: SymbolSetId,
}

impl PointingSymbol {
    pub fn new(set: SymbolSetId) -> Self {
        Self { set }
    }
}

impl Technique for PointingSymbol {
    fn logical_step(&self, state: &mut SolveState) -> LogicalStep {
        for region in state.regions() {
            for symbol in state.symbols(self.set) {
                let cells_for_symbol = state
                    .region(region)
                    .to_cells()
                    .into_iter()
                    .filter(|cell| state.choices(*cell, self.set).can_be(symbol))
                    .collect::<HashSet<_>>();
                let mut regions = cells_for_symbol
                    .iter()
                    .copied()
                    .map(|cell| state.regions_for_cell(cell).collect::<HashSet<RegionId>>())
                    .reduce(|acc, regions| {
                        acc.intersection(&regions)
                            .copied()
                            .collect::<HashSet<RegionId>>()
                    })
                    .expect("Something went wrong collection regions");
                regions.remove(&region);
                if let Some(other_region) = regions.into_iter().next() {
                    // We found another region which symbol X points at.
                    // So we want to eliminate symbol from all *other* cells
                    // in that region
                    let region = state.region(region);
                    let mut acted = LogicalStep::action(format!("Pointing cells in {region}: "));
                    acted.push_cells(cells_for_symbol.iter().map(|cell| state.cell_info(*cell)));
                    acted.push_str(" removed ");
                    acted.push_symbols(Some(state.symbol(symbol)));
                    acted.push_str(" from ");
                    let mut changed = HashSet::new();
                    for cell in state.region(other_region).to_cells() {
                        if !cells_for_symbol.contains(&cell)
                            && state.eliminate(cell, symbol).changed()
                        {
                            changed.insert(cell);
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

    fn difficulty(&self) -> u16 {
        DIFFICULTY_EASY
    }
}
