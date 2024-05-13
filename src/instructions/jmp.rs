use crate::cpu::addressing_mode::{load_operand_addr, AddressingMode};
use super::InstFun;

pub const RUN : InstFun = |ins, cpu| {
    let operand = load_operand_addr(ins.mode, cpu, ins.param.unwrap()) as u16;
    cpu.pc = operand;
};


pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
    (0x4C, AddressingMode::Absolute),
    (0x6C, AddressingMode::Indirect)
];

#[cfg(test)]
mod tests {
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::Register16::*;

    #[test]
    fn test_absolute() {
        let mut runner = TestRunner::new();
        runner.load_and_test(&[0x4C, 0x12, 0x34])
            .verify(PC, 0x3412);
    }

    #[test]
    fn test_relative() {
        let mut runner = TestRunner::new();
        runner
            .set_mem16(0x3412, 0x5678)
            .load_and_test(&[0x6C, 0x12, 0x34])
            .verify(PC, 0x5678);
    }
}