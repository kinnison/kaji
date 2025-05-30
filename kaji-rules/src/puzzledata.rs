//! Data representing a puzzle
//!

use std::num::NonZeroUsize;

use kaji::{PuzzleBuilder, Rule, Symbol, SymbolValue};

use crate::{
    constraints::SudokuIndexerKind,
    rules::{cellpairs::CellPairRelationship, sudoku::SudokuGrid},
};

#[derive(Debug, Default)]
pub struct PuzzleData {
    symbols: Vec<SymbolSetData>,
    grids: Vec<GridData>,
}

#[derive(Debug)]
pub struct SymbolSetData {
    name: String,
    symbols: Vec<SymbolData>,
}

#[derive(Debug)]
pub struct SymbolData {
    display: String,
    value: SymbolValue,
}

#[derive(Debug)]
pub struct GridData {
    row: usize,
    col: usize,
    kind: GridDataKind,
}

#[derive(Debug)]
pub enum GridDataKind {
    Sudoku(SudokuGridData),
}

#[derive(Debug)]
pub struct SudokuGridData {
    size: usize,
    symbols: String,
    regions: Vec<usize>,
    givens: Vec<(usize, usize, usize)>,
    solution: Option<String>,
    rules: SudokuGridRulesData,
}

#[derive(Debug, Default)]
pub struct SudokuGridRulesData {
    pub antiknight: bool,
    pub antiking: bool,
    pub quadruple: Vec<SudokuGridRuleQuadrupleData>,
    pub diagonal_p: bool,
    pub diagonal_n: bool,
    pub disjoint_groups: bool,
    pub clone_pairs: Vec<SudokuGridRuleCloneData>,
    /// (row,col) 1 based
    pub odd_cells: Vec<(usize, usize)>,
    /// (row,col) 1 based    
    pub even_cells: Vec<(usize, usize)>,
    pub pair_relationships: SudokuGridRulePairRelationsData,
    /// (row,col) 1 based
    pub maximum: Vec<(usize, usize)>,
    /// (row,col) 1 based
    pub minimum: Vec<(usize, usize)>,
    /// [(row,col)] 1 based
    pub thermometer: Vec<Vec<(usize, usize)>>,
    /// [diff,[(row,col)]]
    pub whispers: Vec<(i32, Vec<(usize, usize)>)>,
    /// Indexers
    pub indexers: Vec<(usize, usize, SudokuIndexerKind)>,
}

#[derive(Debug)]
pub struct SudokuGridRuleQuadrupleData {
    /// (row,col) 1 based
    pub cells: Vec<(usize, usize)>,
    /// (symbol index, 1 based)
    pub symbols: Vec<usize>,
}

#[derive(Debug)]
pub struct SudokuGridRuleCloneData {
    /// (row,col) 1-based
    pub a: (usize, usize),
    /// (row,col) 1-based
    pub b: (usize, usize),
}

#[derive(Debug, Default)]
pub struct SudokuGridRulePairRelationsData {
    // Negative constraints
    pub nonconsecutive: bool,
    pub anti_black_dot: bool,
    pub anti_x: bool,
    pub anti_v: bool,
    // Cells which have an explicit relationship
    pub relationships: Vec<RawSudokuPairRelationship>,
}

#[derive(Debug)]
pub struct RawSudokuPairRelationship {
    pub name: String,
    /// 1-based (row,col)
    pub cell_a: (usize, usize),
    /// 1-based (row,col)
    pub cell_b: (usize, usize),
    // Relationship
    pub relationship: CellPairRelationship,
}

impl PuzzleData {
    pub fn symbols(&self) -> &[SymbolSetData] {
        &self.symbols
    }

    pub fn symbols_by_name(&self, name: &str) -> Option<&SymbolSetData> {
        self.symbols.iter().find(|s| s.name() == name)
    }

    pub fn push_symbols(&mut self, symbol_set: SymbolSetData) {
        assert!(
            self.symbols_by_name(symbol_set.name()).is_none(),
            "Duplicate symbol set found"
        );
        self.symbols.push(symbol_set);
    }

    pub fn grids(&self) -> &[GridData] {
        &self.grids
    }

    pub fn push_grid(&mut self, grid: GridData) {
        grid.kind().check_for_symbols(self);
        self.grids.push(grid);
    }

    pub fn build(&self, builder: &mut PuzzleBuilder) {
        for set in self.symbols() {
            let mut builder = builder.new_symbol_set(set.name());
            for symbol in set.symbols() {
                let symbol = Symbol::new(symbol.display(), symbol.value());
                builder.push(symbol);
            }
            builder.finish();
        }
        for grid in self.grids() {
            grid.build(builder);
        }
    }
}

impl SymbolSetData {
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            name,
            symbols: vec![],
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn symbols(&self) -> &[SymbolData] {
        &self.symbols
    }

    pub fn push(&mut self, symbol: SymbolData) {
        self.symbols.push(symbol);
    }
}

impl SymbolData {
    pub fn new(display: impl Into<String>, value: SymbolValue) -> Self {
        let display = display.into();
        Self { display, value }
    }

    pub fn display(&self) -> &str {
        &self.display
    }

    pub fn value(&self) -> SymbolValue {
        self.value
    }
}

