use crate::cpu::addressing_mode::{load_operand_addr, AddressingMode};
use super::InstFun;

pub const RUN : InstFun = |ins, cpu| {
    let value = cpu.y.wrapping_sub(1);
    cpu.y = value;
    cpu.update_z(value);
    cpu.update_n(value);
    cpu.pc += ins.len();
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
        (0x88, AddressingMode::Implied),
    ];

#[cfg(test)]
mod test {
    use crate::cpu::addressing_mode::AddressingMode;
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::test_util::Register8::*;
    use crate::cpu::test_util::Flag::*;
    use crate::instructions::common::get_opcode;
    use super::OPCODE_MAP;

    #[test]
    fn test() {
        let mut runner = TestRunner::new();
        let op_code = get_opcode(OPCODE_MAP, AddressingMode::Implied);
        runner.set(Y, 0x23);
        runner.test(&[op_code])
            .verify(Y, 0x22)
            .verify(Z, false)
            .verify(N, false);
        runner.set(Y, 0x1);
        runner.test(&[op_code])
            .verify(X, 0)
            .verify(Z, true)
            .verify(N, false);
        runner.set(Y,0);
        runner.test(&[op_code])
            .verify(Y, 0xff)
            .verify(Z, false)
            .verify(N, true);
    }
}