use crate::cpu::addressing_mode::{load_operand_opt, AddressingMode};
use crate::cpu::CPU;
use super::InstFun;


pub const RUN : InstFun = |ins, cpu: &mut CPU| {
    let m = load_operand_opt(ins.mode, cpu, ins.param);
    let res: u16 = (m as u16) << 1;
    cpu.a = res as u8;
    cpu.update_z(cpu.a);
    cpu.update_n(cpu.a);
    cpu.flags.set_c(res & 0x100 != 0);
    cpu.pc += ins.len();
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
        (0x0A, AddressingMode::Accumulator),
        (0x06, AddressingMode::ZeroPage),
        (0x16, AddressingMode::ZeroPageX),
        (0x0E, AddressingMode::Absolute),
        (0x1E, AddressingMode::AbsoluteX),
    ];


#[cfg(test)]
mod test {
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::test_util::Register8::*;
    use crate::cpu::test_util::Flag::*;

    #[test]
    fn test_accumulator() {
        let mut runner = TestRunner::new();
        runner.set(A, 0x01);
        runner.test(&[0x0A])
            .verify(A, 2)
            .verify(C, false)
            .verify(Z, false)
            .verify(N, false);
        runner.set(A, 0x80);
        runner.test(&[0x0A])
            .verify(A, 0)
            .verify(C, true)
            .verify(Z, true)
            .verify(N, false);
        runner.set(A, 0x40);
        runner.test(&[0x0A])
            .verify(A, 0x80)
            .verify(C, false)
            .verify(Z, false)
            .verify(N, true);
    }
}