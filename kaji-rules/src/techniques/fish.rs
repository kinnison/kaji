use std::collections::HashSet;

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
            for (ridx1, region1) in regions.iter().copied().enumerate() {
                let mut possible_cells1 = HashSet::new();
                for cell in state.region(region1).to_cells() {
                    let choice = state.choices(cell, self.set);
                    if !choice.solved() && choice.can_be(symbol) {
                        possible_cells1.insert(cell);
                    }
                }
                if possible_cells1.len() != self.size {
                    // This region is not suitable
                    continue;
                }
                'region2: for region2 in regions[ridx1 + 1..].iter().copied() {
                    let mut possible_cells2 = HashSet::new();
                    for cell in state.region(region2).to_cells() {
                        let choice = state.choices(cell, self.set);
                        if !choice.solved() && choice.can_be(symbol) {
                            possible_cells2.insert(cell);
                        }
                    }
                    if possible_cells2.len() != self.size {
                        // This region is not suitable
                        continue;
                    }
                    if possible_cells1.intersection(&possible_cells2).count() > 0 {
                        // The regions managed to overlap somehow
                        continue;
                    }
                    // Here we know:
                    // (a) region1 and region2 are of the correct size
                    // (b) they both contain self.size cells which could be symbol
                    // Next determine if we have the fish, ie for each cell in region1
                    // is there a single unique cell in region2 which it can see?
                    let mut saw2 = HashSet::new();
                    let mut region1_sees = HashSet::new();
                    for cell in possible_cells1.iter().copied() {
                        let sees = state.sees(cell, symbol).collect::<HashSet<_>>();
                        let saw = possible_cells2
                            .intersection(&sees)
                            .copied()
                            .collect::<Vec<_>>();
                        if saw.len() != 1 {
                            // We saw either too much, or too little, let's carry on
                            continue 'region2;
                        }
                        if saw2.contains(&saw[0]) {
                            // We already saw this
                            continue 'region2;
                        }
                        saw2.insert(saw[0]);
                        region1_sees.extend(sees);
                    }
                    // Now we have region1_sees which is all the cells region1's cells can see
                    for cell in possible_cells1.iter() {
                        region1_sees.remove(cell);
                    }
                    let mut region2_sees =
                        possible_cells2
                            .iter()
                            .copied()
                            .fold(HashSet::new(), |mut acc, cell| {
                                acc.extend(state.sees(cell, symbol));
                                acc
                            });
                    for cell in possible_cells2.iter() {
                        region2_sees.remove(cell);
                    }
                    // Now we have region1_sees and region2_sees
                    let possible_elim_cells = region1_sees.intersection(&region2_sees);
                    let mut action = LogicalStep::action(format!("{} on ", fish_name(self.size)));
                    action.push_symbols(Some(state.symbol(symbol)));
                    action.push_str(" at ");
                    action.push_cells(possible_cells1.iter().map(|c| state.cell_info(*c)));
                    action.push_str(" and ");
                    action.push_cells(possible_cells2.iter().map(|c| state.cell_info(*c)));
                    action.push_str(" eliminates ");
                    action.push_symbols(Some(state.symbol(symbol)));
                    action.push_str(" from ");
                    let mut cleared = Vec::new();
                    for cell in possible_elim_cells.copied() {
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
        }

        LogicalStep::NoAction
    }
}
