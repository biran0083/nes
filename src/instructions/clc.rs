
use crate::cpu::addressing_mode::{AddressingMode};
use super::InstFun;

pub const RUN : InstFun = |ins, cpu| {
    cpu.flags.set_c(false);
    cpu.pc += ins.len();
};
pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[(0x18, AddressingMode::Implied)];

#[cfg(test)]
mod tests {
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::test_util::Flag::*;

    #[test]
    fn test_brk() {
        let mut runner = TestRunner::new();
        runner.set(C, true)
            .test(&[0x18])
            .verify(C, false);
        runner.set(C, false)
            .test(&[0x18])
            .verify(C, false);
    }
}