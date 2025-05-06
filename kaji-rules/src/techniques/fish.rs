use std::collections::HashSet;

use itertools::Itertools;
use kaji::{LogicalStep, SolveState, SymbolSetId, Technique};

#[derive(Debug)]
pub struct Fish {
    size: usize,
    set: SymbolSetId,
}

fn fish_name(size: usize) -> &'static str {
    match size {
        2 => "x-wing",
        3 => "swordfish",
        4 => "jellyfish",
        _ => unimplemented!(),
    }
}

impl Fish {
    pub fn new(size: usize, set: SymbolSetId) -> Self {
        assert!(
            (2..=4).contains(&size),
            "We only know x-wing, swordfish, and jellyfish"
        );
        Self { size, set }
    }
}

impl Technique for Fish {
    fn logical_step(&self, state: &mut SolveState) -> LogicalStep {
        let house_size = state.symbols(self.set).count();
        let regions = state
            .regions()
            .filter(|region| state.region(*region).len() == house_size)
            .collect::<Vec<_>>();
        for symbol in state.symbols(self.set) {
            'region_set: for regions_to_use in regions.iter().copied().combinations(self.size) {
                let possible_cells = regions_to_use
                    .iter()
                    .copied()
                    .map(|region| {
                        state
                            .region(region)
                            .to_cells()
                            .into_iter()
                            .filter(|&cell| {
                                let choice = state.choices(cell, self.set);
                                !choice.solved() && choice.can_be(symbol)
                            })
                            .collect::<HashSet<_>>()
                    })
                    .collect::<Vec<_>>();
                // Check if all regions have the right possibility count
                if possible_cells.iter().any(|r| r.len() != self.size) {
                    continue 'region_set;
                }
                for (r1, r2) in possible_cells.iter().tuple_combinations() {
                    // Check if any two of the regions picked have overlapping cells
                    if r1.intersection(r2).count() > 0 {
                        continue 'region_set;
                    }
                    // No overlapping cells, do we have unique seeing pairs?
                    let mut seen = HashSet::new();
                    for r1cell in r1.iter().copied() {
                        let mut cell_seen = None;
                        for r2cell in r2.iter().copied() {
                            if state.can_see(r1cell, symbol, r2cell, symbol) {
                                if cell_seen.is_some() {
                                    // Overlapped
                                    continue 'region_set;
                                }
                                cell_seen = Some(r2cell);
                            }
                        }
                        seen.extend(cell_seen);
                    }
                    if &seen != r2 {
                        // We did not see every cell in r2 from the cells in r1
                        continue 'region_set;
                    }
                }
                // Now we know:
                // We have the right number of regions
                // Each region has the right number of possibilities
                // Each cell in a region sees a unique set of cells in each other region
                // Which means we can perform the union/intersection work
                let sees = possible_cells
                    .iter()
                    .map(|rcells| {
                        let mut acc =
                            rcells
                                .iter()
                                .copied()
                                .fold(HashSet::new(), |mut acc, cell| {
                                    acc.extend(state.sees(cell, symbol));
                                    acc
                                });
                        rcells.iter().for_each(|cell| {
                            acc.remove(cell);
                        });
                        acc
                    })
                    .reduce(|acc, elem| acc.intersection(&elem).copied().collect())
                    .expect("No region cells?");
                // sees is now the set of cells seen by all the unions of the input region cells

                let mut action = LogicalStep::action(format!("{} on ", fish_name(self.size)));
                action.push_symbols(Some(state.symbol(symbol)));
                action.push_str(" at ");
                action.push_cells(possible_cells[0].iter().map(|c| state.cell_info(*c)));
                for possible_cells2 in possible_cells[1..].iter() {
                    action.push_str(" and ");
                    action.push_cells(possible_cells2.iter().map(|c| state.cell_info(*c)));
                }
                action.push_str(" eliminates ");
                action.push_symbols(Some(state.symbol(symbol)));
                action.push_str(" from ");
                let mut cleared = Vec::new();
                for cell in sees.into_iter() {
                    if state.eliminate(cell, symbol).changed() {
                        cleared.push(cell);
                    }
                }
                if !cleared.is_empty() {
                    action.push_cells(cleared.into_iter().map(|cell| state.cell_info(cell)));
                    return action;
                }
            }
        }

        LogicalStep::NoAction
    }
}
