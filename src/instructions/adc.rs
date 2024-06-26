use crate::cpu::addressing_mode::{load_operand, AddressingMode};
use super::{common::adc_helper, InstFun};


pub const RUN : InstFun = |ins, cpu| {
    let operand = load_operand(ins.mode, cpu, ins.param.unwrap());
    adc_helper(operand, cpu);
    cpu.pc += ins.len();
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
        (0x69, AddressingMode::Immediate),
        (0x65, AddressingMode::ZeroPage),
        (0x75, AddressingMode::ZeroPageX),
        (0x6D, AddressingMode::Absolute),
        (0x7D, AddressingMode::AbsoluteX),
        (0x79, AddressingMode::AbsoluteY),
        (0x61, AddressingMode::IndexedIndirect),
        (0x71, AddressingMode::IndirectIndexed),
    ];

#[cfg(test)]
mod test {
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::Register8::*;
    use crate::cpu::Flag::*;


    #[test]
    fn test_adc_immediate() {
        let mut runner = TestRunner::new();
        runner.set(A, 0x01);
        runner.load_and_test(&[0x69, 0x01])
            .verify(A, 0x02)
            .verify(C, false)
            .verify(Z, false)
            .verify(V, false)
            .verify(N, false);
        runner.set(A, 0x7f);
        runner.load_and_test(&[0x69, 0x01])
            .verify(A, 0x80)
            .verify(C, false)
            .verify(Z, false)
            .verify(V, true)
            .verify(N, true);
        runner.set(A, 0x80);
        runner.load_and_test(&[0x69, 0x01])
            .verify(A, 0x81)
            .verify(C, false)
            .verify(Z, false)
            .verify(V, false)
            .verify(N, true);
        runner.set(A, 0xff);
        runner.load_and_test(&[0x69, 0x01])
            .verify(A, 0x00)
            .verify(C, true)
            .verify(Z, true)
            .verify(V, false)
            .verify(N, false);
        runner.set(A, 0xff);
        runner.set(C, true);
        runner.load_and_test(&[0x69, 0xff])
            .verify(A, 0xff)
            .verify(C, true)
            .verify(Z, false)
            .verify(V, false)
            .verify(N, true);
    }

    #[test]
    fn test_adc_zero_page() {
        let mut runner = TestRunner::new();
        runner.set_mem(0x01, 0x01);
        runner.set(A, 0x01);
        runner.load_and_test(&[0x65, 0x01])
            .verify(A, 0x02)
            .verify(C, false)
            .verify(Z, false)
            .verify(V, false)
            .verify(N, false);
        runner.set_mem(0x01, 0x7f);
        runner.set(A, 0x7f);
        runner.load_and_test(&[0x65, 0x01])
            .verify(A, 0xfe)
            .verify(C, false)
            .verify(Z, false)
            .verify(V, true)
            .verify(N, true);
        runner.set_mem(0x01, 0x80);
        runner.set(A, 0x80);
        runner.load_and_test(&[0x65, 0x01])
            .verify(A, 0)
            .verify(C, true)
            .verify(Z, true)
            .verify(V, true)
            .verify(N, false);
        runner.set(C, true);
        runner.set_mem(0x01, 0xff);
        runner.set(A, 0xff);
        runner.load_and_test(&[0x65, 0x01])
            .verify(A, 0xff)
            .verify(C, true)
            .verify(Z, false)
            .verify(V, false)
            .verify(N, true);
        runner.set_mem(0x01, 0xff);
        runner.set(A, 0xff);
        runner.set(C, true);
        runner.load_and_test(&[0x65, 0x01])
            .verify(A, 0xff)
            .verify(C, true)
            .verify(Z, false)
            .verify(V, false)
            .verify(N, true);
    }

}