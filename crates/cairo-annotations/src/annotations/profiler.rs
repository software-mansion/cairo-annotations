use crate::annotations::impl_helpers::impl_namespace;
use cairo_lang_sierra::program::{Program, StatementIdx};
use derive_more::Display;
use lazy_static::lazy_static;
use regex::Regex;
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

lazy_static! {
    static ref RE_LOOP_FUNC: Regex = Regex::new(r"\[expr\d*\]")
        .expect("Failed to create regex normalising loop functions names");
    static ref RE_MONOMORPHIZATION: Regex = Regex::new(r"<.*>")
        .expect("Failed to create regex normalising mononorphised generic functions names");
}

/// The fully qualified Cairo path of the Cairo function.
#[derive(
    Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize, Display, Default,
)]
pub struct FunctionName(pub String);

impl FunctionName {
    #[must_use]
    pub fn from_sierra_statement_idx(
        statement_idx: StatementIdx,
        sierra_program: &Program,
        split_generics: bool,
    ) -> Self {
        // The `-1` here can't cause an underflow as the statement id of first function's entrypoint is
        // always 0, so it is always on the left side of the partition, thus the partition index is > 0.
        let function_idx = sierra_program
            .funcs
            .partition_point(|f| f.entry_point.0 <= statement_idx.0)
            - 1;
        let function_name = sierra_program.funcs[function_idx].id.to_string();
        // Remove suffix in case of loop function e.g. `[expr36]`.
        let function_name = RE_LOOP_FUNC.replace(&function_name, "");
        // Remove parameters from monomorphised Cairo generics e.g. `<felt252>`.
        let function_name = if split_generics {
            function_name
        } else {
            RE_MONOMORPHIZATION.replace(&function_name, "")
        };

        Self(function_name.to_string())
    }
}

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
