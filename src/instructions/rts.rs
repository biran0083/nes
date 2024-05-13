use crate::cpu::addressing_mode::{load_operand, AddressingMode};
use super::InstFun;

pub const RUN : InstFun = |ins, cpu| {
    let pc = cpu.pop16();
    cpu.pc = pc + 1;
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
        (0x60, AddressingMode::Immediate),
    ];

#[cfg(test)]
mod test {
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::Register16::*;

    #[test]
    fn test_implied() {
        let mut runner = TestRunner::new();
        runner.load_program(&[0x60]);
        runner.push(0x12);
        runner.push(0x34);
        runner.test()
            .verify(PC, 0x1235);
    }
}