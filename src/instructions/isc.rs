use crate::cpu::addressing_mode::{load_operand_addr, AddressingMode};
use super::InstFun;
use crate::instructions::common::adc_helper;

pub const RUN : InstFun = |ins, cpu| {
    let addr = load_operand_addr(ins.mode, cpu, ins.param.unwrap());
    let value = cpu.get_mem(addr).wrapping_add(1);
    cpu.set_mem(addr, value);
    adc_helper(!value, cpu);
    cpu.pc += ins.len();
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
        (0xE7, AddressingMode::ZeroPage),
        (0xF7, AddressingMode::ZeroPageX),
        (0xEF, AddressingMode::Absolute),
        (0xFF, AddressingMode::AbsoluteX),
        (0xFB, AddressingMode::AbsoluteY),
        (0xE3, AddressingMode::IndexedIndirect),
        (0xF3, AddressingMode::IndirectIndexed),
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
        runner.set(C, false);
        runner.set(A, 0x25);
        runner.load_and_test(&[op_code, 0x10])
            .verify(Mem::new(0x10), 0x24)
            .verify(A, 0x00)
            .verify(Z, true)
            .verify(N, false);

        runner.set_mem(0x10, 0x23);
        runner.set(C, true);
        runner.set(A, 0x25);
        runner.load_and_test(&[op_code, 0x10])
            .verify(Mem::new(0x10), 0x24)
            .verify(A, 0x01)
            .verify(Z, false)
            .verify(N, false);
    }
}