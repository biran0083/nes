use crate::cpu::addressing_mode::{load_operand, AddressingMode};
use super::InstFun;

pub const RUN : InstFun = |ins, cpu| {
    let operand = load_operand(ins.mode, cpu, ins.param.unwrap());
    cpu.a = cpu.a ^ operand;
    cpu.update_z(cpu.a);
    cpu.update_n(cpu.a);
    cpu.pc += ins.len();
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
        (0x49, AddressingMode::Immediate),
        (0x45, AddressingMode::ZeroPage),
        (0x55, AddressingMode::ZeroPageX),
        (0x4D, AddressingMode::Absolute),
        (0x5D, AddressingMode::AbsoluteX),
        (0x59, AddressingMode::AbsoluteY),
        (0x41, AddressingMode::IndexedIndirect),
        (0x51, AddressingMode::IndirectIndexed),
    ];

#[cfg(test)]
mod test {
    use crate::cpu::addressing_mode::AddressingMode;
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::test_util::Register8::*;
    use crate::cpu::test_util::Flag::*;
    use crate::instructions::common::get_opcode;
    use super::OPCODE_MAP;

    #[test]
    fn test_immediate() {
        let mut runner = TestRunner::new();
        let op_code = get_opcode(OPCODE_MAP, AddressingMode::Immediate).unwrap();
        runner.set(A, 0x01);
        runner.load_and_test(&[op_code, 0x10])
            .verify(A, 0x11)
            .verify(Z, false)
            .verify(N, false);
        runner.set(A, 0x11);
        runner.load_and_test(&[op_code, 0x11])
            .verify(A, 0x00)
            .verify(Z, true)
            .verify(N, false);
        runner.set(A, 0xf0);
        runner.load_and_test(&[op_code, 0x01])
            .verify(A, 0xf1)
            .verify(Z, false)
            .verify(N, true);
    }
}