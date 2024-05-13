use crate::cpu::addressing_mode::{AddressingMode};
use super::InstFun;

pub const RUN : InstFun = |ins, cpu| {
    cpu.y = cpu.y.wrapping_add(1);
    cpu.update_z(cpu.y);
    cpu.update_n(cpu.y);
    cpu.pc += ins.len();
};
pub const OPCODE_MAP: &[(u8, AddressingMode)] =  &[(0xC8, AddressingMode::Implied)];

#[cfg(test)]
mod test {
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::test_util::Register8::*;
    use crate::cpu::test_util::Flag::*;

    #[test]
    fn test_inx() {
        let mut runner = TestRunner::new();
        runner.load_and_test(&[0xC8])
            .verify(Y, 1)
            .verify(Z, false)
            .verify(N, false);

        runner.set(Y, 0xff);
        runner.load_and_test(&[0xC8])
            .verify(Y, 0)
            .verify(Z, true)
            .verify(N, false);
        runner.set(Y, 0x7f);
        runner.load_and_test(&[0xC8])
            .verify(Y, 0x80)
            .verify(Z, false)
            .verify(N, true);
    }
}