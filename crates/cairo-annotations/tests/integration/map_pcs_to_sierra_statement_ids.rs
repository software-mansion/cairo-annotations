use crate::helpers::output_same_as_in_file::AssertSameAsInFile;
use crate::helpers::test_project::SCARB_TEMPLATE_TRACE_FILE;
use cairo_annotations::map_pcs_to_sierra_statement_ids;
use cairo_annotations::trace_data::CasmLevelInfo;
use cairo_lang_sierra_to_casm::compiler::CairoProgramDebugInfo;

#[test]
fn test_happy_path() {
    let output = map_pcs_to_sierra_statement_ids(
        &SCARB_TEMPLATE_TRACE_FILE.get_casm_debug_info(),
        SCARB_TEMPLATE_TRACE_FILE.get_casm_level_info(),
        None,
    );

    output.assert_same_as_in_file("map_pcs.txt");
}

#[test]
fn test_empty_sierra_statement_info() {
    let output = map_pcs_to_sierra_statement_ids(
        &CairoProgramDebugInfo {
            sierra_statement_info: Vec::new(),
        },
        SCARB_TEMPLATE_TRACE_FILE.get_casm_level_info(),
        None,
    );

    assert!(output.is_empty());
}

#[test]
fn test_empty_vm_trace() {
    let output = map_pcs_to_sierra_statement_ids(
        &SCARB_TEMPLATE_TRACE_FILE.get_casm_debug_info(),
        &CasmLevelInfo {
            vm_trace: Vec::new(),
            run_with_call_header: SCARB_TEMPLATE_TRACE_FILE
                .get_casm_level_info()
                .run_with_call_header,
        },
        None,
    );

    assert!(output.is_empty());
}
