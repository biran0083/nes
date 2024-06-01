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
        (0x04, AddressingMode::ZeroPage),
        (0x14, AddressingMode::ZeroPageX),
        (0x34, AddressingMode::ZeroPageX),
        (0x44, AddressingMode::ZeroPage),
        (0x54, AddressingMode::ZeroPageX),
        (0x64, AddressingMode::ZeroPage),
        (0x74, AddressingMode::ZeroPageX),
        (0x80, AddressingMode::Immediate),
        (0x82, AddressingMode::Immediate),
        (0x89, AddressingMode::Immediate),
        (0xC2, AddressingMode::Immediate),
        (0xD4, AddressingMode::ZeroPageX),
        (0xE2, AddressingMode::Immediate),
        (0xF4, AddressingMode::ZeroPageX),
    ];
