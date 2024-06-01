use crate::cpu::addressing_mode::AddressingMode;
use super::InstFun;


pub const RUN : InstFun = |ins, cpu| {
    cpu.pc += ins.len();
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
        (0xEA, AddressingMode::Implied),
        (0x1A, AddressingMode::Implied),
        (0x3A, AddressingMode::Implied),
        (0x5A, AddressingMode::Implied),
        (0x7A, AddressingMode::Implied),
        (0xDA, AddressingMode::Implied),
        (0xFA, AddressingMode::Implied),
    ];
