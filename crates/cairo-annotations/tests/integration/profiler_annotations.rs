use crate::helpers::test_project::SCARB_TEMPLATE_TRACE_FILE;
use cairo_annotations::annotations::TryFromDebugInfo;
use cairo_annotations::annotations::profiler::{
    FunctionName, ProfilerAnnotationsV1, VersionedProfilerAnnotations,
};
use cairo_lang_sierra::program::StatementIdx;

#[test]
fn test_versioned() {
    let VersionedProfilerAnnotations::V1(annotations) =
        VersionedProfilerAnnotations::try_from_debug_info(
            SCARB_TEMPLATE_TRACE_FILE.get_debug_info(),
        )
        .unwrap();

    let functions_names = annotations
        .statements_functions
        .get(&StatementIdx(299))
        .unwrap();

    assert_eq!(
        functions_names,
        &[FunctionName("scarb_template::fib".into()),]
    );
}

#[test]
fn test_v1() {
    let annotations =
        ProfilerAnnotationsV1::try_from_debug_info(SCARB_TEMPLATE_TRACE_FILE.get_debug_info())
            .unwrap();

    let functions_names = annotations
        .statements_functions
        .get(&StatementIdx(299))
        .unwrap();

    assert_eq!(
        functions_names,
        &[FunctionName("scarb_template::fib".into()),]
    );
}

#[test]
fn test_serialization_versioned() {
    let debug_info = SCARB_TEMPLATE_TRACE_FILE.get_debug_info();
    let annotations = VersionedProfilerAnnotations::try_from_debug_info(debug_info).unwrap();

    let expected = debug_info
        .annotations
        .get("github.com/software-mansion/cairo-profiler")
        .unwrap();

    assert_eq!(&serde_json::to_value(&annotations).unwrap(), expected);
}

#[test]
fn test_serialization_v1() {
    let debug_info = SCARB_TEMPLATE_TRACE_FILE.get_debug_info();
    let annotations =
        ProfilerAnnotationsV1::try_from_debug_info(SCARB_TEMPLATE_TRACE_FILE.get_debug_info())
            .unwrap();

    let expected = debug_info
        .annotations
        .get("github.com/software-mansion/cairo-profiler")
        .unwrap();

    assert_eq!(&serde_json::to_value(&annotations).unwrap(), expected);
}
