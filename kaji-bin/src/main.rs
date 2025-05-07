use std::path::PathBuf;

use kaji::{PuzzleBuilder, Rule};
use kaji_loader::raw::RawPuzzleData;
use kaji_rules::{constraints::GivenDigits, rules::sudoku::SudokuGrid};

fn load_sample(leaf: &str) -> String {
    let ctoml = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fpath = ctoml.join(format!("../sample-puzzles/{leaf}"));
    std::fs::read_to_string(fpath).expect("Unable to read file")
}

fn main() {
    let mut builder = PuzzleBuilder::default();

    let raw = RawPuzzleData::load(load_sample("swordfish1.json")).unwrap();

    SudokuGrid::new(&raw).apply(&mut builder);
    builder.add_constraint(GivenDigits::new(&raw));

    let puzzle = builder.build();

    let board = puzzle.solve();
    puzzle.print_board(&board);

    if board.solved() {
        let solution = puzzle.solution(&board);
        if let Some(real_solution) = &raw.metadata.solution {
            if real_solution == &solution {
                println!("Solutions match");
            } else {
                println!("Solutions do not match!");
            }
        }
    } else {
        println!("Could not solve!");
    }
}
