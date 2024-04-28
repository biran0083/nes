
use crate::cpu::addressing_mode::{read_param, AddressingMode};
use super::Inst;

pub fn make(mode: AddressingMode, bytes: &[u8]) -> Inst {
    assert!(matches!(mode, AddressingMode::Implied));
    Inst {
        name: "BRK",
        param: read_param(mode, bytes),
        mode,
        f: move |ins, cpu| {
            cpu.x = cpu.a;
            cpu.pc += ins.len();
        },
    }
}

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[(0x00, AddressingMode::Implied)];