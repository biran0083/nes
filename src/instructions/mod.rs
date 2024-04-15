mod common;
mod lda;
mod tax;
mod adc;
mod idx;
mod brk;
pub use common::{make_inst_factories_by_op_code, Inst, InstFactory, disassemble};