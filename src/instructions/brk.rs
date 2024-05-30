
use crate::cpu::addressing_mode::{AddressingMode};
use super::InstFun;

pub const RUN : InstFun = |_, cpu| {
    cpu.push16(cpu.pc + 2);
    cpu.flags.set_b(true);
    cpu.push8(cpu.flags.get());
    cpu.flags.set_b(false);
    cpu.pc = cpu.get_mem16(0xFFFE);
    cpu.flags.set_i(true);
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[(0x00, AddressingMode::Implied)];

#[cfg(test)]
mod tests {
    use crate::cpu::Stack;
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::Register16::*;
    use crate::cpu::Flag::*;
    use crate::cpu::Flags;

    #[test]
    fn test_brk() {
        let mut runner = TestRunner::new();
        let old_flag = runner.get(Flags{}) | 0x10;
        runner.set_mem16(0xfffe, 0x1234)
            .load_and_test(&[0x00])
            .verify(B, false)
            .verify(PC, 0x1234)
            .verify(Stack::new(1), old_flag)
            .verify(Stack::new(2), 0x02)
            .verify(Stack::new(3), 0x80);
    }
}