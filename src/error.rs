
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
}