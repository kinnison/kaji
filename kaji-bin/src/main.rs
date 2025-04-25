use kaji::{PuzzleBuilder, Rule};
use kaji_rules::rules::sudoku::SudokuGrid;

fn main() {
    let mut builder = PuzzleBuilder::default();

    SudokuGrid::new(6).apply(&mut builder);

    let digits = kaji_rules::constraints::GivenDigits::new(&[
        (1, 2, 5),
        (1, 6, 2),
        (2, 5, 4),
        (3, 3, 1),
        (3, 4, 2),
        (4, 3, 5),
        (4, 4, 3),
        (5, 2, 6),
        (6, 1, 4),
        (6, 5, 3),
    ]);

    builder.add_constraint(digits);

    let puzzle = builder.build();

    let board = puzzle.solve();
    puzzle.print_board(&board);
}
