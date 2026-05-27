use crate::helpers::test_project::SCARB_TEMPLATE_TRACE_FILE;
use cairo_annotations::annotations::TryFromDebugInfo;
use cairo_annotations::annotations::coverage::{
    ColumnNumber, LineNumber, SourceCodeLocation, SourceCodeSpan, SourceFileFullPath,
};
use cairo_annotations::annotations::debugger::{
    DebuggerAnnotationsV1, DebuggerAnnotationsV2, FunctionDebugInfo, FunctionDebugInfoV2,
    SierraFunctionId, SierraVarId, VersionedDebuggerAnnotations,
};
use std::collections::HashMap;

#[test]
fn test_deserialization_versioned() {
    let VersionedDebuggerAnnotations::V1(annotations) =
        VersionedDebuggerAnnotations::try_from_debug_info(
            SCARB_TEMPLATE_TRACE_FILE.get_debug_info(),
        )
        .unwrap()
    else {
        panic!("expected v1 annotations from the v1 trace fixture");
    };

    let fib_func_id = SCARB_TEMPLATE_TRACE_FILE
        .get_program()
        .funcs
        .iter()
        .find(|x| x.id.debug_name.as_ref().unwrap().as_str() == "scarb_template::fib")
        .unwrap()
        .id
        .id;
    let function_debug_info = annotations
        .functions_info
        .get(&SierraFunctionId(fib_func_id))
        .unwrap();

    assert_eq!(
        function_debug_info,
        &FunctionDebugInfo {
            function_file_path: SourceFileFullPath(format!(
                "{}/src/lib.cairo",
                SCARB_TEMPLATE_TRACE_FILE.get_project_dir()
            )),
            function_code_span: SourceCodeSpan {
                start: SourceCodeLocation {
                    line: LineNumber(4),
                    col: ColumnNumber(0)
                },
                end: SourceCodeLocation {
                    line: LineNumber(14),
                    col: ColumnNumber(1)
                }
            },
            sierra_to_cairo_variable: HashMap::from([
                (
                    SierraVarId(2),
                    (
                        "n".to_string(),
                        SourceCodeSpan {
                            start: SourceCodeLocation {
                                line: LineNumber(4),
                                col: ColumnNumber(11)
                            },
                            end: SourceCodeLocation {
                                line: LineNumber(4),
                                col: ColumnNumber(12)
                            }
                        }
                    )
                ),
                (
                    SierraVarId(3),
                    (
                        "a".to_string(),
                        SourceCodeSpan {
                            start: SourceCodeLocation {
                                line: LineNumber(5),
                                col: ColumnNumber(12)
                            },
                            end: SourceCodeLocation {
                                line: LineNumber(5),
                                col: ColumnNumber(13)
                            }
                        }
                    )
                ),
                (
                    SierraVarId(4),
                    (
                        "b".to_string(),
                        SourceCodeSpan {
                            start: SourceCodeLocation {
                                line: LineNumber(6),
                                col: ColumnNumber(12)
                            },
                            end: SourceCodeLocation {
                                line: LineNumber(6),
                                col: ColumnNumber(13)
                            }
                        }
                    )
                )
            ])
        }
    );
}

#[test]
fn test_deserialization_v1() {
    let annotations =
        DebuggerAnnotationsV1::try_from_debug_info(SCARB_TEMPLATE_TRACE_FILE.get_debug_info())
            .unwrap();

    let fib_func_id = SCARB_TEMPLATE_TRACE_FILE
        .get_program()
        .funcs
        .iter()
        .find(|x| x.id.debug_name.as_ref().unwrap().as_str() == "scarb_template::fib")
        .unwrap()
        .id
        .id;
    let function_debug_info = annotations
        .functions_info
        .get(&SierraFunctionId(fib_func_id))
        .unwrap();

    assert_eq!(
        function_debug_info,
        &FunctionDebugInfo {
            function_file_path: SourceFileFullPath(format!(
                "{}/src/lib.cairo",
                SCARB_TEMPLATE_TRACE_FILE.get_project_dir()
            )),
            function_code_span: SourceCodeSpan {
                start: SourceCodeLocation {
                    line: LineNumber(4),
                    col: ColumnNumber(0)
                },
                end: SourceCodeLocation {
                    line: LineNumber(14),
                    col: ColumnNumber(1)
                }
            },
            sierra_to_cairo_variable: HashMap::from([
                (
                    SierraVarId(2),
                    (
                        "n".to_string(),
                        SourceCodeSpan {
                            start: SourceCodeLocation {
                                line: LineNumber(4),
                                col: ColumnNumber(11)
                            },
                            end: SourceCodeLocation {
                                line: LineNumber(4),
                                col: ColumnNumber(12)
                            }
                        }
                    )
                ),
                (
                    SierraVarId(3),
                    (
                        "a".to_string(),
                        SourceCodeSpan {
                            start: SourceCodeLocation {
                                line: LineNumber(5),
                                col: ColumnNumber(12)
                            },
                            end: SourceCodeLocation {
                                line: LineNumber(5),
                                col: ColumnNumber(13)
                            }
                        }
                    )
                ),
                (
                    SierraVarId(4),
                    (
                        "b".to_string(),
                        SourceCodeSpan {
                            start: SourceCodeLocation {
                                line: LineNumber(6),
                                col: ColumnNumber(12)
                            },
                            end: SourceCodeLocation {
                                line: LineNumber(6),
                                col: ColumnNumber(13)
                            }
                        }
                    )
                )
            ])
        }
    );
}

