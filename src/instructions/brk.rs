
use crate::cpu::addressing_mode::{AddressingMode};
use super::InstFun;

pub const RUN : InstFun = |_, cpu| {
    cpu.push16(cpu.pc);
    cpu.push8(cpu.flags.get());
    cpu.flags.set_b(true);
    cpu.pc = cpu.get_mem16(0xFFFE);

};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[(0x00, AddressingMode::Implied)];

#[cfg(test)]
mod tests {
    use crate::cpu::test_util::Stack;
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::test_util::Register16::*;
    use crate::cpu::test_util::Flag::*;
    use crate::cpu::test_util::Flags;

    #[test]
    fn test_brk() {
        let mut runner = TestRunner::new();
        let old_flag = runner.get(Flags{});
        runner.set_mem16(0xfffe, 0x1234)
            .test(&[0x00])
            .verify(B, true)
            .verify(PC, 0x1234)
            .verify(Stack::new(1), old_flag)
            .verify(Stack::new(2), 0x00)
            .verify(Stack::new(3), 0x80);
    }
}