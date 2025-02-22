//! This module is only used for internal purposes and does not contain any publicly usable
//! structs
#![allow(missing_docs)]

pub use iai_callgrind_runner::api::{
    Arg as InternalArg, Assistant as InternalAssistant, BinaryBenchmark as InternalBinaryBenchmark,
    BinaryBenchmarkConfig as InternalBinaryBenchmarkConfig,
    BinaryBenchmarkGroup as InternalBinaryBenchmarkGroup, Cmd as InternalCmd,
    ExitWith as InternalExitWith, Fixtures as InternalFixtures,
    LibraryBenchmark as InternalLibraryBenchmark,
    LibraryBenchmarkBench as InternalLibraryBenchmarkBench,
    LibraryBenchmarkBenches as InternalLibraryBenchmarkBenches,
    LibraryBenchmarkConfig as InternalLibraryBenchmarkConfig,
    LibraryBenchmarkGroup as InternalLibraryBenchmarkGroup,
    RawCallgrindArgs as InternalRawCallgrindArgs, Run as InternalRun,
};

#[derive(Debug, Clone)]
pub struct InternalMacroLibBench {
    pub id_display: Option<&'static str>,
    pub args_display: Option<&'static str>,
    pub func: fn(),
    pub config: Option<fn() -> crate::internal::InternalLibraryBenchmarkConfig>,
}
