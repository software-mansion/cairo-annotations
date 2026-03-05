use crate::helpers::test_project::SCARB_TEMPLATE_TRACE_FILE;
use cairo_annotations::annotations::TryFromDebugInfo;
use cairo_annotations::annotations::coverage::{
    ColumnNumber, LineNumber, SourceCodeLocation, SourceCodeSpan, SourceFileFullPath,
};
use cairo_annotations::annotations::debugger::{
    DebuggerAnnotationsV1, FunctionDebugInfo, SierraFunctionId, SierraVarId,
    VersionedDebuggerAnnotations,
};
use std::collections::HashMap;

#[test]
fn test_deserialization_versioned() {
    let VersionedDebuggerAnnotations::V1(annotations) =
        VersionedDebuggerAnnotations::try_from_debug_info(
            SCARB_TEMPLATE_TRACE_FILE.get_debug_info(),
        )
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
