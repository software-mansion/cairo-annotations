use crate::annotations::impl_helpers::impl_namespace;
use cairo_lang_sierra::program::StatementIdx;
use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VersionedCoverageAnnotations {
    V1(CoverageAnnotationsV1),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CoverageAnnotationsV1 {
    pub statements_code_locations: HashMap<StatementIdx, Vec<CodeLocation>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CodeLocation(pub SourceFileFullPath, pub SourceCodeSpan);

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SourceFileFullPath(pub String);

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SourceCodeSpan {
    pub start: SourceCodeLocation,
    pub end: SourceCodeLocation,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SourceCodeLocation {
    pub line: LineNumber,
    pub col: ColumnNumber,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Hash,
    Ord,
    PartialOrd,
    Serialize,
    Deserialize,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign,
)]
pub struct ColumnNumber(pub usize);

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Hash,
    Ord,
    PartialOrd,
    Serialize,
    Deserialize,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign,
)]
pub struct LineNumber(pub usize);

// We can't use untagged enum here. See https://github.com/serde-rs/json/issues/1103
impl Serialize for VersionedCoverageAnnotations {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            VersionedCoverageAnnotations::V1(v1) => v1.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for VersionedCoverageAnnotations {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        CoverageAnnotationsV1::deserialize(deserializer).map(VersionedCoverageAnnotations::V1)
    }
}

impl_namespace!(
    "github.com/software-mansion/cairo-coverage",
    CoverageAnnotationsV1,
    VersionedCoverageAnnotations
);
