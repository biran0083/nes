use crate::cpu::{addressing_mode::{load_operand_addr, AddressingMode}, Mem, Register8, Retriever, Setter, CPU};
use super::{Inst, InstFun};

fn helper<T>(ins: &Inst, cpu: &mut CPU, t: T)
        where T: Retriever<u8> + Setter<u8> {
    let c = if cpu.flags.c() { 1 } else { 0 };
    let old_value = t.get(cpu);
    let value = old_value >> 1 | c << 7;
    cpu.flags.set_c((old_value & 0x01) != 0);
    t.set(cpu, value);

    cpu.flags.set_z(value == 0);
    cpu.flags.set_n(value & 0x80 != 0);
    cpu.pc += ins.len();
}

pub const RUN : InstFun = |ins, cpu| {
    if ins.mode == AddressingMode::Accumulator {
        helper(ins, cpu, Register8::A)
    } else {
        let addr = load_operand_addr(ins.mode, cpu, ins.param.unwrap());
        helper(ins, cpu, Mem::new(addr))
    };
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
    (0x6A, AddressingMode::Accumulator),
    (0x66, AddressingMode::ZeroPage),
    (0x76, AddressingMode::ZeroPageX),
    (0x6E, AddressingMode::Absolute),
    (0x7E, AddressingMode::AbsoluteX),
];

#[cfg(test)]
mod test {
    use crate::cpu::Mem;
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::Flag::*;
    use crate::cpu::Register8::*;

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