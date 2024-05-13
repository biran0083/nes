use crate::cpu::addressing_mode::AddressingMode;
use super::InstFun;

pub const RUN : InstFun = |ins, cpu| {
    cpu.push8(cpu.flags.get());
    cpu.pc += ins.len();
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
        (0x08, AddressingMode::Implied),
    ];

#[cfg(test)]
mod test {
    use crate::cpu::Flags;
    use crate::cpu::Stack;
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::Register8::*;
    use crate::cpu::Flag::*;

    #[test]
    fn test() {
        let mut runner = TestRunner::new();
        runner.set(C, true);
        runner.set(N, true);
        let value = runner.get(Flags{});

        runner.load_and_test(&[0x08])
            .verify(Stack::new(1), value)
            .verify(SP, 0xfe);
    }
}