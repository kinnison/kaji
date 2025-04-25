use kaji::{PuzzleBuilder, Rule};
use kaji_rules::rules::sudoku::SudokuGrid;

fn main() {
    let mut builder = PuzzleBuilder::default();

    SudokuGrid::new(9).apply(&mut builder);

    let digits = kaji_rules::constraints::GivenDigits::from_pattern(&[
        "71..8....",
        ".....6..4",
        ".3......1",
        "9...671..",
        "....5....",
        "8.29....3",
        "....1.2.5",
        ".....387.",
        "..68.....",
    ]);

    builder.add_constraint(digits);

    let puzzle = builder.build();

    let board = puzzle.solve();
    puzzle.print_board(&board);
}
