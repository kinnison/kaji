use kaji::{PuzzleBuilder, Rule};
use kaji_rules::rules::sudoku::SudokuGrid;

fn main() {
    let mut builder = PuzzleBuilder::default();

    SudokuGrid::new(9).apply(&mut builder);

    let digits = kaji_rules::constraints::GivenDigits::from_pattern(&[
        "4.9..6..5",
        ".8275...3",
        "..14.396.",
        "948...21.",
        "....6..5.",
        "..32.1.79",
        "3.5..7428",
        "12.8.4...",
        ".7.59.6..",
    ]);

    builder.add_constraint(digits);

    let puzzle = builder.build();

    let board = puzzle.solve();
    puzzle.print_board(&board);
}
