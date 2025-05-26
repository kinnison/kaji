use kaji_rules::puzzledata::GridDataKind;

use kaji_bin::load_fpuzzles_puzzle;

fn main() {
    let (puzzledata, puzzle) = load_fpuzzles_puzzle("german-whisper-test-1");

    let board = puzzle.solve();
    puzzle.print_board(&board);

    if board.solved() {
        let solution = puzzle.solution(&board);
        #[allow(clippy::infallible_destructuring_match)]
        let raw_board = match puzzledata.grids()[0].kind() {
            GridDataKind::Sudoku(grid) => grid,
        };
        if let Some(real_solution) = raw_board.solution_() {
            if real_solution == solution {
                println!("Solutions match");
            } else {
                println!("Solutions do not match!");
            }
        }
    } else {
        println!("Could not solve!");
    }
}
