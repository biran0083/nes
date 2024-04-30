use crate::cpu::addressing_mode::{load_operand, AddressingMode};
use crate::cpu::CPU;
use super::InstFun;

pub const RUN : InstFun = |ins, cpu: &mut CPU| {
    let m = load_operand(ins.mode, cpu, ins.param.unwrap());
    let res = cpu.a & m;
    cpu.flags.set_z(res == 0);
    cpu.flags.set_v((m & 0x40) != 0);
    cpu.flags.set_n((m & 0x80) != 0);
    cpu.pc = ins.len();
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
        (0x24, AddressingMode::ZeroPage),
        (0x2C, AddressingMode::Absolute),
    ];


#[cfg(test)]
mod test {
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::test_util::Register8::*;
    use crate::cpu::test_util::Flag::*;

    #[test]
    fn test_absolute() {
        let mut runner = TestRunner::new();
        runner.set(A, 0x80);
        runner.set_mem(0x0000, 0x00);
        runner.test(&[0x2C, 0x00, 0x00])
            .verify(Z, true)
            .verify(V, false)
            .verify(N, false);
        runner.set(A, 0x80);
        runner.set_mem(0x1234, 0xc0);
        runner.test(&[0x2C, 0x34, 0x12])
            .verify(Z, false)
            .verify(V, true)
            .verify(N, true);
    }

    #[test]
    fn test_zero_page() {
        let mut runner = TestRunner::new();
        runner.set(A, 0x80);
        runner.set_mem(0x00, 0x00);
        runner.test(&[0x24, 0x00])
            .verify(Z, true)
            .verify(V, false)
            .verify(N, false);
        runner.set(A, 0x80);
        runner.set_mem(0x01, 0xc0);
        runner.test(&[0x24, 0x01])
            .verify(Z, false)
            .verify(V, true)
            .verify(N, true);
    }
}