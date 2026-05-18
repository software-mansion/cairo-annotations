use crate::helpers::test_project::SCARB_TEMPLATE_TRACE_FILE;
use cairo_annotations::annotations::TryFromDebugInfo;
use cairo_annotations::annotations::type_names::{
    EnumInfo, SierraTypeId, StructInfo, TypeNamesAnnotationsV1, VersionedTypeNamesAnnotations,
};

fn find_in_structs(annotations: &TypeNamesAnnotationsV1, name: &str) -> (SierraTypeId, StructInfo) {
    annotations
        .structs
        .iter()
        .find(|(_, v)| v.name == name)
        .map_or_else(
            || panic!("struct {name} not found in annotations"),
            |(k, v)| (k.clone(), v.clone()),
        )
}

fn find_in_enums(annotations: &TypeNamesAnnotationsV1, name: &str) -> (SierraTypeId, EnumInfo) {
    annotations
        .enums
        .iter()
        .find(|(_, v)| v.name == name)
        .map_or_else(
            || panic!("enum {name} not found in annotations"),
            |(k, v)| (k.clone(), v.clone()),
        )
}

#[test]
fn test_deserialization_versioned() {
    let VersionedTypeNamesAnnotations::V1(annotations) =
        VersionedTypeNamesAnnotations::try_from_debug_info(
            SCARB_TEMPLATE_TRACE_FILE.get_debug_info(),
        )
        .unwrap();

    let (_, panic_info) = find_in_structs(&annotations, "core::panics::Panic");
    assert_eq!(
        panic_info,
        StructInfo {
            name: "core::panics::Panic".to_string(),
            members: vec![],
        }
    );

    let (_, bool_info) = find_in_enums(&annotations, "core::bool");
    assert_eq!(
        bool_info,
        EnumInfo {
            name: "core::bool".to_string(),
            variants: vec!["False".to_string(), "True".to_string()],
        }
    );
}

#[test]
fn test_deserialization_v1() {
    let annotations =
        TypeNamesAnnotationsV1::try_from_debug_info(SCARB_TEMPLATE_TRACE_FILE.get_debug_info())
            .unwrap();

    let (_, panic_info) = find_in_structs(&annotations, "core::panics::Panic");
    assert_eq!(
        panic_info,
        StructInfo {
            name: "core::panics::Panic".to_string(),
            members: vec![],
        }
    );

    let (_, bool_info) = find_in_enums(&annotations, "core::bool");
    assert_eq!(
        bool_info,
        EnumInfo {
            name: "core::bool".to_string(),
            variants: vec!["False".to_string(), "True".to_string()],
        }
    );
}

#[test]
fn test_serialization_versioned() {
    let debug_info = SCARB_TEMPLATE_TRACE_FILE.get_debug_info();
    let annotations = VersionedTypeNamesAnnotations::try_from_debug_info(debug_info).unwrap();

    let expected = debug_info
        .annotations
        .get("github.com/software-mansion-labs/cairo-debugger/user-types")
        .unwrap();

    assert_eq!(&serde_json::to_value(&annotations).unwrap(), expected);
}

#[test]
fn test_serialization_v1() {
    let debug_info = SCARB_TEMPLATE_TRACE_FILE.get_debug_info();
    let annotations =
        TypeNamesAnnotationsV1::try_from_debug_info(SCARB_TEMPLATE_TRACE_FILE.get_debug_info())
            .unwrap();

    let expected = debug_info
        .annotations
        .get("github.com/software-mansion-labs/cairo-debugger/user-types")
        .unwrap();

    assert_eq!(&serde_json::to_value(&annotations).unwrap(), expected);
}
