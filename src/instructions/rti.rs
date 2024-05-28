use crate::cpu::addressing_mode::AddressingMode;
use super::InstFun;

pub const RUN : InstFun = |_, cpu| {
    let flags = cpu.pop8();
    let pc = cpu.pop16();
    cpu.flags.set(flags);
    cpu.pc = pc;
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
        (0x40, AddressingMode::Implied),
    ];

#[cfg(test)]
mod test {
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::Register16::*;
    use crate::cpu::Flags;

    #[test]
    fn test_implied() {
        let mut runner = TestRunner::new();
        runner.load_program(&[0x40]);
        runner.push(0x12);
        runner.push(0x34);
        runner.push(0x56);
        runner.test()
            .verify(PC, 0x1234)
            .verify(Flags{}, 0x56);
    }
}