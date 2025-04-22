use kaji::Puzzle;

fn main() {
    let mut puzzle = Puzzle::new_sudoku(6);

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

    puzzle.add_constraint(digits);
    puzzle.add_constraint(kaji_rules::constraints::NakedSingle);
    puzzle.add_constraint(kaji_rules::constraints::HiddenSingle);

    let board = puzzle.solve();
    puzzle.print_board(&board);
}
