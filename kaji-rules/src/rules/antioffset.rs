//! This Rule implements things like antiknight, antiking etc.
//!

use itertools::Itertools;
use kaji::{CellIndex, PuzzleBuilder, Rule, SymbolSetId};

#[derive(Debug)]
pub struct AntiOffset {
    offset1: i32,
    offset2: i32,
    cells: Vec<CellIndex>,
    symbols: SymbolSetId,
}

impl AntiOffset {
    pub fn new(
        offset1: i32,
        offset2: i32,
        symbols: SymbolSetId,
        cells: impl IntoIterator<Item = CellIndex>,
    ) -> Self {
        assert!(offset1 > 0);
        assert!(offset2 > 0);
        Self {
            offset1,
            offset2,
            symbols,
            cells: cells.into_iter().collect_vec(),
        }
    }
}

impl Rule for AntiOffset {
    fn apply(&self, builder: &mut PuzzleBuilder) {
        let all_offsets = {
            let (o1, o2) = (self.offset1, self.offset2);
            if o1 == o2 {
                vec![(o1, o1), (-o1, o1), (o1, -o1), (-o1, -o1)]
            } else {
                vec![
                    (o1, o2),
                    (-o1, o2),
                    (o1, -o2),
                    (-o1, -o2),
                    (o2, o1),
                    (-o2, o1),
                    (o2, -o1),
                    (-o2, -o1),
                ]
            }
        };
        let all_symbols = builder.symbols(self.symbols).collect::<Vec<_>>();
        for cell in self.cells.iter().copied() {
            let (row, col) = {
                let ci = builder.cell_info(cell);
                (ci.row() as i32, ci.col() as i32)
            };
            for (rofs, cofs) in all_offsets.iter().copied() {
                let (nrow, ncol) = (row + rofs, col + cofs);
                if let Some(other) = builder.cell_at(nrow as usize, ncol as usize) {
                    for symbol in all_symbols.iter().copied() {
                        builder.add_inference(cell, symbol, other, symbol);
                    }
                }
            }
        }
    }
}
