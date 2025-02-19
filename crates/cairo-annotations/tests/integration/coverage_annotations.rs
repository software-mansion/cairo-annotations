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
        .get(&StatementIdx(215))
        .unwrap();

    assert_eq!(
        code_locations,
        &[CodeLocation(
            SourceFileFullPath(format!("{}/src/lib.cairo", trace_file.get_project_dir())),
            SourceCodeSpan {
                start: SourceCodeLocation {
                    line: LineNumber(10),
                    col: ColumnNumber(12)
                },
                end: SourceCodeLocation {
                    line: LineNumber(10),
                    col: ColumnNumber(17)
                }
            },
            None
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
        .get(&StatementIdx(1136))
        .unwrap();

    for x in &annotations.statements_code_locations {
        for y in x.1 {
             // if y.0.0.contains("[") {
                 if let Some(z) =  y.2 {
                     if !z {
                        // panic!("{} {}", x.0, y.0.0);
                     }
                 }
             // }
            if let Some(z) =  y.2 {
                if z {
                    //println!("{}", x.0);
                }
            }
        }
    }

    assert_eq!(
        code_locations,
        &[CodeLocation(
            SourceFileFullPath(format!("{}/src/lib.cairo", trace_file.get_project_dir())),
            SourceCodeSpan {
                start: SourceCodeLocation {
                    line: LineNumber(10),
                    col: ColumnNumber(12)
                },
                end: SourceCodeLocation {
                    line: LineNumber(10),
                    col: ColumnNumber(17)
                }
            },
            None
        )]
    );
}
