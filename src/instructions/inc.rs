use crate::cpu::addressing_mode::{load_operand_addr, AddressingMode};
use super::InstFun;

pub const RUN : InstFun = |ins, cpu| {
    let addr = load_operand_addr(ins.mode, cpu, ins.param.unwrap());
    let value = cpu.get_mem(addr).wrapping_add(1);
    cpu.set_mem(addr, value);
    cpu.update_z(value);
    cpu.update_n(value);
    cpu.pc += ins.len();
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
        (0xE6, AddressingMode::ZeroPage),
        (0xF6, AddressingMode::ZeroPageX),
        (0xEE, AddressingMode::Absolute),
        (0xFE, AddressingMode::AbsoluteX),
    ];

#[cfg(test)]
mod test {
    use crate::cpu::addressing_mode::AddressingMode;
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::test_util::Mem;
    use crate::cpu::test_util::Flag::*;
    use crate::instructions::common::get_opcode;
    use super::OPCODE_MAP;

    #[test]
    fn test_zero_page() {
        let mut runner = TestRunner::new();
        let op_code = get_opcode(OPCODE_MAP, AddressingMode::ZeroPage).unwrap();
        runner.set_mem(0x10, 0x23);
        runner.load_and_test(&[op_code, 0x10])
            .verify(Mem::new(0x10), 0x24)
            .verify(Z, false)
            .verify(N, false);
        runner.set_mem(0x11, 0xff);
        runner.load_and_test(&[op_code, 0x11])
            .verify(Mem::new(0x11), 0)
            .verify(Z, true)
            .verify(N, false);
        runner.set_mem(0x12, 0x7f);
        runner.load_and_test(&[op_code, 0x12])
            .verify(Mem::new(0x12), 0x80)
            .verify(Z, false)
            .verify(N, true);
    }
}