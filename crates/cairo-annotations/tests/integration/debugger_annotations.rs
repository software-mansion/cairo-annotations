use crate::helpers::test_project::SCARB_TEMPLATE_TRACE_FILE;
use cairo_annotations::annotations::TryFromDebugInfo;
use cairo_annotations::annotations::coverage::{
    ColumnNumber, LineNumber, SourceCodeLocation, SourceCodeSpan, SourceFileFullPath,
};
use cairo_annotations::annotations::debugger::{
    DebuggerAnnotationsV1, DebuggerAnnotationsV2, FunctionDebugInfo, FunctionDebugInfoV2,
    ParameterInfo, SierraFunctionId, SierraVarId, VersionedDebuggerAnnotations,
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
        panic!("expected V1 annotations from the scarb template trace file");
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

fn sample_v2_annotations() -> DebuggerAnnotationsV2 {
    let span = |line: usize, start_col: usize, end_col: usize| SourceCodeSpan {
        start: SourceCodeLocation {
            line: LineNumber(line),
            col: ColumnNumber(start_col),
        },
        end: SourceCodeLocation {
            line: LineNumber(line),
            col: ColumnNumber(end_col),
        },
    };

    DebuggerAnnotationsV2 {
        functions_info: HashMap::from([(
            SierraFunctionId(42),
            FunctionDebugInfoV2 {
                function_file_path: SourceFileFullPath("/tmp/foo.cairo".to_string()),
                function_code_span: span(1, 0, 50),
                sierra_to_cairo_variables: HashMap::from([(
                    SierraVarId(0),
                    vec![
                        ("x".to_string(), span(1, 7, 8)),
                        ("y".to_string(), span(2, 8, 9)),
                    ],
                )]),
                parameters: vec![ParameterInfo {
                    sierra_var_id: SierraVarId(0),
                    name: "x".to_string(),
                    definition_span: span(1, 7, 8),
                }],
            },
        )]),
    }
}

#[test]
fn test_v2_round_trip() {
    let annotations = sample_v2_annotations();
    let value = serde_json::to_value(&annotations).unwrap();
    let round_tripped: DebuggerAnnotationsV2 = serde_json::from_value(value).unwrap();
    assert_eq!(annotations, round_tripped);
}

#[test]
fn test_versioned_dispatches_to_v2() {
    let annotations = sample_v2_annotations();
    let value = serde_json::to_value(&annotations).unwrap();
    let versioned: VersionedDebuggerAnnotations = serde_json::from_value(value).unwrap();
    let VersionedDebuggerAnnotations::V2(v2) = versioned else {
        panic!("expected V2 dispatch");
    };
    assert_eq!(v2, annotations);
}

#[test]
fn test_versioned_dispatches_to_v1_for_v1_payload() {
    // V1 payload travelling through the Versioned enum must still resolve to V1, even
    // though V2 is tried first.
    let v1 = DebuggerAnnotationsV1::try_from_debug_info(
        SCARB_TEMPLATE_TRACE_FILE.get_debug_info(),
    )
    .unwrap();
    let value = serde_json::to_value(&v1).unwrap();
    let versioned: VersionedDebuggerAnnotations = serde_json::from_value(value).unwrap();
    let VersionedDebuggerAnnotations::V1(round_tripped) = versioned else {
        panic!("expected V1 dispatch for V1 payload");
    };
    assert_eq!(round_tripped, v1);
}

#[test]
fn test_v1_payload_rejected_as_v2() {
    // Sanity check: a V1 payload must not accidentally parse as V2.
    let v1 = DebuggerAnnotationsV1::try_from_debug_info(
        SCARB_TEMPLATE_TRACE_FILE.get_debug_info(),
    )
    .unwrap();
    let value = serde_json::to_value(&v1).unwrap();
    assert!(serde_json::from_value::<DebuggerAnnotationsV2>(value).is_err());
}

#[test]
fn test_v2_payload_rejected_as_v1() {
    // And vice versa: a V2 payload must not accidentally parse as V1.
    let v2 = sample_v2_annotations();
    let value = serde_json::to_value(&v2).unwrap();
    assert!(serde_json::from_value::<DebuggerAnnotationsV1>(value).is_err());
}
