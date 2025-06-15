use crate::annotations::impl_helpers::impl_namespace;
use cairo_lang_sierra::program::StatementIdx;
use derive_more::Display;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;

/// Versioned representation of Profiler Annotations.
///
/// Always prefer using this enum when Serializing/Deserializing instead of inner ones.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VersionedProfilerAnnotations {
    V1(ProfilerAnnotationsV1),
}

/// The mapping from sierra statement index
/// to stack a fully qualified Cairo paths of the Cairo functions
/// which caused the statement to be generated.
/// And all functions that were inlined
/// or generated along the way up to the first non-inlined function from the original code.
///
/// The vector represents the stack from the least meaningful elements.
///
/// Introduced in Scarb 2.7.0.
///
/// Needs `unstable-add-statements-functions-debug-info = true`
/// under `[profile.dev.cairo]` in the Scarb config to be generated.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProfilerAnnotationsV1 {
    pub statements_functions: HashMap<StatementIdx, Vec<FunctionName>>,
}

/// The fully qualified Cairo path of the Cairo function.
#[derive(
    Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize, Display, Default,
)]
pub struct FunctionName(pub String);

// We can't use untagged enum here. See https://github.com/serde-rs/json/issues/1103
impl Serialize for VersionedProfilerAnnotations {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            VersionedProfilerAnnotations::V1(v1) => v1.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for VersionedProfilerAnnotations {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        ProfilerAnnotationsV1::deserialize(deserializer).map(VersionedProfilerAnnotations::V1)
    }
}

impl_namespace!(
    "github.com/software-mansion/cairo-profiler",
    ProfilerAnnotationsV1,
    VersionedProfilerAnnotations
);
