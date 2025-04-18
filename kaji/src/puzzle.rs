use crate::constraints::*;
use crate::symbols::*;

use std::collections::HashMap;
use std::fmt::Display;
use std::usize;

#[derive(Debug)]
pub struct Puzzle {
    symbols: Vec<SymbolSet>,
    constraints: Vec<Box<dyn Constraint>>,
    cells: Vec<CellInfo>,
    regions: Vec<Region>,
    implications: HashMap<(CellIndex, SymbolId), Vec<(CellIndex, SymbolId)>>,
    rowcols: HashMap<(usize, usize), CellIndex>,
}

#[derive(Debug)]
pub struct CellInfo {
    name: String,
    row: usize,
    col: usize,
}

#[derive(Debug)]
pub struct Region {
    name: String,
    cells: Vec<CellIndex>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CellIndex(usize);

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct RegionIndex(usize);

/// A board is the cell state of the grid during a puzzle solve
///
/// For example, it will contain the found digits and the results of
/// any implications they have.  It is meant to be moderately cheap
/// to clone, so every cell in the board consists of a SymbolChoice for each
/// symbol set the cell could have.
#[derive(Debug)]
pub struct Board {
    ncells: usize,
    nsymbols: usize,
    cells: Vec<SymbolChoice>,
}

impl Puzzle {
    pub fn new_sudoku(size: usize) -> Puzzle {
        assert_eq!(size, 6);
        let mut digits = SymbolSet::new("digits");
        digits.push("1");
        digits.push("2");
        digits.push("3");
        digits.push("4");
        digits.push("5");
        digits.push("6");
        let mut cells = Vec::new();
        let mut rowcols = HashMap::new();
        for row in 1..=6 {
            for col in 1..=6 {
                let name = format!("r{row}c{col}");
                cells.push(CellInfo { name, row, col });
                rowcols.insert((row, col), CellIndex(cells.len() - 1));
            }
        }
        let mut regions = Vec::new();
        for n in 1..=6 {
            let rowregion = Region {
                name: format!("Row {n}"),
                cells: rowcols
                    .iter()
                    .filter_map(|v| if v.0 .0 == n { Some(*v.1) } else { None })
                    .collect(),
            };
            regions.push(rowregion);
            let colregion = Region {
                name: format!("Column {n}"),
                cells: rowcols
                    .iter()
                    .filter_map(|v| if v.0 .1 == n { Some(*v.1) } else { None })
                    .collect(),
            };
            regions.push(colregion);
            // Boxes are a bit more of a pain
            let boxrow = ((n - 1) & !1) + 1;
            let boxcol = 1 + (3 * ((n - 1) & 1));
            let mut boxregion = Region {
                name: format!("Box {n}"),
                cells: vec![],
            };
            for row in boxrow..=boxrow + 1 {
                for col in boxcol..=boxcol + 2 {
                    boxregion.cells.push(rowcols[&(row, col)]);
                }
            }
            regions.push(boxregion);
        }
        let mut ret = Self {
            symbols: vec![digits],
            constraints: vec![],
            cells,
            regions,
            implications: HashMap::new(),
            rowcols,
        };

        let digits = ret.symbols[0].to_ids(0).collect::<Vec<_>>();
        for region in ret.regions() {
            let cells = ret.region(region).to_cells();
            for cell0 in 0..(cells.len() - 1) {
                for digit in &digits {
                    for cell1 in cell0 + 1..cells.len() {
                        ret.add_inference(cells[cell0], *digit, cells[cell1], *digit);
                    }
                }
            }
        }

        ret
    }

    pub fn regions(&self) -> impl Iterator<Item = RegionIndex> {
        (0..self.regions.len()).map(RegionIndex)
    }

    pub fn region(&self, region: RegionIndex) -> &Region {
        &self.regions[region.0]
    }

    pub fn add_inference(
        &mut self,
        cell0: CellIndex,
        has: SymbolId,
        cell1: CellIndex,
        lacks: SymbolId,
    ) {
        // if cell0 is has, cell1 cannot be lacks
        // corollary: if cell1 is lacks, cell0 cannot be has
        self.implications
            .entry((cell0, has))
            .or_default()
            .push((cell1, lacks));
        self.implications
            .entry((cell1, lacks))
            .or_default()
            .push((cell0, has));
    }

    pub fn initial_board(&self) -> Board {
        let mut board = Board::empty(self.cells.len(), &self.symbols);
        for constraint in &self.constraints {
            constraint.prep_board(self, &mut board);
        }
        board
    }

