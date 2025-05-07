use serde::Deserialize;
use serde_json::Result;

#[derive(Debug, Deserialize)]
pub struct RawPuzzleData {
    pub id: String,
    pub metadata: RawPuzzleMetadata,
    pub cells: Vec<Vec<RawPuzzleCell>>,
    pub regions: Vec<Vec<RawRowColPair>>,
}

#[derive(Debug, Deserialize)]
pub struct RawPuzzleMetadata {
    pub title: Option<String>,
    pub solution: Option<String>,
    #[serde(default)]
    pub antiking: bool,
    #[serde(default)]
    pub antiknight: bool,
}

#[derive(Debug, Deserialize)]
pub struct RawPuzzleCell {
    pub value: Option<usize>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct RawRowColPair(pub usize, pub usize);

impl RawPuzzleData {
    pub fn load(s: impl AsRef<str>) -> Result<Self> {
        let s = s.as_ref();
        serde_json::from_str(s)
    }
}
