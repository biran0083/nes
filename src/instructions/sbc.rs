use crate::cpu::addressing_mode::{load_operand, AddressingMode};
use super::{common::adc_helper, InstFun};


pub const RUN : InstFun = |ins, cpu| {
    let operand = load_operand(ins.mode, cpu, ins.param.unwrap());
    adc_helper(cpu.a, !operand, cpu);
    cpu.pc += ins.len();
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
        (0xE9, AddressingMode::Immediate),
        (0xE5, AddressingMode::ZeroPage),
        (0xF5, AddressingMode::ZeroPageX),
        (0xED, AddressingMode::Absolute),
        (0xFD, AddressingMode::AbsoluteX),
        (0xF9, AddressingMode::AbsoluteY),
        (0xE1, AddressingMode::IndexedIndirect),
        (0xF1, AddressingMode::IndirectIndexed),
    ];

#[cfg(test)]
mod test {
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::Register8::*;
    use crate::cpu::Flag::*;


    #[test]
    fn test_immediate() {
        let mut runner = TestRunner::new();
        runner.set(A, 0x01);
        runner.set(C, true);
        runner.load_and_test(&[0xE9, 0x01])
            .verify(A, 0)
            .verify(C, true)
            .verify(Z, true)
            .verify(V, false)
            .verify(N, false);
        runner.set(A, 80);
        runner.set(C, true);
        runner.load_and_test(&[0xE9, 240]) // -16
            .verify(A, 96)
            .verify(C, false)
            .verify(Z, false)
            .verify(V, false)
            .verify(N, false);
        runner.set(A, 80);
        runner.set(C, true);
        runner.load_and_test(&[0xE9, 176]) // -80
            .verify(A, 160)
            .verify(C, false)
            .verify(Z, false)
            .verify(V, true)
            .verify(N, true);
        runner.set(A, 80);
        runner.set(C, true);
        runner.load_and_test(&[0xE9, 112])
            .verify(A, 224)
            .verify(C, false)
            .verify(Z, false)
            .verify(V, false)
            .verify(N, true);
        runner.set(A, 80);
        runner.set(C, true);
        runner.load_and_test(&[0xE9, 48])
            .verify(A, 32)
            .verify(C, true)
            .verify(Z, false)
            .verify(V, false)
            .verify(N, false);
        runner.set(A, 208);
        runner.set(C, true);
        runner.load_and_test(&[0xE9, 112])
            .verify(A, 96)
            .verify(C, true)
            .verify(Z, false)
            .verify(V, true)
            .verify(N, false);

        runner.set(A, 0x01);
        runner.set(C, false);
        runner.load_and_test(&[0xE9, 0x01])
            .verify(A, 0xff)
            .verify(C, false)
            .verify(Z, false)
            .verify(V, false)
            .verify(N, true);
    }
}