    pub fn print_board(&self, board: &Board) {
        assert_eq!(self.cells.len() * self.symbols.len(), board.cells.len());
        let mut minrow = usize::MAX;
        let mut mincol = usize::MAX;
        let mut maxrow = usize::MIN;
        let mut maxcol = usize::MIN;
        for &(row, col) in self.rowcols.keys() {
            minrow = minrow.min(row);
            mincol = mincol.min(col);
            maxrow = maxrow.max(row);
            maxcol = maxcol.max(col);
        }
        let cellwidth: usize = self.symbols.iter().map(|s| s.width()).sum();
        for row in minrow..=maxrow {
            for col in mincol..=maxcol {
                if let Some(cell) = self.rowcols.get(&(row, col)).copied() {
                    for (set, choice) in board.choices(cell).enumerate() {
                        if let Some(value) = choice.single_value() {
                            let symbol = &self.symbols[set][value];
                            print!("{symbol}");
                        } else {
                            for _ in 0..self.symbols[set].width() {
                                print!("?");
                            }
                        }
                    }
                } else {
                    print!("{:cellwidth$}", " ");
                }
            }
            println!();
        }
    }

    pub fn add_constraint<C: Constraint + 'static>(&mut self, constraint: C) {
        self.constraints.push(Box::new(constraint));
    }

    pub fn symbol_set(&self, idx: usize) -> &SymbolSet {
        &self.symbols[idx]
    }

    pub fn cell_at(&self, row: usize, col: usize) -> Option<CellIndex> {
        self.rowcols.get(&(row, col)).copied()
    }

    pub fn symbols(&self, set: usize) -> impl Iterator<Item = SymbolId> {
        self.symbols[set].to_ids(set)
    }

    pub fn set_symbol(&self, board: &mut Board, cell: CellIndex, symbol: SymbolId) {
        let (_set, symbolnr) = symbol.into_parts();
        let choice = board.choice_mut(cell, symbol);
        assert!(choice.can_be(symbolnr));
        choice.set(symbolnr);
        self.propagate_changes(board, cell);
    }

    pub fn propagate_changes(&self, board: &mut Board, cell: CellIndex) {
        for set in 0..self.symbols.len() {
            if let Some(symbol) = board.choice_set(cell, set).single_value() {
                let symbol = self.symbols[set].id(set, symbol);
                if let Some(implications) = self.implications.get(&(cell, symbol)) {
                    for &(othercell, lacks) in implications {
                        board
                            .choice_mut(othercell, lacks)
                            .unset(lacks.into_parts().1);
                    }
                }
            }
        }
    }

    pub fn all_cells(&self) -> impl Iterator<Item = CellIndex> {
        (0..self.cells.len()).map(CellIndex)
    }

    pub fn symbol_sets(&self) -> impl Iterator<Item = usize> {
        0..self.symbols.len()
    }

    pub fn cell_info(&self, cell: CellIndex) -> &CellInfo {
        &self.cells[cell.0]
    }

    pub fn logical_step(&self, board: &mut Board) -> LogicalStep {
        let mut finished = true;
        for constraint in &self.constraints {
            match constraint.logical_step(self, board) {
                LogicalStep::NoAction => {
                    finished = false;
                }
                LogicalStep::Finished => {}
                ls => return ls,
            }
        }
        if finished || board.solved() {
            LogicalStep::Finished
        } else {
            LogicalStep::NoAction
        }
    }

    pub fn solve(&self) -> Board {
        let mut board = self.initial_board();
        loop {
            match self.logical_step(&mut board) {
                LogicalStep::Acted(s) => {
                    println!("{s}");
                }
                LogicalStep::NoAction => {
                    println!("Failed to solve!");
                    break;
                }
                LogicalStep::Finished => {
                    println!("Finished");
                    break;
                }
            }
        }
        board
    }
}

impl Region {
    pub fn to_cells(&self) -> Vec<CellIndex> {
        self.cells.clone()
    }
}

impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)
    }
}

impl Board {
    fn empty(ncells: usize, symbols: &[SymbolSet]) -> Self {
        let mut cells = Vec::with_capacity(ncells * symbols.len());
        for _cell in 0..ncells {
            for symbols in symbols {
                cells.push(SymbolChoice::any(symbols.len()));
            }
        }
        Self {
            ncells,
            nsymbols: symbols.len(),
            cells,
        }
    }

    pub fn solved(&self) -> bool {
        self.cells.iter().all(|c| c.solved())
    }

    pub fn choices(&self, idx: CellIndex) -> impl Iterator<Item = SymbolChoice> + use<'_> {
        let idx = idx.0 * self.nsymbols;
        assert!(idx + self.nsymbols <= self.cells.len());
        self.cells.iter().skip(idx).take(self.nsymbols).copied()
    }

    pub fn choice(&self, idx: CellIndex, symbol: SymbolId) -> SymbolChoice {
        let idx = idx.0 * self.nsymbols + symbol.into_parts().0;
        self.cells[idx]
    }

    fn choice_mut(&mut self, idx: CellIndex, symbol: SymbolId) -> &mut SymbolChoice {
        let idx = idx.0 * self.nsymbols + symbol.into_parts().0;
        &mut self.cells[idx]
    }

    pub fn choice_set(&self, idx: CellIndex, set: usize) -> SymbolChoice {
        let idx = idx.0 * self.nsymbols + set;
        self.cells[idx]
    }
}

impl Display for CellInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)
    }
}
