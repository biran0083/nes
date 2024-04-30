use crate::cpu::{addressing_mode::{load_operand, AddressingMode}, test_util::{Flag, Setter}};
use super::InstFun;

pub const RUN : InstFun = |ins, cpu| {
    let operand = load_operand(ins.mode, cpu, ins.param.unwrap());
    let res = cpu.x.wrapping_sub(operand);
    Flag::C.set(cpu, cpu.x >= operand);
    Flag::Z.set(cpu, cpu.x == operand);
    Flag::N.set(cpu, res & 0x80 != 0);
    cpu.pc += ins.len();
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
        (0xE0, AddressingMode::Immediate),
        (0xE4, AddressingMode::ZeroPage),
        (0xEC, AddressingMode::Absolute),
    ];

#[cfg(test)]
mod test {
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::test_util::Register8::*;
    use crate::cpu::test_util::Flag::*;

    #[test]
    fn test_immediate() {
        let mut runner = TestRunner::new();
        runner.set(X, 0x01);
        runner.test(&[0xE0, 0x01])
            .verify(C, true)
            .verify(Z, true)
            .verify(N, false);
        runner.set(X, 0xff);
        runner.test(&[0xE0, 0x00])
            .verify(C, true)
            .verify(Z, false)
            .verify(N, true);
        runner.set(X, 0x03);
        runner.test(&[0xE0, 0x02])
            .verify(C, true)
            .verify(Z, false)
            .verify(N, false);
        runner.set(X, 0x02);
        runner.test(&[0xE0, 0x03])
            .verify(C, false)
            .verify(Z, false)
            .verify(N, true);
    }
}