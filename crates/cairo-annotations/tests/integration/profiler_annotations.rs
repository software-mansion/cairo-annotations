use crate::helpers::test_project::TestProject;
use cairo_annotations::annotations::TryFromDebugInfo;
use cairo_annotations::annotations::profiler::{
    FunctionName, ProfilerAnnotationsV1, VersionedProfilerAnnotations,
};
use cairo_lang_sierra::program::StatementIdx;

#[test]
fn test_versioned() {
    let trace_file = TestProject::new("scarb_template")
        .generate_trace_files()
        .first_trace_file();

    let VersionedProfilerAnnotations::V1(annotations) =
        VersionedProfilerAnnotations::try_from_debug_info(trace_file.get_debug_info()).unwrap();

    let functions_names = annotations
        .statements_functions
        .get(&StatementIdx(215))
        .unwrap();

    assert_eq!(
        functions_names,
        &[FunctionName("scarb_template::fib".into()),]
    );
}

#[test]
fn test_v1() {
    let trace_file = TestProject::new("scarb_template")
        .generate_trace_files()
        .first_trace_file();

    let annotations =
        ProfilerAnnotationsV1::try_from_debug_info(trace_file.get_debug_info()).unwrap();

    let functions_names = annotations
        .statements_functions
        .get(&StatementIdx(215))
        .unwrap();

    assert_eq!(
        functions_names,
        &[FunctionName("scarb_template::fib".into()),]
    );
}
