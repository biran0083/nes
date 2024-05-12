use crate::cpu::addressing_mode::{load_operand_addr, AddressingMode};
use super::InstFun;

pub const RUN : InstFun = |ins, cpu| {
    let operand = load_operand_addr(ins.mode, cpu, ins.param.unwrap()) as u16;
    let return_addr = cpu.pc + ins.len() as u16;
    cpu.push16(return_addr - 1);
    cpu.pc = operand;
};


pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
    (0x20, AddressingMode::Absolute),
];

#[cfg(test)]
mod tests {
    use crate::cpu::test_util::Stack;
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::test_util::Register16::*;

    #[test]
    fn test_absolute() {
        let mut runner = TestRunner::new();
        runner.test(&[0x20, 0x12, 0x34])
            .verify(PC, 0x3412)
            .verify(Stack::new(1), 0x02)
            .verify(Stack::new(2), 0x80);
    }
}