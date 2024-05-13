use crate::cpu::addressing_mode::{load_operand_addr, AddressingMode};
use super::InstFun;

pub const RUN : InstFun = |ins, cpu| {
    let c = if cpu.flags.c() { 1 } else { 0 };
    let value = if ins.mode == AddressingMode::Accumulator {
        let value = cpu.a << 1 | c;
        cpu.flags.set_c((cpu.a & 0x80) != 0);
        cpu.a = value;
        value
    } else {
        let addr = load_operand_addr(ins.mode, cpu, ins.param.unwrap());
        let old_value = cpu.get_mem(addr);
        let value = old_value << 1 | c;
        cpu.flags.set_c((old_value & 0x80) != 0);
        cpu.set_mem(addr, value);
        value
    };
    cpu.flags.set_z(value == 0);
    cpu.flags.set_n(value & 0x80 != 0);
    cpu.pc += ins.len();
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
    (0x2A, AddressingMode::Accumulator),
    (0x26, AddressingMode::ZeroPage),
    (0x36, AddressingMode::ZeroPageX),
    (0x2E, AddressingMode::Absolute),
    (0x3E, AddressingMode::AbsoluteX),
];

#[cfg(test)]
mod test {
    use crate::cpu::test_util::Mem;
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::test_util::Flag::*;
    use crate::cpu::test_util::Register8::*;

    #[test]
    fn test_accumulate() {
        let mut runner = TestRunner::new();
        runner.set(C, true);
        runner.set(A, 0x01);
        runner.load_and_test(&[0x2A])
            .verify(A, 0x03)
            .verify(C, false)
            .verify(N, false)
            .verify(Z, false);

        runner.set(C, false);
        runner.set(A, 0x80);
        runner.load_and_test(&[0x2A])
            .verify(A, 0)
            .verify(C, true)
            .verify(N, false)
            .verify(Z, true);

        runner.set(C, true);
        runner.set(A, 0x7f);
        runner.load_and_test(&[0x2A])
            .verify(A, 0xff)
            .verify(C, false)
            .verify(N, true)
            .verify(Z, false);
    }

    #[test]
    fn test_zero_page() {
        let mut runner = TestRunner::new();
        runner.set(C, true);
        runner.set(Mem::new(0x12), 0x01);
        runner.load_and_test(&[0x26, 0x12])
            .verify(Mem::new(0x12), 0x03)
            .verify(C, false)
            .verify(N, false)
            .verify(Z, false);

        runner.set(C, false);
        runner.set(Mem::new(0x12), 0x80);
        runner.load_and_test(&[0x26, 0x12])
            .verify(Mem::new(0x12), 0)
            .verify(C, true)
            .verify(N, false)
            .verify(Z, true);

        runner.set(C, true);
        runner.set(Mem::new(0x12), 0x7f);
        runner.load_and_test(&[0x26, 0x12])
            .verify(Mem::new(0x12), 0xff)
            .verify(C, false)
            .verify(N, true)
            .verify(Z, false);
    }
}