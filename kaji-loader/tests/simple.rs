use std::path::PathBuf;

use kaji_loader::raw::*;

fn load_sample(leaf: &str) -> String {
    let ctoml = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fpath = ctoml.join(format!("../sample-puzzles/{leaf}"));
    std::fs::read_to_string(fpath).expect("Unable to read file")
}

#[test]
fn load_puzzle1() {
    let puzzle1 = load_sample("puzzle1.json");
    let _data = RawPuzzleData::load(&puzzle1).unwrap();
}
