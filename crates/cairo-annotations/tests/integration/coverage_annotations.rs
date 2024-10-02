use crate::helpers::test_project::TestProject;
use cairo_annotations::annotations::coverage::{
    CodeLocation, ColumnNumber, CoverageAnnotationsV1, LineNumber, SourceCodeLocation,
    SourceCodeSpan, SourceFileFullPath, VersionedCoverageAnnotations,
};
use cairo_annotations::annotations::TryFromDebugInfo;
use cairo_lang_sierra::program::StatementIdx;

#[test]
fn test_versioned() {
    let trace_file = TestProject::new("scarb_template")
        .generate_trace_files()
        .first_trace_file();

    let VersionedCoverageAnnotations::V1(annotations) =
        VersionedCoverageAnnotations::try_from_debug_info(trace_file.get_debug_info()).unwrap();

    let code_locations = annotations
        .statements_code_locations
        .get(&StatementIdx(331))
        .unwrap();

    assert_eq!(
        code_locations,
        &[CodeLocation(
            SourceFileFullPath(format!("{}/src/lib.cairo", trace_file.get_project_dir())),
            SourceCodeSpan {
                start: SourceCodeLocation {
                    line: LineNumber(7),
                    col: ColumnNumber(4)
                },
                end: SourceCodeLocation {
                    line: LineNumber(7),
                    col: ColumnNumber(4)
                }
            }
        )]
    );
}

#[test]
fn test_v1() {
    let trace_file = TestProject::new("scarb_template")
        .generate_trace_files()
        .first_trace_file();

    let annotations =
        CoverageAnnotationsV1::try_from_debug_info(trace_file.get_debug_info()).unwrap();

    let code_locations = annotations
        .statements_code_locations
        .get(&StatementIdx(331))
        .unwrap();

    assert_eq!(
        code_locations,
        &[CodeLocation(
            SourceFileFullPath(format!("{}/src/lib.cairo", trace_file.get_project_dir())),
            SourceCodeSpan {
                start: SourceCodeLocation {
                    line: LineNumber(7),
                    col: ColumnNumber(4)
                },
                end: SourceCodeLocation {
                    line: LineNumber(7),
                    col: ColumnNumber(4)
                }
            }
        )]
    );
}
