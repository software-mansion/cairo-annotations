use crate::helpers::test_project::SCARB_TEMPLATE_TRACE_FILE;
use cairo_annotations::annotations::TryFromDebugInfo;
use cairo_annotations::annotations::coverage::{
    CodeLocation, ColumnNumber, CoverageAnnotationsV1, LineNumber, SourceCodeLocation,
    SourceCodeSpan, SourceFileFullPath, VersionedCoverageAnnotations,
};
use cairo_lang_sierra::program::StatementIdx;

#[test]
fn test_deserialization_versioned() {
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
fn test_deserialization_v1() {
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

#[test]
fn test_serialization_versioned() {
    let debug_info = SCARB_TEMPLATE_TRACE_FILE.get_debug_info();
    let annotations = VersionedCoverageAnnotations::try_from_debug_info(debug_info).unwrap();

    let expected = debug_info
        .annotations
        .get("github.com/software-mansion/cairo-coverage")
        .unwrap();

    assert_eq!(&serde_json::to_value(&annotations).unwrap(), expected);
}

#[test]
fn test_serialization_v1() {
    let debug_info = SCARB_TEMPLATE_TRACE_FILE.get_debug_info();
    let annotations =
        CoverageAnnotationsV1::try_from_debug_info(SCARB_TEMPLATE_TRACE_FILE.get_debug_info())
            .unwrap();

    let expected = debug_info
        .annotations
        .get("github.com/software-mansion/cairo-coverage")
        .unwrap();

    assert_eq!(&serde_json::to_value(&annotations).unwrap(), expected);
}
