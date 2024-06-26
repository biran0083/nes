mod common;
use crate::define_instructions;
use crate::instructions::common::InstructionInfo;
pub use common::{
    disassemble, Inst, InstFun, INST_FACTORIES_BY_NAME_MODE, INST_FACTORIES_BY_OP_CODE,
};
use lazy_static::lazy_static;

define_instructions!(
    adc, and, asl, bcc, bcs, beq, bit, bmi, bne, bpl, brk, bvc, bvs, clc, cld, cli, clv, cmp, cpx,
    cpy, dec, dex, dey, eor, inc, inx, iny, jmp, jsr, lda, ldx, ldy, lsr, nop, ora, pha, php, pla,
    plp, rol, ror, rti, rts, sbc, sec, sed, sei, sta, stx, sty, tax, tay, tsx, txa, txs, tya,
    // unofficial instructions
    slo, sre, lax, lar, kil, isb, dcp, axs, sax, rla, rra, anc,
);
