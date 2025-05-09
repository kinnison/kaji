use crate::constraints::{Constraint, LogicalStep};
use crate::{symbols::*, Technique};

use std::collections::HashMap;
use std::fmt::Display;
use std::ops::{BitAnd, BitOr};

#[derive(Debug)]
/// A representation of a puzzle which may be solved
pub struct Puzzle {
    symbols: Vec<RawSymbolSet>,
    constraints: Vec<Box<dyn Constraint>>,
    techniques: Vec<Box<dyn Technique>>,
    cells: Vec<CellInfo>,
    cell_regions: Vec<Vec<RegionId>>,
    regions: Vec<Region>,
    implications: HashMap<(CellIndex, SymbolId), Vec<(CellIndex, SymbolId)>>,
    rowcols: HashMap<(usize, usize), CellIndex>,
}

#[derive(Debug, Default)]
/// A way to construct [`Puzzle`]s piece by piece
pub struct PuzzleBuilder {
    symbols: Vec<RawSymbolSet>,
    constraints: Vec<Box<dyn Constraint>>,
    techniques: Vec<Box<dyn Technique>>,
    cells: Vec<CellInfo>,
    cell_regions: Vec<Vec<RegionId>>,
    regions: Vec<Region>,
    implications: HashMap<(CellIndex, SymbolId), Vec<(CellIndex, SymbolId)>>,
    rowcols: HashMap<(usize, usize), CellIndex>,
}

/// Information about a cell in a [`Puzzle`]
///
/// Cells are in an orthogonal grid, but are not necessarily tightly
/// packed.  Each cell has a row and column number associated, and
/// that is a unique coordinate for the cell.
#[derive(Debug)]
pub struct CellInfo {
    name: String,
    row: usize,
    col: usize,
}

/// A set of cells with some kind of uniqueness constraint
///
/// A region is a set of cells where one or more of the
/// [`SymbolSet`][SymbolSetId]s must be unique within the
/// region for the puzzle to be considered solved.
#[derive(Debug)]
pub struct Region {
    name: String,
    cells: Vec<CellIndex>,
}

/// An index into the [cells][CellInfo] in a [`Puzzle`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CellIndex(usize);

/// An index into the [`Region`]s in a [`Puzzle`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct RegionId(usize);

/// An index into the `SymbolSets` in a [`Puzzle`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SymbolSetId(usize);

/// A representation of an in-progress solve.
///
/// In practice this encapsulates a [`&Puzzle`][Puzzle]
/// and an in-progress [`Board`]
pub struct SolveState<'p> {
    puzzle: &'p Puzzle,
    board: Board,
}

/// A representation of which [`Symbol`]s are available.
///
/// Within a [`Board`] each cell has a [`SymbolChoice`]
/// for each [`SymbolSet`][SymbolSetId].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SymbolChoice {
    set: SymbolSetId,
    choice: RawSymbolChoice,
}

/// A board is the cell state of the grid during a puzzle solve
///
/// For example, it will contain the found digits and the results of
/// any implications they have.  It is meant to be moderately cheap
/// to clone, so every cell in the board consists of a SymbolChoice for each
/// symbol set the cell could have.
#[derive(Debug)]
pub struct Board {
    nsymbols: usize,
    cells: Vec<RawSymbolChoice>,
}

/// Whether or not an action actually altered the board
#[derive(Debug, Clone, Copy)]
pub enum Effect {
    /// Nothing changed on the [`Board`]
    Unchanged,
    /// Something changed on the [`Board`]
    Changed,
}

impl Effect {
    pub fn changed(self) -> bool {
        matches!(self, Self::Changed)
    }
}

impl PuzzleBuilder {
    pub fn new_symbol_set(&mut self, name: &str) -> SymbolSetBuilder {
        SymbolSetBuilder::new(self, name)
    }

    pub(crate) fn push_symbol_set(&mut self, set: RawSymbolSet) -> SymbolSetId {
        self.symbols.push(set);
        SymbolSetId(self.symbols.len() - 1)
    }

    pub fn new_cell(&mut self, cell: CellInfo) -> CellIndex {
        assert!(
            !self.rowcols.contains_key(&(cell.row, cell.col)),
            "Attempted to insert duplicate row/col"
        );
        let ret = CellIndex(self.cells.len());
        self.rowcols.insert((cell.row, cell.col), ret);
        self.cells.push(cell);
        self.cell_regions.push(vec![]);
        ret
    }

