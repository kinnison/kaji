use kaji_bin::*;
use kaji_rules::puzzledata::GridDataKind;
use rstest::rstest;

#[rstest]
#[case::odd_even("strange-level")]
#[case::palindrome("back-and-forth")]
#[case::clone_cells("clone-wars")]
#[case::disjoint_groups("joined-up-thinking")]
#[case::irregular_regions("piggly-wiggly")]
#[case::quadruple("quad-bike")]
#[case::diagonals("wormhole-sixxtreme")]
fn fpuzzle_works(#[case] puzzle: &str) {
    let (puzzledata, puzzle) = load_fpuzzles_puzzle(puzzle);

    let board = puzzle.solve();
    puzzle.print_board(&board);

    assert!(board.solved());
    let solution = puzzle.solution(&board);
    #[allow(clippy::infallible_destructuring_match)]
    let raw_board = match puzzledata.grids()[0].kind() {
        GridDataKind::Sudoku(grid) => grid,
    };
    if let Some(real_solution) = raw_board.solution_() {
        assert_eq!(real_solution, solution);
    } else {
        panic!("No solution in puzzle input")
    }
}

#[rstest]
#[case::antiknight("antiknight1")]
#[case::unknown("puzzle1")]
// TODO: Determine what this puzzle was and what we have broken?
//#[case::unknown("puzzle2")]
#[case::swordfish("swordfish1")]
#[case::xwing("xwing1")]
#[case::xwing("xwing2")]
#[case::xwing("xwing3")]
fn sudokumaker_puzzle_works(#[case] puzzle: &str) {
    let (puzzledata, puzzle) = load_sudokumaker_puzzle(puzzle);

    let board = puzzle.solve();
    puzzle.print_board(&board);

    assert!(board.solved());
    let solution = puzzle.solution(&board);
    #[allow(clippy::infallible_destructuring_match)]
    let raw_board = match puzzledata.grids()[0].kind() {
        GridDataKind::Sudoku(grid) => grid,
    };
    if let Some(real_solution) = raw_board.solution_() {
        assert_eq!(real_solution, solution);
    } else {
        panic!("No solution in puzzle input")
    }
}
