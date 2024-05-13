use crate::cpu::addressing_mode::AddressingMode;
use super::InstFun;

pub const RUN : InstFun = |ins, cpu| {
    cpu.push8(cpu.a);
    cpu.pc += ins.len();
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
        (0x48, AddressingMode::Implied),
    ];

#[cfg(test)]
mod test {
    use crate::cpu::Stack;
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::Register8::*;

    #[test]
    fn test() {
        let mut runner = TestRunner::new();
        runner.set(A, 0x23);
        runner.load_and_test(&[0x48])
            .verify(Stack::new(1), 0x23)
            .verify(SP, 0xfe);
    }
}