mod common;
mod lda;
mod tax;
mod adc;
mod inx;
mod brk;
mod and;
mod asl;
pub use common::{Inst, INST_FACTORIES, disassemble};