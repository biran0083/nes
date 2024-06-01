use crate::cpu::addressing_mode::{load_operand, AddressingMode};
use crate::cpu::CPU;
use super::InstFun;
use crate::cpu::Register8::*;
use crate::cpu::Setter;
use crate::cpu::Retriever;

pub const RUN : InstFun = |ins, cpu: &mut CPU| {
    let value = load_operand(ins.mode, cpu, ins.param.unwrap()).wrapping_add(SP.get(cpu));
    A.set(cpu, value);
    X.set(cpu, value);
    SP.set(cpu, value);
    cpu.update_z(value);
    cpu.update_n(value);
    cpu.pc += ins.len();
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
    (0xBB, AddressingMode::AbsoluteY),
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
        let opcode = get_opcode(OPCODE_MAP, AddressingMode::AbsoluteY).unwrap();
        runner.load_program(&[opcode, 0x01, 0x00]);
        runner.set(Y, 0x02);
        runner.set_mem(0x03, 0x34);
        runner.set(SP, 0x12);
        runner.test()
            .verify(A, 0x46)
            .verify(X, 0x46)
            .verify(Z, false)
            .verify(N, false);
    }
}