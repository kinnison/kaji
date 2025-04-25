use kaji::{PuzzleBuilder, Rule};
use kaji_rules::rules::sudoku::SudokuGrid;

fn main() {
    let mut builder = PuzzleBuilder::default();

    SudokuGrid::new(9).apply(&mut builder);

    let digits = kaji_rules::constraints::GivenDigits::from_pattern(&[
        "3.....4.1",
        ".85.7.2.3",
        "....1...8",
        "..4....2.",
        ".59......",
        "...9...64",
        "6....2...",
        "..35.....",
        "....67...",
    ]);

    builder.add_constraint(digits);

    let puzzle = builder.build();

    let board = puzzle.solve();
    puzzle.print_board(&board);
}
