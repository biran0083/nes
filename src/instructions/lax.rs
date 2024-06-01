use crate::cpu::addressing_mode::{load_operand, AddressingMode};
use crate::cpu::CPU;
use super::InstFun;
use crate::cpu::Register8::*;
use crate::cpu::Setter;

pub const RUN : InstFun = |ins, cpu: &mut CPU| {
    let value = load_operand(ins.mode, cpu, ins.param.unwrap());
    A.set(cpu, value);
    X.set(cpu, value);
    cpu.update_z(value);
    cpu.update_n(value);
    cpu.pc += ins.len();
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
    (0xA7, AddressingMode::ZeroPage),
    (0xB7, AddressingMode::ZeroPageY),
    (0xAF, AddressingMode::Absolute),
    (0xBF, AddressingMode::AbsoluteY),
    (0xA3, AddressingMode::IndexedIndirect),
    (0xB3, AddressingMode::IndirectIndexed),

];

#[cfg(test)]
mod test {
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::Register8::*;
    use crate::cpu::Flag::*;
    use super::OPCODE_MAP;
    use crate::cpu::addressing_mode::AddressingMode;
    use crate::instructions::common::get_opcode;


    #[test]
    fn test_zero_page() {
        let mut runner = TestRunner::new();
        let opcode = get_opcode(OPCODE_MAP, AddressingMode::ZeroPage).unwrap();
        runner.load_and_test(&[opcode, 0x01])
            .verify(A, 0)
            .verify(X, 0)
            .verify(Z, true)
            .verify(N, false);
        runner.set_mem(0x01, 10);
        runner.load_and_test(&[opcode, 0x01])
            .verify(A, 10)
            .verify(X, 10)
            .verify(Z, false)
            .verify(N, false);
        runner.set_mem(0x01, 0xff);
        runner.load_and_test(&[opcode, 0x01])
            .verify(A, 0xff)
            .verify(X, 0xff)
            .verify(Z, false)
            .verify(N, true);
    }
}