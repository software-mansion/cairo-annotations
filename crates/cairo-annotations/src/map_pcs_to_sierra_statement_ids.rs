use crate::trace_data::CasmLevelInfo;
use cairo_lang_sierra::program::StatementIdx;
use cairo_lang_sierra_to_casm::compiler::{CairoProgramDebugInfo, SierraStatementDebugInfo};
use serde::{Deserialize, Serialize};

/// Enum to represent the result of mapping a pc to a sierra statement id.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum MappingResult {
    /// The pc was successfully mapped to a sierra statement id.
    SierraStatementIdx(StatementIdx),
    /// The pc was not mapped to a sierra statement id because it was in the header of the program.
    Header,
    /// The pc was not mapped to a sierra statement id because it was outside the function area.
    PcOutOfFunctionArea,
}

impl From<MappingResult> for Option<StatementIdx> {
    fn from(mapping_result: MappingResult) -> Self {
        match mapping_result {
            MappingResult::SierraStatementIdx(statement_idx) => Some(statement_idx),
            MappingResult::Header | MappingResult::PcOutOfFunctionArea => None,
        }
    }
}

/// Function to map the program counters in the trace to the sierra statement ids.
///
/// Returns an empty vector if the sierra statement info is empty or the vm trace is empty.
#[must_use]
pub fn map_pcs_to_sierra_statement_ids(
    CairoProgramDebugInfo {
        sierra_statement_info,
    }: &CairoProgramDebugInfo,
    CasmLevelInfo {
        run_with_call_header,
        vm_trace,
        program_offset,
    }: &CasmLevelInfo,
) -> Vec<MappingResult> {
    if sierra_statement_info.is_empty() {
        return Vec::new();
    }

    // Some CASM programs starts with a header(s) of instructions to wrap the real program.
    // `real_minimal_pc` is the PC in the trace that points to the same CASM instruction which would
    // be in the PC=1 in the original CASM program.
    // If a header is built to have a `ret` last instruction, this is the same as the PC of the last
    // trace entry plus 1. It may also end with something else like `jump rel 0` (like in case of
    // scarb execute), then it's the same as the PC of the last trace entry plus 2.
    // The first instruction after that is the first instruction in the original CASM program.
    // This logic only applies when a header was added to the CASM program, otherwise the
    // `real_minimal_pc` is the default one, which is 1.
    // In some cases there are more headers - an example being scarb execute target `standalone`,
    // which has two: one with `jump rel 0` and a second one which size needs to be included to
    // properly map pcs to statement ids.
    // In order to accommodate such cases, there's an option to set custom program offset here.
    let real_minimal_pc = if let Some(offset) = program_offset {
        offset + 1
    } else {
        run_with_call_header
            .then(|| vm_trace.last())
            .flatten()
            .map_or(1, |trace_entry| trace_entry.pc + 1)
    };

    vm_trace
        .iter()
        .map(|step| step.pc)
        .map(|pc| map_pc_to_sierra_statement_id(sierra_statement_info, pc, real_minimal_pc))
        .collect()
}

fn map_pc_to_sierra_statement_id(
    sierra_statement_info: &[SierraStatementDebugInfo],
    pc: usize,
    real_minimal_pc: usize,
) -> MappingResult {
    if pc < real_minimal_pc {
        return MappingResult::Header;
    }

    // The real pc is equal to (1 + pc - real_minimal_pc). This difference, however
    // is a code offset of the real pc. We need real code offset to map pc to a sierra statement.
    let real_pc_code_offset = pc - real_minimal_pc;

    // The `-1` here can't cause an underflow as the first statement's start offset is always 0,
    // so it is always on the left side of the partition, thus the partition index is > 0.
    // But for sanity, we use saturating_sub.
    let statement_index = StatementIdx(
        sierra_statement_info
            .partition_point(|statement_debug_info| {
                statement_debug_info.start_offset <= real_pc_code_offset
            })
            .saturating_sub(1),
    );

    // End offset is exclusive, and the CASM debug info is sorted in non-descending order by both
    // end offset and start offset. Therefore, the end offset of the last element in that vector
    // is the bytecode length.
    let bytecode_length = sierra_statement_info
        .last()
        .expect("sierra_statement_info is not empty")
        .end_offset;

    // If offset is greater or equal the bytecode length, it means that it is the outside `ret` used
    // for e.g., getting a pointer to builtins costs table, const segments, etc.
    if real_pc_code_offset >= bytecode_length {
        MappingResult::PcOutOfFunctionArea
    } else {
        MappingResult::SierraStatementIdx(statement_index)
    }
}
