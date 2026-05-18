use crate::annotations::coverage::{SourceCodeSpan, SourceFileFullPath};
use crate::annotations::impl_helpers::impl_namespace;
use serde::de::Error as _;
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
#[serde(deny_unknown_fields)]
pub struct DebuggerAnnotationsV1 {
    pub functions_info: HashMap<SierraFunctionId, FunctionDebugInfo>,
}

/// The debug info of a sierra function.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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

/// Generalized mapping from sierra function id to its debug info.
///
/// Compared to V1:
/// - Each sierra variable may correspond to multiple cairo variables (e.g. let-rebindings
///   `let y = x; let z = y;` all alias the same sierra var). The new value is an ordered
///   list of every cairo binding observed for that sierra var.
/// - Function parameters are recorded explicitly, independent of how (or whether) the
///   function body references each param.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DebuggerAnnotationsV2 {
    pub functions_info: HashMap<SierraFunctionId, FunctionDebugInfoV2>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FunctionDebugInfoV2 {
    /// Path to the user file the function comes from.
    pub function_file_path: SourceFileFullPath,
    /// Span of the function in the user file it comes from.
    pub function_code_span: SourceCodeSpan,
    /// All cairo bindings the compiler observed for each sierra variable, in observation
    /// order. The first entry is the introduction site; subsequent entries are aliases
    /// (e.g. let-rebindings).
    pub sierra_to_cairo_variables: HashMap<SierraVarId, Vec<(CairoVariableName, SourceCodeSpan)>>,
    /// Function parameters in declaration order. Always complete, regardless of whether
    /// the body references each param.
    pub parameters: Vec<ParameterInfo>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ParameterInfo {
    pub sierra_var_id: SierraVarId,
    pub name: CairoVariableName,
    pub definition_span: SourceCodeSpan,
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
        // Buffer into an intermediate value so we can try each typed deserialization
        // without consuming the original deserializer twice. V2 is attempted first;
        // a V2 payload exposes `sierra_to_cairo_variables` and `parameters` fields that
        // V1 lacks, and both versions use `deny_unknown_fields`, so the two shapes are
        // mutually exclusive on the serde side.
        let value = serde_json::Value::deserialize(deserializer)?;
        let v2_err = match serde_json::from_value::<DebuggerAnnotationsV2>(value.clone()) {
            Ok(v2) => return Ok(VersionedDebuggerAnnotations::V2(v2)),
            Err(err) => err,
        };
        let v1_err = match serde_json::from_value::<DebuggerAnnotationsV1>(value) {
            Ok(v1) => return Ok(VersionedDebuggerAnnotations::V1(v1)),
            Err(err) => err,
        };
        Err(D::Error::custom(format!(
            "debugger annotations matched neither V2 ({v2_err}) nor V1 ({v1_err})"
        )))
    }
}

impl_namespace!(
    "github.com/software-mansion-labs/cairo-debugger",
    DebuggerAnnotationsV1,
    DebuggerAnnotationsV2,
    VersionedDebuggerAnnotations
);
