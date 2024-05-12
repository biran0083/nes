use crate::cpu::addressing_mode::{load_operand, AddressingMode};
use super::InstFun;

pub const RUN : InstFun = |ins, cpu| {
    let operand = load_operand(ins.mode, cpu, ins.param.unwrap());
    cpu.a = cpu.a | operand;
    cpu.update_z(cpu.a);
    cpu.update_n(cpu.a);
    cpu.pc += ins.len();
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
        (0x09, AddressingMode::Immediate),
        (0x05, AddressingMode::ZeroPage),
        (0x15, AddressingMode::ZeroPageX),
        (0x0D, AddressingMode::Absolute),
        (0x1D, AddressingMode::AbsoluteX),
        (0x19, AddressingMode::AbsoluteY),
        (0x01, AddressingMode::IndexedIndirect),
        (0x11, AddressingMode::IndirectIndexed),
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
        runner.test(&[0x09, 0x10])
            .verify(A, 0x11)
            .verify(Z, false)
            .verify(N, false);
        runner.set(A, 0xff);
        runner.test(&[0x09, 0x00])
            .verify(A, 0xff)
            .verify(Z, false)
            .verify(N, true);
        runner.set(A, 0);
        runner.test(&[0x09, 0])
            .verify(A, 0)
            .verify(Z, true)
            .verify(N, false);
    }
}