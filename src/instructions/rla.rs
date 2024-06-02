use crate::cpu::addressing_mode::{load_operand_addr, AddressingMode};
use super::InstFun;


pub const RUN : InstFun = |ins, cpu| {
    let addr = load_operand_addr(ins.mode, cpu, ins.param.unwrap());
    let c = if cpu.flags.c() { 1 } else { 0 };
    let old_value = cpu.get_mem(addr);
    let value = old_value << 1 | c;
    cpu.flags.set_c((old_value & 0x80) != 0);
    cpu.set_mem(addr, value);
    cpu.a = cpu.a & value;
    cpu.update_z(cpu.a);
    cpu.update_n(cpu.a);
    cpu.pc += ins.len();
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
    (0x27, AddressingMode::ZeroPage),
    (0x37, AddressingMode::ZeroPageX),
    (0x2F, AddressingMode::Absolute),
    (0x3F, AddressingMode::AbsoluteX),
    (0x3B, AddressingMode::AbsoluteY),
    (0x23, AddressingMode::IndexedIndirect),
    (0x33, AddressingMode::IndirectIndexed),
];

#[cfg(test)]
mod test {
    use crate::cpu::Mem;
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::Flag::*;
    use crate::cpu::Register8::*;

    #[test]
    fn test_zero_page() {
        let mut runner = TestRunner::new();
        runner.set(C, true);
        runner.set(A, 0x31);
        runner.set(Mem::new(0x12), 0x01);
        runner.load_and_test(&[0x27, 0x12])
            .verify(Mem::new(0x12), 0x03)
            .verify(A, 0x01)
            .verify(C, false)
            .verify(N, false)
            .verify(Z, false);
    }
}