#[test]
fn test_serialization_versioned() {
    let debug_info = SCARB_TEMPLATE_TRACE_FILE.get_debug_info();
    let annotations = VersionedDebuggerAnnotations::try_from_debug_info(debug_info).unwrap();

    let expected = debug_info
        .annotations
        .get("github.com/software-mansion-labs/cairo-debugger")
        .unwrap();

    assert_eq!(&serde_json::to_value(&annotations).unwrap(), expected);
}

#[test]
fn test_serialization_v1() {
    let debug_info = SCARB_TEMPLATE_TRACE_FILE.get_debug_info();
    let annotations =
        DebuggerAnnotationsV1::try_from_debug_info(SCARB_TEMPLATE_TRACE_FILE.get_debug_info())
            .unwrap();

    let expected = debug_info
        .annotations
        .get("github.com/software-mansion-labs/cairo-debugger")
        .unwrap();

    assert_eq!(&serde_json::to_value(&annotations).unwrap(), expected);
}

fn sample_span(line: usize, col: usize) -> SourceCodeSpan {
    SourceCodeSpan {
        start: SourceCodeLocation {
            line: LineNumber(line),
            col: ColumnNumber(col),
        },
        end: SourceCodeLocation {
            line: LineNumber(line),
            col: ColumnNumber(col + 1),
        },
    }
}

fn sample_v2_annotations() -> DebuggerAnnotationsV2 {
    DebuggerAnnotationsV2 {
        functions_info: HashMap::from([(
            SierraFunctionId(7),
            FunctionDebugInfoV2 {
                function_file_path: SourceFileFullPath("/proj/src/lib.cairo".to_string()),
                function_code_span: sample_span(1, 0),
                sierra_to_cairo_variables: HashMap::from([
                    (
                        // The user's example: Sierra slot [0] starts as `x` (param)
                        // and gets rebound to `y` at the `let mut y = x` statement.
                        SierraVarId(0),
                        vec![
                            ("x".to_string(), sample_span(1, 7)),
                            ("y".to_string(), sample_span(2, 12)),
                        ],
                    ),
                    (SierraVarId(1), vec![("z".to_string(), sample_span(3, 12))]),
                ]),
            },
        )]),
    }
}

#[test]
fn test_v2_round_trip_via_versioned() {
    let v2 = sample_v2_annotations();
    let versioned = VersionedDebuggerAnnotations::V2(v2.clone());

    let json = serde_json::to_value(&versioned).unwrap();
    let decoded: VersionedDebuggerAnnotations = serde_json::from_value(json).unwrap();

    let VersionedDebuggerAnnotations::V2(decoded_v2) = decoded else {
        panic!("v2 payload should round-trip as v2, not v1");
    };
    assert_eq!(decoded_v2, v2);
}

#[test]
fn test_v1_payload_still_deserializes_as_v1() {
    // A v1 JSON payload, even after adding v2, must still decode as v1
    // (debugger relies on this fallback for older artifacts).
    let v1 = DebuggerAnnotationsV1 {
        functions_info: HashMap::from([(
            SierraFunctionId(1),
            FunctionDebugInfo {
                function_file_path: SourceFileFullPath("/proj/src/lib.cairo".to_string()),
                function_code_span: sample_span(1, 0),
                sierra_to_cairo_variable: HashMap::from([(
                    SierraVarId(0),
                    ("x".to_string(), sample_span(1, 7)),
                )]),
            },
        )]),
    };

    let json = serde_json::to_value(&v1).unwrap();
    let decoded: VersionedDebuggerAnnotations = serde_json::from_value(json).unwrap();

    let VersionedDebuggerAnnotations::V1(decoded_v1) = decoded else {
        panic!("v1 payload must decode as v1");
    };
    assert_eq!(decoded_v1, v1);
}
