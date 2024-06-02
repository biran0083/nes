use super::{common::adc_helper, InstFun};
use crate::cpu::addressing_mode::{load_operand_addr, AddressingMode};

pub const RUN: InstFun = |ins, cpu| {
    let addr = load_operand_addr(ins.mode, cpu, ins.param.unwrap());
    let c = if cpu.flags.c() { 1 } else { 0 };
    let old_value = cpu.get_mem(addr);
    let value = old_value >> 1 | c << 7;
    cpu.flags.set_c((old_value & 0x01) != 0);
    cpu.set_mem(addr, value);
    adc_helper(value, cpu);
    cpu.pc += ins.len();
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
    (0x67, AddressingMode::ZeroPage),
    (0x77, AddressingMode::ZeroPageX),
    (0x6F, AddressingMode::Absolute),
    (0x7F, AddressingMode::AbsoluteX),
    (0x7B, AddressingMode::AbsoluteY),
    (0x63, AddressingMode::IndexedIndirect),
    (0x73, AddressingMode::IndirectIndexed),
];

#[cfg(test)]
mod test {
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::Flag::*;
    use crate::cpu::Mem;
    use crate::cpu::Register8::*;

    #[test]
    fn test_zero_page() {
        let mut runner = TestRunner::new();
        runner.set(C, true);
        runner.set(A, 0x10);
        runner.set(Mem::new(0x12), 0x02);
        runner
            .load_and_test(&[0x67, 0x12])
            .verify(Mem::new(0x12), 0x81)
            .verify(A, 0x91)
            .verify(C, false)
            .verify(N, true)
            .verify(Z, false);
    }
}