    pub fn add_constraint<C: Constraint + 'static>(&mut self, constraint: C) {
        self.constraints.push(Box::new(constraint));
    }

    pub fn add_technique<T: Technique + 'static>(&mut self, technique: T) {
        self.techniques.push(Box::new(technique))
    }

    pub fn add_region(&mut self, region: Region) -> RegionId {
        assert!(
            !self.regions.iter().any(|r| r.name == region.name),
            "Attempted to insert duplicate for region {}",
            region.name
        );
        let ret = RegionId(self.regions.len());
        for CellIndex(cell) in region.cells.iter().copied() {
            self.cell_regions[cell].push(ret);
        }
        self.regions.push(region);
        ret
    }

    pub fn symbols(&self, set: SymbolSetId) -> impl Iterator<Item = SymbolId> {
        self.symbols[set.0].to_ids(set.0)
    }

    pub fn region(&self, region: RegionId) -> &Region {
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

    pub fn build(self) -> Puzzle {
        let Self {
            symbols,
            constraints,
            techniques,
            cells,
            cell_regions,
            regions,
            rowcols,
            implications,
        } = self;

        Puzzle {
            symbols,
            constraints,
            techniques,
            cells,
            cell_regions,
            regions,
            implications,
            rowcols,
        }
    }

    pub fn cell_info(&self, cell: CellIndex) -> &CellInfo {
        &self.cells[cell.0]
    }

    pub fn cell_at(&self, row: usize, col: usize) -> Option<CellIndex> {
        self.rowcols.get(&(row, col)).copied()
    }
}

impl Puzzle {
    pub fn regions(&self) -> impl Iterator<Item = RegionId> {
        (0..self.regions.len()).map(RegionId)
    }

    pub fn region(&self, region: RegionId) -> &Region {
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

    pub fn sees(&self, cell: CellIndex, symbol: SymbolId) -> impl Iterator<Item = CellIndex> + '_ {
        self.implications
            .get(&(cell, symbol))
            .map(move |v| {
                v.iter()
                    .filter_map(move |e| if e.1 == symbol { Some(e.0) } else { None })
            })
            .into_iter()
            .flatten()
    }

    fn initial_board(&self) -> SolveState {
        let mut state = SolveState::new(self, Board::empty(self.cells.len(), &self.symbols));
        for constraint in &self.constraints {
            constraint.prep_board(&mut state);
        }
        state
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

    pub fn solution(&self, board: &Board) -> String {
        assert!(board.solved());
        assert_eq!(self.cells.len() * self.symbols.len(), board.cells.len());

        let mut ret = String::new();
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

        for row in minrow..=maxrow {
            for col in mincol..=maxcol {
                if let Some(cell) = self.rowcols.get(&(row, col)).copied() {
                    for (set, choice) in board.choices(cell).enumerate() {
                        if let Some(value) = choice.single_value() {
                            let symbol = &self.symbols[set][value];
                            ret.push_str(&format!("{symbol}"));
                        }
                    }
                }
            }
        }

        ret
    }

    pub fn add_constraint<C: Constraint + 'static>(&mut self, constraint: C) {
        self.constraints.push(Box::new(constraint));
    }

    fn symbol_set(&self, idx: SymbolSetId) -> &RawSymbolSet {
        &self.symbols[idx.0]
    }

    pub fn cell_at(&self, row: usize, col: usize) -> Option<CellIndex> {
        self.rowcols.get(&(row, col)).copied()
    }

    pub fn symbols(&self, set: SymbolSetId) -> impl Iterator<Item = SymbolId> {
        self.symbols[set.0].to_ids(set.0)
    }

    fn symbol_set_idx_by_name(&self, set: &str) -> usize {
        for (idx, rawset) in self.symbols.iter().enumerate() {
            if rawset.name() == set {
                return idx;
            }
        }
        panic!("Unable to find {set}");
    }

    pub fn symbols_by_set_name(&self, set: &str) -> impl Iterator<Item = SymbolId> {
        let set = self.symbol_set_idx_by_name(set);
        self.symbols[set].to_ids(set)
    }

