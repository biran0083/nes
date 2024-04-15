use crate::cpu::addressing_mode::{read_param, AddressingMode};
use super::Inst;

pub fn make(mode: AddressingMode, bytes: &[u8]) -> Inst {
    assert!(matches!(mode, AddressingMode::Implied));
    Inst {
        name: "INX",
        param: read_param(mode, bytes),
        mode,
        f: move |ins, cpu| {
            cpu.X = cpu.X + 1;
            cpu.update_z(cpu.X);
            cpu.update_n(cpu.X);
            cpu.PC += ins.len();
        },
    }
}

pub const OPCODE_MAP: &[(u8, AddressingMode)] =  &[(0xe8, AddressingMode::Implied)];