use crate::felt_deserialize::deserialize as felt_deserialize;
use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};
use starknet_types_core::felt::Felt;
use std::collections::HashMap;
use std::ops::{AddAssign, SubAssign};
use strum::VariantArray;
use strum_macros::{Display, EnumString, VariantArray};

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ClassHash(#[serde(deserialize_with = "felt_deserialize")] pub Felt);

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ContractAddress(#[serde(deserialize_with = "felt_deserialize")] pub Felt);

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct EntryPointSelector(#[serde(deserialize_with = "felt_deserialize")] pub Felt);

/// Versioned representation of `CallTrace`.
///
/// Always prefer using this enum when Serializing/Deserializing instead of inner ones.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VersionedCallTrace {
    V1(CallTraceV1),
}

/// Tree structure representing trace of a call.
/// This struct should be serialized and used as an input to cairo-profiler.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallTraceV1 {
    pub entry_point: CallEntryPoint,
    #[serde(rename = "used_execution_resources")]
    pub cumulative_resources: ExecutionResources,
    pub used_l1_resources: L1Resources,
    pub nested_calls: Vec<CallTraceNode>,
    pub cairo_execution_info: Option<CairoExecutionInfo>,
}

/// Struct needed for function level profiling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CairoExecutionInfo {
    /// Path to a file with serialized `ContractClass` or `VersionedProgram`.
    pub source_sierra_path: Utf8PathBuf,
    pub casm_level_info: CasmLevelInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CasmLevelInfo {
    pub run_with_call_header: bool,
    pub vm_trace: Vec<TraceEntry>,
}

/// Enum representing node of a trace of a call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CallTraceNode {
    EntryPointCall(Box<CallTraceV1>),
    DeployWithoutConstructor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceEntry {
    pub pc: usize,
    pub ap: usize,
    pub fp: usize,
}

type SyscallCounter = HashMap<DeprecatedSyscallSelector, SyscallUsage>;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct SyscallUsage {
    pub call_count: usize,
    pub linear_factor: usize,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct ExecutionResources {
    pub vm_resources: VmExecutionResources,
    pub gas_consumed: Option<u64>,
    /// Present for `snforge` >= `0.46.0`.
    pub syscall_counter: Option<SyscallCounter>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
pub struct VmExecutionResources {
    pub n_steps: usize,
    pub n_memory_holes: usize,
    pub builtin_instance_counter: HashMap<String, usize>,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    Deserialize,
    Serialize,
    Eq,
    Hash,
    PartialEq,
    EnumString,
    VariantArray,
)]
pub enum DeprecatedSyscallSelector {
    CallContract,
    DelegateCall,
    DelegateL1Handler,
    Deploy,
    EmitEvent,
    GetBlockHash,
    GetBlockNumber,
    GetBlockTimestamp,
    GetCallerAddress,
    GetContractAddress,
    GetClassHashAt,
    GetExecutionInfo,
    GetSequencerAddress,
    GetTxInfo,
    GetTxSignature,
    Keccak,
    LibraryCall,
    LibraryCallL1Handler,
    ReplaceClass,
    Secp256k1Add,
    Secp256k1GetPointFromX,
    Secp256k1GetXy,
    Secp256k1Mul,
    Secp256k1New,
    Secp256r1Add,
    Secp256r1GetPointFromX,
    Secp256r1GetXy,
    Secp256r1Mul,
    Secp256r1New,
    SendMessageToL1,
    StorageRead,
    StorageWrite,
    Sha256ProcessBlock,
    KeccakRound,
    MetaTxV0,
}

impl DeprecatedSyscallSelector {
    #[must_use]
    pub fn all() -> &'static [Self] {
        Self::VARIANTS
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct CallEntryPoint {
    pub class_hash: Option<ClassHash>,
    pub entry_point_type: EntryPointType,
    pub entry_point_selector: EntryPointSelector,
    pub contract_address: ContractAddress,
    pub call_type: CallType,

    /// Contract name to display instead of contract address
    pub contract_name: Option<String>,
    /// Function name to display instead of entry point selector
    pub function_name: Option<String>,
    /// Calldata length to use for deploy syscall cost estimation
    pub calldata_len: Option<usize>,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum CallType {
    #[default]
    Call = 0,
    Delegate = 1,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum EntryPointType {
    #[serde(rename = "CONSTRUCTOR")]
    Constructor,
    #[serde(rename = "EXTERNAL")]
    #[default]
    External,
    #[serde(rename = "L1_HANDLER")]
    L1Handler,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct L1Resources {
    pub l2_l1_message_sizes: Vec<usize>,
}

impl AddAssign<&VmExecutionResources> for VmExecutionResources {
    fn add_assign(&mut self, other: &VmExecutionResources) {
        self.n_steps += other.n_steps;
        self.n_memory_holes += other.n_memory_holes;

        for (key, value) in &other.builtin_instance_counter {
            *self
                .builtin_instance_counter
                .entry(key.clone())
                .or_default() += *value;
        }
    }
}

impl SubAssign<&VmExecutionResources> for VmExecutionResources {
    fn sub_assign(&mut self, other: &VmExecutionResources) {
        self.n_steps = self.n_steps.saturating_sub(other.n_steps);
        self.n_memory_holes = self.n_memory_holes.saturating_sub(other.n_memory_holes);

        for (key, value) in &other.builtin_instance_counter {
            if let Some(self_value) = self.builtin_instance_counter.get_mut(key) {
                *self_value = self_value.saturating_sub(*value);
            }
        }
        // Remove entries where the value is 0
        self.builtin_instance_counter.retain(|_, value| *value > 0);
    }
}

impl AddAssign<&ExecutionResources> for ExecutionResources {
    fn add_assign(&mut self, other: &ExecutionResources) {
        self.vm_resources += &other.vm_resources;
        self.gas_consumed = match (self.gas_consumed, other.gas_consumed) {
            (Some(self_gas), Some(other_gas)) => Some(self_gas + other_gas),
            (Some(self_gas), None) => Some(self_gas),
            (None, Some(other_gas)) => Some(other_gas),
            (None, None) => None,
        };

        if let Some(other_counter) = &other.syscall_counter {
            let self_counter = self.syscall_counter.get_or_insert_with(HashMap::new);
            for (&selector, usage) in other_counter {
                self_counter
                    .entry(selector)
                    .and_modify(|existing| {
                        existing.call_count += usage.call_count;
                        existing.linear_factor += usage.linear_factor;
                    })
                    .or_insert_with(|| usage.clone());
            }
        }
    }
}

impl SubAssign<&ExecutionResources> for ExecutionResources {
    fn sub_assign(&mut self, other: &ExecutionResources) {
        self.vm_resources -= &other.vm_resources;

        if let Some(other_gas) = other.gas_consumed
            && let Some(self_gas) = &mut self.gas_consumed
        {
            *self_gas = self_gas.saturating_sub(other_gas);
        }

        if let Some(self_counter) = &mut self.syscall_counter
            && let Some(other_counter) = &other.syscall_counter
        {
            for (selector, usage) in other_counter {
                if let Some(self_usage) = self_counter.get_mut(selector) {
                    self_usage.call_count = self_usage.call_count.saturating_sub(usage.call_count);
                    self_usage.linear_factor =
                        self_usage.linear_factor.saturating_sub(usage.linear_factor);
                }
            }
            // Remove entries where both values are 0
            self_counter.retain(|_, usage| usage.call_count > 0 || usage.linear_factor > 0);
        }
    }
}