    fn set_symbol(&self, board: &mut Board, cell: CellIndex, symbol: SymbolId) {
        let (_set, symbolnr) = symbol.into_parts();
        let choice = board.choice_mut(cell, symbol);
        assert!(choice.can_be(symbolnr));
        choice.set(symbolnr);
        self.propagate_changes(board, cell);
    }

    fn eliminate(&self, board: &mut Board, cell: CellIndex, symbol: SymbolId) -> Effect {
        let (_set, symbolnr) = symbol.into_parts();
        let choice = board.choice_mut(cell, symbol);
        let choicecopy = *choice;
        choice.unset(symbolnr);
        if *choice != choicecopy {
            self.propagate_changes(board, cell);
            Effect::Changed
        } else {
            Effect::Unchanged
        }
    }

    fn restrict(&self, board: &mut Board, cell: CellIndex, symbols: SymbolChoice) -> Effect {
        let choice = board.choice_mut_by_set(cell, symbols.set);
        let choicecopy = *choice;
        *choice &= symbols.choice;
        if *choice != choicecopy {
            self.propagate_changes(board, cell);
            Effect::Changed
        } else {
            Effect::Unchanged
        }
    }

    fn propagate_changes(&self, board: &mut Board, cell: CellIndex) {
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

    pub fn symbol_sets(&self) -> impl Iterator<Item = SymbolSetId> {
        (0..self.symbols.len()).map(SymbolSetId)
    }

    pub fn cell_info(&self, cell: CellIndex) -> &CellInfo {
        &self.cells[cell.0]
    }

    fn logical_step(&self, board: &mut SolveState) -> LogicalStep {
        let mut finished = true;
        for technique in &self.techniques {
            match technique.logical_step(board) {
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
        board.into_board()
    }

    fn can_see(
        &self,
        cell1: CellIndex,
        symbol1: SymbolId,
        cell2: CellIndex,
        symbol2: SymbolId,
    ) -> bool {
        if let Some(pairs) = self.implications.get(&(cell1, symbol1)) {
            pairs.contains(&(cell2, symbol2))
        } else {
            false
        }
    }
}

impl Region {
    pub fn new(name: impl Into<String>, cells: impl IntoIterator<Item = CellIndex>) -> Self {
        Self {
            name: name.into(),
            cells: cells.into_iter().collect(),
        }
    }

    pub fn to_cells(&self) -> Vec<CellIndex> {
        self.cells.clone()
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.cells.len()
    }
}

impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)
    }
}

impl Board {
    fn empty(ncells: usize, symbols: &[RawSymbolSet]) -> Self {
        let mut cells = Vec::with_capacity(ncells * symbols.len());
        for _cell in 0..ncells {
            for symbols in symbols {
                cells.push(RawSymbolChoice::any(symbols.len()));
            }
        }
        Self {
            nsymbols: symbols.len(),
            cells,
        }
    }

    pub fn solved(&self) -> bool {
        self.cells.iter().all(|c| c.solved())
    }

    fn choices(&self, idx: CellIndex) -> impl Iterator<Item = RawSymbolChoice> + use<'_> {
        let idx = idx.0 * self.nsymbols;
        assert!(idx + self.nsymbols <= self.cells.len());
        self.cells.iter().skip(idx).take(self.nsymbols).copied()
    }

    fn choice_mut(&mut self, idx: CellIndex, symbol: SymbolId) -> &mut RawSymbolChoice {
        let idx = idx.0 * self.nsymbols + symbol.into_parts().0;
        &mut self.cells[idx]
    }

    fn choice_mut_by_set(&mut self, idx: CellIndex, set: SymbolSetId) -> &mut RawSymbolChoice {
        let idx = idx.0 * self.nsymbols + set.0;
        &mut self.cells[idx]
    }

    fn choice_set(&self, idx: CellIndex, set: usize) -> RawSymbolChoice {
        let idx = idx.0 * self.nsymbols + set;
        self.cells[idx]
    }
}

impl CellInfo {
    pub fn new(name: impl Into<String>, row: usize, col: usize) -> Self {
        Self {
            name: name.into(),
            row,
            col,
        }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }
}

impl Display for CellInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)
    }
}

