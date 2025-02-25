use crate::helpers::test_project::SCARB_TEMPLATE_TRACE_FILE;
use cairo_annotations::annotations::TryFromDebugInfo;
use cairo_annotations::annotations::coverage::{
    CodeLocation, ColumnNumber, CoverageAnnotationsV1, LineNumber, SourceCodeLocation,
    SourceCodeSpan, SourceFileFullPath, VersionedCoverageAnnotations,
};
use cairo_lang_sierra::program::StatementIdx;

#[test]
fn test_versioned() {
    let VersionedCoverageAnnotations::V1(annotations) =
        VersionedCoverageAnnotations::try_from_debug_info(
            SCARB_TEMPLATE_TRACE_FILE.get_debug_info(),
        )
        .unwrap();

    let code_locations = annotations
        .statements_code_locations
        .get(&StatementIdx(215))
        .unwrap();

    assert_eq!(
        code_locations,
        &[CodeLocation(
            SourceFileFullPath(format!(
                "{}/src/lib.cairo",
                SCARB_TEMPLATE_TRACE_FILE.get_project_dir()
            )),
            SourceCodeSpan {
                start: SourceCodeLocation {
                    line: LineNumber(10),
                    col: ColumnNumber(12)
                },
                end: SourceCodeLocation {
                    line: LineNumber(10),
                    col: ColumnNumber(17)
                }
            }
        )]
    );
}

#[test]
fn test_v1() {
    let annotations =
        CoverageAnnotationsV1::try_from_debug_info(SCARB_TEMPLATE_TRACE_FILE.get_debug_info())
            .unwrap();

    let code_locations = annotations
        .statements_code_locations
        .get(&StatementIdx(215))
        .unwrap();

    assert_eq!(
        code_locations,
        &[CodeLocation(
            SourceFileFullPath(format!(
                "{}/src/lib.cairo",
                SCARB_TEMPLATE_TRACE_FILE.get_project_dir()
            )),
            SourceCodeSpan {
                start: SourceCodeLocation {
                    line: LineNumber(10),
                    col: ColumnNumber(12)
                },
                end: SourceCodeLocation {
                    line: LineNumber(10),
                    col: ColumnNumber(17)
                }
            }
        )]
    );
}
