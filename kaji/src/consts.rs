//! Constants for Kaji
//!

/// The digits (numerical symbols) found in a puzzle.
///
/// These are likely to be 1 through 9, or similar, but
/// could equally well be -4 through 4, 0 to 9, etc.
pub const SYMBOL_SET_DIGITS: &str = "digits";

/// The difficulty rating for very simple techniques
/// such as naked or hidden singles
pub const DIFFICULTY_TRIVIAL: u16 = 1000;

/// The difficulty rating for slightly complex techniques
/// such as small tuples or small pointing sets
pub const DIFFICULTY_EASY: u16 = 2000;

/// The difficulty rating for medium complexity techniques
/// such as quadruples or similarly small constraints
pub const DIFFICULTY_MEDIUM: u16 = 5000;

/// The difficulty rating for harder techniques such
/// as small fish, some kinds of lines, etc.
pub const DIFFICULTY_HARD: u16 = 8000;

/// The difficulty rating for fiendish things such as
/// larger fish.
pub const DIFFICULTY_FIENDISH: u16 = 10_000;
