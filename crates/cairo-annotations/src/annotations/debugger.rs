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
}

/// The mapping from x to x. TODO
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
        }
    }
}

impl<'de> Deserialize<'de> for VersionedDebuggerAnnotations {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        DebuggerAnnotationsV1::deserialize(deserializer).map(VersionedDebuggerAnnotations::V1)
    }
}

impl_namespace!(
    "github.com/software-mansion-labs/cairo-debugger",
    DebuggerAnnotationsV1,
    VersionedDebuggerAnnotations
);
