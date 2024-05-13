use crate::cpu::addressing_mode::{load_operand, AddressingMode};
use super::InstFun;

pub const RUN : InstFun = |ins, cpu| {
    let operand = load_operand(ins.mode, cpu, ins.param.unwrap());
    cpu.a = cpu.a & operand;
    cpu.update_z(cpu.a);
    cpu.update_n(cpu.a);
    cpu.pc += ins.len();
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
        (0x29, AddressingMode::Immediate),
        (0x25, AddressingMode::ZeroPage),
        (0x35, AddressingMode::ZeroPageX),
        (0x2D, AddressingMode::Absolute),
        (0x3D, AddressingMode::AbsoluteX),
        (0x39, AddressingMode::AbsoluteY),
        (0x21, AddressingMode::IndexedIndirect),
        (0x31, AddressingMode::IndirectIndexed),
    ];

#[cfg(test)]
mod test {
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::test_util::Register8::*;
    use crate::cpu::test_util::Flag::*;

    #[test]
    fn test_immediate() {
        let mut runner = TestRunner::new();
        runner.set(A, 0x01);
        runner.load_and_test(&[0x29, 0x10])
            .verify(A, 0x00)
            .verify(Z, true)
            .verify(N, false);
        runner.set(A, 0xff);
        runner.load_and_test(&[0x29, 0x00])
            .verify(A, 0x00)
            .verify(Z, true)
            .verify(N, false);
        runner.set(A, 0xf0);
        runner.load_and_test(&[0x29, 0xf1])
            .verify(A, 0xf0)
            .verify(Z, false)
            .verify(N, true);
    }
}