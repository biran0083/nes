use thiserror::Error;

#[derive(Error, Debug)]
pub enum NesError {
    #[error("IO error")]
    Io,
    #[error("ParseInt error")]
    ParseInt,
    #[error("Instruction not found: {0}")]
    InstNotFound(String),
    #[error("Failed to assemble instruction: {0}")]
    AssemblerFailure(String),
    #[error("Failed to disassemble instruction: {0}")]
    DisassemblerFailure(String),
    #[error("Invalid file extension: {0}")]
    InvalidFileExtension(String),
    #[error("Failed to parse cpu state")]
    ParseCpuStateError,
    #[error("CPU halted")]
    HaltError,
    #[error("Reach end of the state file")]
    EndOfFile,
    #[error("Test failed: {0}")]
    TestFailed(String),
    #[error("Invalid color index: {0}")]
    InvalidColorIndex(u8),
}
