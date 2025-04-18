use std::{
    fmt::Display,
    ops::{Deref, Index},
};

/// A puzzle cell symbol
///
/// Cells can ultimately have only one symbol present in them
/// per [`SymbolSet`] associated with the [`Puzzle`]
#[derive(Debug)]
pub struct Symbol {
    name: String,
}

/// A set of symbols in a sudoku puzzle
///
/// A collection of symbols, of which exactly one can be present in
/// a board's cells.
///
/// For example, sudoku has nine symbols (the digits 1-9), but
/// a sudoku yin/yang hybrid puzzle has two sets, the sudoku digits,
/// and a set of dark/light.
///
/// For ease of implementation, symbol sets will be limited to
/// 31 symbols per set.  This allows the available symbols to be
/// represented as a u32, leaving one bit available to say the cell
/// is solved.
#[derive(Debug)]
pub struct SymbolSet {
    name: String,
    symbols: Vec<Symbol>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// A Symbol Id encodes the set and symbol
///
/// Internally this is represented as the bottom five bits being the
/// symbol index, and the rest being the symbol set.
#[repr(transparent)]
pub struct SymbolId(usize);

#[derive(Clone, Copy)]
/// A SymbolChoice is a u32 which represents which symbols from a symbolset
/// a given cell could have in a given board.
#[repr(transparent)]
#[derive(Debug)]
pub struct SymbolChoice(u32);

impl Symbol {
    pub(crate) fn width(&self) -> usize {
        self.name.len()
    }
}

impl SymbolSet {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            symbols: vec![],
        }
    }

    pub fn push(&mut self, symbol: impl Into<Symbol>) {
        self.symbols.push(symbol.into());
    }

    pub fn to_ids(&self, set: usize) -> impl Iterator<Item = SymbolId> {
        (0..self.symbols.len()).map(move |s| SymbolId::new(set, s))
    }

    pub fn id(&self, set: usize, symbol: usize) -> SymbolId {
        assert!(symbol < self.symbols.len());
        SymbolId::new(set, symbol)
    }

    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    pub fn width(&self) -> usize {
        self.symbols.iter().map(Symbol::width).max().unwrap_or(0)
    }
}

impl Index<usize> for SymbolSet {
    type Output = Symbol;

    fn index(&self, index: usize) -> &Self::Output {
        &self.symbols[index]
    }
}

impl Symbol {
    pub fn new(display: impl Into<String>) -> Self {
        Self {
            name: display.into(),
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)
    }
}

impl From<String> for Symbol {
    fn from(val: String) -> Self {
        Symbol::new(val)
    }
}

impl From<&str> for Symbol {
    fn from(val: &str) -> Self {
        Symbol::new(val)
    }
}

impl SymbolId {
    fn new(set: usize, symbol: usize) -> Self {
        Self((set << 5) | (symbol & 31))
    }

    pub(crate) fn into_parts(self) -> (usize, usize) {
        (self.0 >> 5, self.0 & 31)
    }
}

impl SymbolChoice {
    const VALUE_MASK: u32 = (1 << 31) - 1;
    const SOLVED_MASK: u32 = (1 << 31);

    pub(crate) fn any(size: usize) -> Self {
        assert!(size < 32);
        Self((1 << size) - 1)
    }

    pub fn solved(&self) -> bool {
        (self.0 & Self::SOLVED_MASK) != 0
    }

    pub fn single_value(&self) -> Option<usize> {
        let values = self.0 & Self::VALUE_MASK;
        if values.count_ones() == 1 {
            Some(values.ilog2() as usize)
        } else {
            None
        }
    }

    pub fn can_be(&self, symbol: usize) -> bool {
        (self.0 & (1 << symbol)) != 0
    }

    pub fn set(&mut self, symbol: usize) {
        self.0 = Self::SOLVED_MASK | (1 << symbol);
    }

    pub fn unset(&mut self, symbol: usize) {
        self.0 &= !(1 << symbol)
    }

    pub fn options(&self) -> OptionIter {
        OptionIter {
            mask: self.0 & Self::VALUE_MASK,
        }
    }
}

pub struct OptionIter {
    mask: u32,
}

impl Iterator for OptionIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.mask.leading_zeros() {
            32 => None,
            n => {
                let idx = (31 - n) as usize;
                self.mask &= !(1 << idx);
                Some(idx)
            }
        }
    }
}
