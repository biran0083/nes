use super::InstFun;
use crate::cpu::addressing_mode::{load_operand, AddressingMode};

pub const RUN: InstFun = |ins, cpu| {
    let operand = load_operand(ins.mode, cpu, ins.param.unwrap());
    cpu.a = cpu.a & operand;
    cpu.update_z(cpu.a);
    cpu.update_n(cpu.a);
    cpu.flags.set_c(cpu.a & 0x80 != 0);
    cpu.pc += ins.len();
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
    (0x0B, AddressingMode::Immediate),
    (0x2B, AddressingMode::Immediate),
];

#[cfg(test)]
mod test {
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::Flag::*;
    use crate::cpu::Register8::*;

    #[test]
    fn test_immediate() {
        let mut runner = TestRunner::new();
        runner.set(A, 0x81);
        runner
            .load_and_test(&[0x0B, 0xff])
            .verify(A, 0x81)
            .verify(Z, false)
            .verify(N, true)
            .verify(C, true);
        runner.set(A, 0xf0);
        runner
            .load_and_test(&[0x0B, 0x0f])
            .verify(A, 0x00)
            .verify(Z, true)
            .verify(N, false)
            .verify(C, false);
    }
}
