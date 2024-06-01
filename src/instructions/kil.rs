use crate::cpu::addressing_mode::AddressingMode;
use super::InstFun;


pub const RUN : InstFun = |_ins, cpu| {
    cpu.halt = true;
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
        (0x02, AddressingMode::Implied),
        (0x12, AddressingMode::Implied),
        (0x22, AddressingMode::Implied),
        (0x32, AddressingMode::Implied),
        (0x42, AddressingMode::Implied),
        (0x52, AddressingMode::Implied),
        (0x62, AddressingMode::Implied),
        (0x72, AddressingMode::Implied),
        (0x92, AddressingMode::Implied),
        (0xB2, AddressingMode::Implied),
        (0xD2, AddressingMode::Implied),
        (0xF2, AddressingMode::Implied),
    ];