impl GridData {
    pub fn new(row: usize, col: usize, kind: GridDataKind) -> Self {
        Self { row, col, kind }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn kind(&self) -> &GridDataKind {
        &self.kind
    }

    pub fn build(&self, builder: &mut PuzzleBuilder) {
        match self.kind() {
            GridDataKind::Sudoku(grid) => grid.build(builder, self.row, self.col),
        }
    }
}

impl GridDataKind {
    fn check_for_symbols(&self, symbols: &PuzzleData) {
        match self {
            GridDataKind::Sudoku(grid) => grid.check_for_symbols(symbols),
        }
    }
}

impl SudokuGridData {
    /// Create a new sudoku grid
    ///
    /// ```
    /// # use kaji_rules::puzzledata::SudokuGridData;
    /// # use std::num::NonZeroUsize;
    ///
    /// let grid = SudokuGridData::new(
    ///     "digits",
    ///     NonZeroUsize::new(9).unwrap(),
    ///     SudokuGridData::default_regions(9),
    /// );
    /// assert_eq!(grid.symbols(), "digits");
    /// assert_eq!(grid.size(), 9);
    /// assert_eq!(grid.regions()[3], 2);
    /// ```
    pub fn new(symbols: impl Into<String>, size: NonZeroUsize, regions: Vec<usize>) -> Self {
        let symbols = symbols.into();
        let size = size.get();
        assert_eq!(regions.len(), size * size);
        assert!(regions.iter().all(|cell| (1..=size).contains(cell)));
        for rnum in 1..=size {
            let rcount = regions.iter().filter(|&&cell| cell == rnum).count();
            assert_eq!(rcount, size)
        }
        Self {
            symbols,
            size,
            regions,
            solution: None,
            givens: Default::default(),
            rules: Default::default(),
        }
    }

    /// The symbols for the grid
    pub fn symbols(&self) -> &str {
        &self.symbols
    }

    /// The size of the grid (eg. 9 for a normal sudoku)
    pub fn size(&self) -> usize {
        self.size
    }

    /// The regions for the cells as a linear vector
    pub fn regions(&self) -> &[usize] {
        &self.regions
    }

    /// The given digits for the grid
    ///
    /// This is in the form of tuples of (row, col, digit)
    /// where row and col are one-indexed relative to this grid
    /// and digit is one-indexed (ie 1 is the first symbol)
    pub fn givens(&self) -> &[(usize, usize, usize)] {
        &self.givens
    }

    /// Add a given digit to the grid
    pub fn push_given(&mut self, row: usize, col: usize, digit: usize) {
        self.givens.push((row, col, digit));
    }
    /// The rules for this grid
    pub fn rules(&self) -> &SudokuGridRulesData {
        &self.rules
    }

    /// The rules for this grid, mutably
    pub fn rules_mut(&mut self) -> &mut SudokuGridRulesData {
        &mut self.rules
    }

    fn build(&self, builder: &mut PuzzleBuilder, rofs: usize, cofs: usize) {
        let digits = builder
            .symbol_set(self.symbols())
            .expect("Somehow the digits weren't registered!");
        SudokuGrid::new(digits, rofs, cofs, self).apply(builder);
    }

    pub fn set_solution_(&mut self, solution: &str) {
        self.solution = Some(solution.into());
    }

    pub fn solution_(&self) -> Option<&str> {
        self.solution.as_deref()
    }

    /// Compute the default regions for the given grid size
    ///
    /// ```
    /// # use kaji_rules::puzzledata::SudokuGridData;
    /// assert_eq!(SudokuGridData::default_regions(4),
    ///     vec![
    ///         1, 1, 2, 2,
    ///         1, 1, 2, 2,
    ///         3, 3, 4, 4,
    ///         3, 3, 4, 4,
    /// ]);
    /// assert_eq!(SudokuGridData::default_regions(5),
    ///     vec![
    ///         1, 1, 1, 1, 1,
    ///         2, 2, 2, 2, 2,
    ///         3, 3, 3, 3, 3,
    ///         4, 4, 4, 4, 4,
    ///         5, 5, 5, 5, 5,
    /// ]);
    /// assert_eq!(SudokuGridData::default_regions(6),
    ///     vec![
    ///         1, 1, 1, 2, 2, 2,
    ///         1, 1, 1, 2, 2, 2,
    ///         3, 3, 3, 4, 4, 4,
    ///         3, 3, 3, 4, 4, 4,
    ///         5, 5, 5, 6, 6, 6,
    ///         5, 5, 5, 6, 6, 6,
    /// ]);
    /// ```
    pub fn default_regions(size: usize) -> Vec<usize> {
        let mut ret = Vec::with_capacity(size * size);
        let mut region_height = size.isqrt();
        while (size % region_height) != 0 {
            region_height -= 1;
        }
        let region_width = size / region_height;
        assert_eq!(region_height * region_width, size);
        for row in 0..size {
            let n = (row / region_height) * region_height;
            for col in 0..size {
                let m = col / region_width;
                let z = n + m + 1;
                ret.push(z);
            }
        }
        ret
    }

    fn check_for_symbols(&self, symbols: &PuzzleData) {
        let Some(symbols) = symbols.symbols_by_name(self.symbols()) else {
            panic!("Unable to find symbols: {}", self.symbols);
        };
        assert!(symbols.symbols().len() >= self.size);
    }
}
