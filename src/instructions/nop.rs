use crate::cpu::addressing_mode::{load_operand, AddressingMode};
use super::InstFun;


pub const RUN : InstFun = |ins, cpu| {
    cpu.pc += ins.len();
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
        (0xEA, AddressingMode::Implied),
    ];
