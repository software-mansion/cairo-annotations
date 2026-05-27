use crate::annotations::coverage::{SourceCodeSpan, SourceFileFullPath};
use crate::annotations::impl_helpers::impl_namespace;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;

/// Versioned representation of Debugger Annotations.
///
/// Always prefer using this enum when Serializing/Deserializing instead of inner ones.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VersionedDebuggerAnnotations {
    V1(DebuggerAnnotationsV1),
    V2(DebuggerAnnotationsV2),
}

/// The mapping from sierra function id to its debug info.
///
/// Introduced in Scarb 2.15.0.
///
/// Needs `add-functions-debug-info = true`
/// under `[profile.dev.cairo]` in the Scarb config to be generated.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DebuggerAnnotationsV1 {
    pub functions_info: HashMap<SierraFunctionId, FunctionDebugInfo>,
}

/// The debug info of a sierra function.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FunctionDebugInfo {
    /// Path to the user file the function comes from.
    pub function_file_path: SourceFileFullPath,
    /// Span of the function in the user file it comes from.
    pub function_code_span: SourceCodeSpan,
    /// Mapping from a sierra variable to a cairo variable (its name and definition span).
    /// The sierra variable value corresponds to the cairo variable value at some point during
    /// execution of the function code.
    pub sierra_to_cairo_variable: HashMap<SierraVarId, (CairoVariableName, SourceCodeSpan)>,
}

/// The mapping from Sierra function id to its debug info (v2).
///
/// Compared to [`DebuggerAnnotationsV1`], the Sierra -> Cairo variable mapping is now a multimap.
/// It solves the issue of re-used Sierra variable IDs. For example, a Cairo function:
/// ```ignore
/// fn foo(x: felt252) {
///     let y = x;
/// }
/// ```
/// produces a following Sierra:
/// ```ignore
/// F1:
/// store_temp<felt252>([0]) -> [0];
/// drop<felt252>([0]);
/// ```
/// meaning that Sierra variable ID `[0]` is used to represent the value
/// of both `x` and `y` Cairo variables.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DebuggerAnnotationsV2 {
    pub functions_info: HashMap<SierraFunctionId, FunctionDebugInfoV2>,
}

/// The debug info of a sierra function (V2).
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FunctionDebugInfoV2 {
    /// Path to the user file the function comes from.
    pub function_file_path: SourceFileFullPath,
    /// Span of the function in the user file it comes from.
    pub function_code_span: SourceCodeSpan,
    /// All Cairo variables observed for each Sierra var, in appearance order.
    /// The first entry is the original binding (function parameter or first `let`);
    /// later entries are rebindings sharing the same Sierra ID.
    pub sierra_to_cairo_variables: HashMap<SierraVarId, Vec<(CairoVariableName, SourceCodeSpan)>>,
}

type CairoVariableName = String;

/// An id of a sierra function - equivalent to `id` field of [`cairo_lang_sierra::ids::FunctionId`].
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SierraFunctionId(pub u64);

/// An id of a sierra variable - equivalent to `id` field of [`cairo_lang_sierra::ids::VarId`].
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SierraVarId(pub u64);

// We can't use untagged enum here. See https://github.com/serde-rs/json/issues/1103
impl Serialize for VersionedDebuggerAnnotations {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            VersionedDebuggerAnnotations::V1(v1) => v1.serialize(serializer),
            VersionedDebuggerAnnotations::V2(v2) => v2.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for VersionedDebuggerAnnotations {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // The two payloads are structurally distinct (`functions_info` values differ),
        // so we can attempt V2 first and fall back to V1.
        let value = serde_json::Value::deserialize(deserializer)?;

        if let Ok(v2) = DebuggerAnnotationsV2::deserialize(&value) {
            return Ok(VersionedDebuggerAnnotations::V2(v2));
        }

        DebuggerAnnotationsV1::deserialize(&value)
            .map(VersionedDebuggerAnnotations::V1)
            .map_err(serde::de::Error::custom)
    }
}

impl_namespace!(
    "github.com/software-mansion-labs/cairo-debugger",
    DebuggerAnnotationsV1,
    DebuggerAnnotationsV2,
    VersionedDebuggerAnnotations
);
