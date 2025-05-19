use std::path::PathBuf;

use kaji::{Puzzle, PuzzleBuilder};
use kaji_loader::{fpuzzles::FpuzzlesData, sudokumaker::RawSudokuMakerData};
use kaji_rules::puzzledata::PuzzleData;

fn load_sample(leaf: &str) -> String {
    let ctoml = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fpath = ctoml.join(format!("../sample-puzzles/{leaf}"));
    std::fs::read_to_string(fpath).expect("Unable to read file")
}

pub fn load_fpuzzles_puzzle(puzzle_name: &str) -> (PuzzleData, Puzzle) {
    let mut builder = PuzzleBuilder::default();

    let raw = FpuzzlesData::load(load_sample(&format!("{puzzle_name}.json"))).unwrap();

    let puzzledata = PuzzleData::from(raw);

    puzzledata.build(&mut builder);

    let puzzle = builder.build();
    (puzzledata, puzzle)
}

pub fn load_sudokumaker_puzzle(puzzle_name: &str) -> (PuzzleData, Puzzle) {
    let mut builder = PuzzleBuilder::default();

    let raw = RawSudokuMakerData::load(load_sample(&format!("{puzzle_name}.json"))).unwrap();

    let puzzledata = PuzzleData::from(raw);

    puzzledata.build(&mut builder);

    let puzzle = builder.build();
    (puzzledata, puzzle)
}