impl<'p> SolveState<'p> {
    pub fn new(puzzle: &'p Puzzle, board: Board) -> Self {
        Self { puzzle, board }
    }

    pub fn solved(&self) -> bool {
        self.board.solved()
    }

    pub fn into_board(self) -> Board {
        self.board
    }

    pub fn symbols_by_set_name(&self, set: &str) -> impl Iterator<Item = SymbolId> {
        self.puzzle.symbols_by_set_name(set)
    }

    pub fn cell_at(&self, row: usize, col: usize) -> Option<CellIndex> {
        self.puzzle.cell_at(row, col)
    }

    pub fn set_symbol(&mut self, cell: CellIndex, symbol: SymbolId) {
        self.puzzle.set_symbol(&mut self.board, cell, symbol);
    }

    pub fn eliminate(&mut self, cell: CellIndex, symbol: SymbolId) -> Effect {
        self.puzzle.eliminate(&mut self.board, cell, symbol)
    }

    pub fn restrict(&mut self, cell: CellIndex, symbols: SymbolChoice) -> Effect {
        self.puzzle.restrict(&mut self.board, cell, symbols)
    }

    pub fn all_cells(&self) -> impl Iterator<Item = CellIndex> {
        self.puzzle.all_cells()
    }

    pub fn symbol_sets(&self) -> impl Iterator<Item = SymbolSetId> {
        self.puzzle.symbol_sets()
    }

    pub fn choices(&self, cell: CellIndex, set: SymbolSetId) -> SymbolChoice {
        let raw_choice = self
            .board
            .choices(cell)
            .nth(set.0)
            .expect("For some reason, a choice was missing");
        SymbolChoice::new(set, raw_choice)
    }

    pub fn cell_info(&self, cell: CellIndex) -> &CellInfo {
        self.puzzle.cell_info(cell)
    }

    pub fn symbol(&self, value: SymbolId) -> &Symbol {
        let (set, idx) = value.into_parts();
        &self.puzzle.symbols[set][idx]
    }

    pub fn regions(&self) -> impl Iterator<Item = RegionId> {
        self.puzzle.regions()
    }

    pub fn region(&self, region: RegionId) -> &Region {
        self.puzzle.region(region)
    }

    pub fn symbols(&self, set: SymbolSetId) -> impl Iterator<Item = SymbolId> {
        self.puzzle.symbol_set(set).to_ids(set.0)
    }

    pub fn regions_for_cell(&self, cell: CellIndex) -> impl Iterator<Item = RegionId> {
        self.puzzle.cell_regions[cell.0].clone().into_iter()
    }

    pub fn sees(&self, cell: CellIndex, symbol: SymbolId) -> impl Iterator<Item = CellIndex> + 'p {
        self.puzzle.sees(cell, symbol)
    }

    pub fn can_see(
        &self,
        cell1: CellIndex,
        symbol1: SymbolId,
        cell2: CellIndex,
        symbol2: SymbolId,
    ) -> bool {
        self.puzzle.can_see(cell1, symbol1, cell2, symbol2)
    }
}

impl SymbolChoice {
    pub(crate) fn new(set: SymbolSetId, choice: RawSymbolChoice) -> Self {
        Self { set, choice }
    }

    pub fn solved(&self) -> bool {
        self.choice.solved()
    }

    pub fn single_value(&self) -> Option<SymbolId> {
        self.choice
            .single_value()
            .map(|symbol| SymbolId::new(self.set.0, symbol))
    }

    pub fn options(&self) -> impl Iterator<Item = SymbolId> {
        let set = self.set.0;
        self.choice.options().map(move |v| SymbolId::new(set, v))
    }

    pub fn can_be(&self, symbol: SymbolId) -> bool {
        assert_eq!(self.set.0, symbol.set_index());
        self.choice.can_be(symbol.symbol_index())
    }
}

impl BitOr for SymbolChoice {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        assert_eq!(self.set, rhs.set);
        let choice = self.choice | rhs.choice;
        Self {
            set: self.set,
            choice,
        }
    }
}

impl BitAnd for SymbolChoice {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        assert_eq!(self.set, rhs.set);
        let choice = self.choice & rhs.choice;
        Self {
            set: self.set,
            choice,
        }
    }
}
