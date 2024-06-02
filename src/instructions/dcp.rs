use crate::cpu::{addressing_mode::{load_operand_addr, AddressingMode}, Flag, Retriever, Setter};
use super::InstFun;
use crate::cpu::Register8::*;

pub const RUN : InstFun = |ins, cpu| {
    let addr = load_operand_addr(ins.mode, cpu, ins.param.unwrap());
    let value = cpu.get_mem(addr).wrapping_sub(1);
    cpu.set_mem(addr, value);
    let res = A.get(&cpu).wrapping_sub(value);
    Flag::C.set(cpu, A.get(&cpu) >= value);
    Flag::Z.set(cpu, A.get(&cpu) == value);
    Flag::N.set(cpu, res & 0x80 != 0);
    cpu.pc += ins.len();
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
        (0xC7, AddressingMode::ZeroPage),
        (0xD7, AddressingMode::ZeroPageX),
        (0xCF, AddressingMode::Absolute),
        (0xDF, AddressingMode::AbsoluteX),
        (0xDB, AddressingMode::AbsoluteY),
        (0xC3, AddressingMode::IndexedIndirect),
        (0xD3, AddressingMode::IndirectIndexed),
    ];

#[cfg(test)]
mod test {
    use crate::cpu::addressing_mode::AddressingMode;
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::Mem;
    use crate::cpu::Flag::*;
    use crate::cpu::Register8::*;
    use crate::instructions::common::get_opcode;
    use super::OPCODE_MAP;

    #[test]
    fn test_zero_page() {
        let mut runner = TestRunner::new();
        let op_code = get_opcode(OPCODE_MAP, AddressingMode::ZeroPage).unwrap();
        runner.set_mem(0x10, 0x23);
        runner.set(A, 0x22);
        runner.load_and_test(&[op_code, 0x10])
            .verify(Mem::new(0x10), 0x22)
            .verify(C, true)
            .verify(Z, true)
            .verify(N, false);
    }
}