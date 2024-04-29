use crate::cpu::addressing_mode::{load_operand, AddressingMode};
use crate::cpu::CPU;
use super::InstFun;

pub const RUN : InstFun = |ins, cpu: &mut CPU| {
    cpu.a = load_operand(ins.mode, cpu, ins.param.unwrap());
    cpu.update_z(cpu.a);
    cpu.update_n(cpu.a);
    cpu.pc += ins.len();
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
        (0xA9, AddressingMode::Immediate),
        (0xA5, AddressingMode::ZeroPage),
        (0xB5, AddressingMode::ZeroPageX),
        (0xAD, AddressingMode::Absolute),
        (0xBD, AddressingMode::AbsoluteX),
        (0xB9, AddressingMode::AbsoluteY),
        (0xA1, AddressingMode::IndexedIndirect),
        (0xB1, AddressingMode::IndirectIndexed),
    ];

#[cfg(test)]
mod test {
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::test_util::Register8::*;
    use crate::cpu::test_util::Flag::*;

    #[test]
    fn test_lda_immediate() {
        let mut runner = TestRunner::new();
        runner.test(&[0xA9, 0x00])
            .verify(A, 0)
            .verify(Z, true)
            .verify(N, false);
        runner.test(&[0xA9, 0x01])
            .verify(A, 1)
            .verify(Z, false)
            .verify(N, false);
        runner.test(&[0xA9, 0x91])
            .verify(A, 0x91)
            .verify(Z, false)
            .verify(N, true);
    }

    #[test]
    fn test_lda_zero_page() {
        let mut runner = TestRunner::new();
        runner.test(&[0xA5, 0x01])
            .verify(A, 0)
            .verify(Z, true)
            .verify(N, false);
        runner.set_mem(0x01, 10);
        runner.test(&[0xA5, 0x01])
            .verify(A, 10)
            .verify(Z, false)
            .verify(N, false);
        runner.set_mem(0x01, 0xff);
        runner.test(&[0xA5, 0x01])
            .verify(A, 0xff)
            .verify(Z, false)
            .verify(N, true);
    }

    #[test]
    fn test_lda_zero_page_x() {
        let mut runner = TestRunner::new();
        runner.set_mem(0x01, 0x00);
        runner.test(&[0xB5, 0x01])
            .verify(A, 0)
            .verify(Z, true)
            .verify(N, false);
        runner.set(X, 2);

        runner.set_mem(0x03, 10);
        runner.test(&[0xB5, 0x01])
            .verify(A, 10)
            .verify(Z, false)
            .verify(N, false);

        runner.set(X, 0x80);
        runner.set_mem(0x7f, 0xff);
        runner.test(&[0xB5, 0xff])
            .verify(A, 0xff)
            .verify(Z, false)
            .verify(N, true);
    }

    #[test]
    fn test_lda_absolute() {
        let mut runner = TestRunner::new();
        runner.set_mem(0x1234, 0x11);
        runner.test(&[0xAD, 0x34, 0x12])
            .verify(A, 0x11)
            .verify(Z, false)
            .verify(N, false);
    }

    #[test]
    fn test_lda_absolute_x() {
        let mut runner = TestRunner::new();
        runner.set_mem(0x1235, 0xf0);
        runner.set(X, 1);
        runner.test(&[0xBD, 0x34, 0x12])
            .verify(A, 0xf0)
            .verify(Z, false)
            .verify(N, true);
    }

    #[test]
    fn test_lda_absolute_y() {
        let mut runner = TestRunner::new();
        runner.set_mem(0x1236, 0x13);
        runner.set(Y, 2);
        runner.test(&[0xB9, 0x34, 0x12])
            .verify(A, 0x13)
            .verify(Z, false)
            .verify(N, false);
    }

    #[test]
    fn test_lda_indexed_indirect() {
        let mut runner = TestRunner::new();
        runner.set(X, 0x11);
        runner.set_mem(0x21, 0x12);
        runner.set_mem(0x22, 0x34);
        runner.set_mem(0x3412, 0x56);
        runner.test(&[0xA1, 0x10])
            .verify(A, 0x56)
            .verify(Z, false)
            .verify(N, false);
    }

    #[test]
    fn test_lda_indirect_indexed() {
        let mut runner = TestRunner::new();
        runner.set(Y, 0x0f);
        runner.set_mem(0x10, 0x45);
        runner.set_mem(0x11, 0x23);
        runner.set_mem(0x2345, 0xff);
        runner.test(&[0xB1, 0x10])
            .verify(A, 0x0e)
            .verify(Z, false)
            .verify(N, false);
    }

}