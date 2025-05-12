use std::path::PathBuf;

use kaji::PuzzleBuilder;
use kaji_loader::raw::RawPuzzleData;
use kaji_rules::puzzledata::{GridDataKind, PuzzleData};

fn load_sample(leaf: &str) -> String {
    let ctoml = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fpath = ctoml.join(format!("../sample-puzzles/{leaf}"));
    std::fs::read_to_string(fpath).expect("Unable to read file")
}

fn main() {
    let mut builder = PuzzleBuilder::default();

    let raw = RawPuzzleData::load(load_sample("antiknight1.json")).unwrap();

    let puzzledata = PuzzleData::from(raw);

    println!("{puzzledata:#?}");

    puzzledata.build(&mut builder);

    let puzzle = builder.build();

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
