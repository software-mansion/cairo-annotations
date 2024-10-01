use assert_fs::fixture::PathCopy;
use assert_fs::TempDir;
use cairo_lang_sierra::debug_info::DebugInfo;
use cairo_lang_sierra::program::{ProgramArtifact, VersionedProgram};
use cairo_lang_sierra_to_casm::compiler::{CairoProgramDebugInfo, SierraToCasmConfig};
use cairo_lang_sierra_to_casm::metadata::{calc_metadata, MetadataComputationConfig};
use serde::de::DeserializeOwned;
use snapbox::cmd::Command as SnapboxCommand;
use std::fs;
use std::path::PathBuf;
use trace_data::{CairoExecutionInfo, CallTrace, CasmLevelInfo};

pub struct TestProject {
    dir: TempDir,
}

impl TestProject {
    pub fn new(test_project_name: &str) -> Self {
        let dir = TempDir::new().unwrap();

        dir.copy_from(
            format!("tests/data/{test_project_name}/"),
            &["*.toml", "*.cairo"],
        )
        .unwrap();

        Self { dir }
    }

    pub fn generate_trace_files(self) -> TestProjectOutput {
        SnapboxCommand::new("snforge")
            .arg("test")
            .arg("--save-trace-data")
            .current_dir(&self.dir)
            .assert()
            .success();
        TestProjectOutput(self)
    }
}

pub struct TestProjectOutput(TestProject);

impl TestProjectOutput {
    pub fn first_trace_file(self) -> TraceFile {
        let trace_path = self.0.dir.path().join("snfoundry_trace");

        let trace_file_path = fs::read_dir(&trace_path)
            .unwrap()
            .next()
            .map(|entry| entry.unwrap().path())
            .unwrap();

        let call_trace: CallTrace = read_and_deserialize(&trace_file_path);

        let cairo_execution_info = call_trace.cairo_execution_info.unwrap();
        let VersionedProgram::V1 { program, .. } =
            read_and_deserialize(&cairo_execution_info.source_sierra_path.clone().into());

        let project_dir = self.0.dir.path().canonicalize().unwrap();
        TraceFile {
            project_dir,
            cairo_execution_info,
            program,
        }
    }
}

pub struct TraceFile {
    project_dir: PathBuf,
    cairo_execution_info: CairoExecutionInfo,
    program: ProgramArtifact,
}

impl TraceFile {
    pub fn get_project_dir(&self) -> String {
        self.project_dir.display().to_string()
    }
    pub fn get_casm_level_info(&self) -> &CasmLevelInfo {
        &self.cairo_execution_info.casm_level_info
    }

    pub fn get_debug_info(&self) -> &DebugInfo {
        self.program.debug_info.as_ref().unwrap()
    }

    pub fn get_casm_debug_info(&self) -> CairoProgramDebugInfo {
        cairo_lang_sierra_to_casm::compiler::compile(
            &self.program.program,
            &calc_metadata(&self.program.program, MetadataComputationConfig::default()).unwrap(),
            SierraToCasmConfig {
                gas_usage_check: false,
                max_bytecode_size: usize::MAX,
            },
        )
        .map(|casm| casm.debug_info)
        .unwrap()
    }
}

fn read_and_deserialize<T: DeserializeOwned>(file_path: &PathBuf) -> T {
    let content = fs::read_to_string(file_path).unwrap();
    serde_json::from_str(&content).unwrap()
}
