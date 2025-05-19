//! Raw fpuzzles data
//!

use std::num::NonZeroUsize;

use serde::{de::Visitor, Deserialize};
use serde_json::Result;

mod convert;

#[derive(Debug, Deserialize)]
pub struct FpuzzlesData {
    pub size: usize,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub ruleset: String,
    pub solution: Option<Vec<NonZeroUsize>>,
    pub grid: Vec<Vec<FpuzzlesCellData>>,

    // Rules
    #[serde(default)]
    pub antiking: bool,
    #[serde(default)]
    pub antiknight: bool,
    #[serde(default)]
    pub quadruple: Vec<FpuzzlesQuadruple>,
    #[serde(default, rename = "diagonal+")]
    pub diagonal_p: bool,
    #[serde(default, rename = "diagonal-")]
    pub diagonal_n: bool,
    #[serde(default)]
    pub disjointgroups: bool,
    #[serde(default)]
    pub clone: Vec<FpuzzlesClones>,
    #[serde(default)]
    pub palindrome: Vec<FpuzzlesLines>,
    #[serde(default)]
    pub odd: Vec<FpuzzlesSingleCell>,
    #[serde(default)]
    pub even: Vec<FpuzzlesSingleCell>,
    #[serde(default)]
    pub nonconsecutive: bool,
    #[serde(default)]
    pub xv: Vec<FpuzzlesXVPair>,
    #[serde(default)]
    pub difference: Vec<FpuzzlesOrthoPair>,
    #[serde(default)]
    pub ratio: Vec<FpuzzlesOrthoPair>,
}

#[derive(Debug, Deserialize)]
pub struct FpuzzlesCellData {
    pub value: Option<NonZeroUsize>,
    pub region: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct FpuzzlesQuadruple {
    pub cells: Vec<FpuzzlesCellRef>,
    pub values: Vec<NonZeroUsize>,
}

#[derive(Debug)]
pub struct FpuzzlesCellRef {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Deserialize)]
pub struct FpuzzlesClones {
    pub cells: Vec<FpuzzlesCellRef>,
    #[serde(rename = "cloneCells")]
    pub clone_cells: Vec<FpuzzlesCellRef>,
}

#[derive(Debug, Deserialize)]
pub struct FpuzzlesLines {
    pub lines: Vec<Vec<FpuzzlesCellRef>>,
}

#[derive(Debug, Deserialize)]
pub struct FpuzzlesSingleCell {
    pub cell: FpuzzlesCellRef,
}

#[derive(Debug, Deserialize)]
pub struct FpuzzlesXVPair {
    pub cells: (FpuzzlesCellRef, FpuzzlesCellRef),
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct FpuzzlesOrthoPair {
    pub cells: (FpuzzlesCellRef, FpuzzlesCellRef),
    pub value: Option<i32>,
}

impl<'de> Deserialize<'de> for FpuzzlesCellRef {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct FpuzzlesCellRefVisitor;
        impl<'de> Visitor<'de> for FpuzzlesCellRefVisitor {
            type Value = FpuzzlesCellRef;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string of the form RNCN")
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                // v is of the form RnCn where n may be 1 or more digits
                let first = v
                    .chars()
                    .next()
                    .ok_or(serde::de::Error::custom("empty cell refs not permitted"))?;

                if !first.eq_ignore_ascii_case(&'r') {
                    return Err(serde::de::Error::custom(
                        "cell ref must be of the form rncn",
                    ));
                }
                let v = v.to_ascii_lowercase();
                if let Some((row, col)) = v[1..].split_once('c') {
                    let row = row.parse().map_err(|e| {
                        serde::de::Error::custom(format!("Failure parsing {row}: {e}"))
                    })?;
                    let col = col.parse().map_err(|e| {
                        serde::de::Error::custom(format!("Failure parsing {col}: {e}"))
                    })?;
                    Ok(FpuzzlesCellRef { row, col })
                } else {
                    Err(serde::de::Error::custom(
                        "cell ref must be of the form rncn",
                    ))
                }
            }
        }

        deserializer.deserialize_str(FpuzzlesCellRefVisitor)
    }
}

impl FpuzzlesData {
    pub fn load(s: impl AsRef<str>) -> Result<Self> {
        let s = s.as_ref();
        serde_json::from_str(s)
    }
}
