use crate::helpers::output_same_as_in_file::AssertSameAsInFile;
use crate::helpers::test_project::TestProject;
use cairo_annotations::map_pcs_to_sierra_statement_ids;
use cairo_annotations::trace_data::CasmLevelInfo;
use cairo_lang_sierra_to_casm::compiler::CairoProgramDebugInfo;

#[test]
fn test_happy_path() {
    let trace_file = TestProject::new("scarb_template")
        .generate_trace_files()
        .first_trace_file();

    let output = map_pcs_to_sierra_statement_ids(
        &trace_file.get_casm_debug_info(),
        trace_file.get_casm_level_info(),
    );

    output.assert_same_as_in_file("map_pcs.txt");
}

#[test]
fn test_empty_sierra_statement_info() {
    let trace_file = TestProject::new("scarb_template")
        .generate_trace_files()
        .first_trace_file();

    let output = map_pcs_to_sierra_statement_ids(
        &CairoProgramDebugInfo {
            sierra_statement_info: Vec::new(),
        },
        trace_file.get_casm_level_info(),
    );

    assert!(output.is_empty());
}

#[test]
fn test_empty_vm_trace() {
    let trace_file = TestProject::new("scarb_template")
        .generate_trace_files()
        .first_trace_file();

    let output = map_pcs_to_sierra_statement_ids(
        &trace_file.get_casm_debug_info(),
        &CasmLevelInfo {
            vm_trace: Vec::new(),
            run_with_call_header: trace_file.get_casm_level_info().run_with_call_header,
        },
    );

    assert!(output.is_empty());
}
