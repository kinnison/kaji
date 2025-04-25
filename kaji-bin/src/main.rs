use kaji::{PuzzleBuilder, Rule};
use kaji_rules::{constraints::GivenDigits, rules::sudoku::SudokuGrid};

fn main() {
    let mut builder = PuzzleBuilder::default();

    SudokuGrid::new(9).apply(&mut builder);

    let digits = GivenDigits::from_pattern(&[
        "3.6.82...",
        "..8...4..",
        "...13.7..",
        ".1.9..5..",
        "....7..8.",
        ".3...1..7",
        "4....59..",
        ".........",
        "5.7.1.8..",
    ]);

    builder.add_constraint(digits);

    // Remove this constraint to need finned xwing
    builder.add_constraint(GivenDigits::new(&[(2, 5, 9)]));

    let puzzle = builder.build();

    let board = puzzle.solve();
    puzzle.print_board(&board);
}
