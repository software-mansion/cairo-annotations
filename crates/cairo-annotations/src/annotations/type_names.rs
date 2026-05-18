use crate::annotations::impl_helpers::impl_namespace;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;

/// Versioned representation of Type Names Annotations.
///
/// Always prefer using this enum when Serializing/Deserializing instead of inner ones.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VersionedTypeNamesAnnotations {
    V1(TypeNamesAnnotationsV1),
}

/// The mapping from sierra type id to its debug info (name and members/variants).
///
/// Needs `add-types-debug-info = true`
/// under `[profile.dev.cairo]` in the Scarb config to be generated.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TypeNamesAnnotationsV1 {
    pub structs: HashMap<SierraTypeId, StructInfo>,
    pub enums: HashMap<SierraTypeId, EnumInfo>,
}

/// Debug info of a Sierra struct type.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StructInfo {
    /// Concretized name of the struct type.
    pub name: String,
    /// Names of the struct members.
    pub members: Vec<String>,
}

/// Debug info of a Sierra enum type.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EnumInfo {
    /// Concretized name of the enum type.
    pub name: String,
    /// Names of the enum variants.
    pub variants: Vec<String>,
}

/// An id of a sierra type - equivalent to `id` field of [`cairo_lang_sierra::ids::ConcreteTypeId`].
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SierraTypeId(pub u64);

// We can't use untagged enum here. See https://github.com/serde-rs/json/issues/1103
impl Serialize for VersionedTypeNamesAnnotations {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            VersionedTypeNamesAnnotations::V1(v1) => v1.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for VersionedTypeNamesAnnotations {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        TypeNamesAnnotationsV1::deserialize(deserializer).map(VersionedTypeNamesAnnotations::V1)
    }
}

impl_namespace!(
    "github.com/software-mansion-labs/cairo-debugger/user-types",
    TypeNamesAnnotationsV1,
    VersionedTypeNamesAnnotations
);
