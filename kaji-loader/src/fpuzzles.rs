//! Raw fpuzzles data
//!

use std::num::NonZeroUsize;

use serde::Deserialize;
use serde_json::Result;

mod convert;

#[derive(Debug, Deserialize)]
pub struct FpuzzlesData {
    pub size: usize,
    pub title: String,
    pub author: String,
    pub ruleset: String,
    pub solution: Option<Vec<NonZeroUsize>>,
    pub grid: Vec<Vec<FpuzzlesCellData>>,

    // Rules
    #[serde(default)]
    pub antiking: bool,
    #[serde(default)]
    pub antiknight: bool,
}

#[derive(Debug, Deserialize)]
pub struct FpuzzlesCellData {
    pub value: Option<NonZeroUsize>,
    pub region: Option<usize>,
}

impl FpuzzlesData {
    pub fn load(s: impl AsRef<str>) -> Result<Self> {
        let s = s.as_ref();
        serde_json::from_str(s)
    }
}
