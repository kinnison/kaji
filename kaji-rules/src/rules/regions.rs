use kaji::{PuzzleBuilder, RegionId, Rule, SymbolSetId};

pub struct NonRepeatRegion {
    region: RegionId,
    symbols: SymbolSetId,
}

impl NonRepeatRegion {
    pub fn new(region: RegionId, symbols: SymbolSetId) -> Self {
        Self { region, symbols }
    }
}

impl Rule for NonRepeatRegion {
    fn apply(&self, builder: &mut PuzzleBuilder) {
        let symbols = builder.symbols(self.symbols).collect::<Vec<_>>();
        let region = builder.region(self.region).to_cells();
        for cell0 in 0..(region.len() - 1) {
            for symbol in symbols.iter().copied() {
                for cell1 in cell0 + 1..region.len() {
                    builder.add_inference(region[cell0], symbol, region[cell1], symbol);
                }
            }
        }
    }
}
