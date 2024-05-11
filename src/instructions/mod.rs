mod common;
mod lda;
mod tax;
mod adc;
mod brk;
mod and;
mod asl;
mod bcc;
mod bcs;
mod beq;
mod bit;
mod bmi;
mod bne;
mod bpl;
mod bvc;
mod bvs;
mod clc;
mod sec;
mod cld;
mod sed;
mod cli;
mod sei;
mod clv;
mod cmp;
mod cpx;
mod cpy;
mod dec;
mod dex;
mod dey;
mod eor;
mod inc;
mod inx;
mod iny;
pub use common::{Inst, InstFun, INST_FACTORIES, disassemble};