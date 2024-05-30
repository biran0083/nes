use crate::cpu::addressing_mode::AddressingMode;
use super::InstFun;

pub const RUN : InstFun = |ins, cpu| {
    let value = cpu.pop8();
    cpu.flags.set(value);
    cpu.flags.set_b(false);
    cpu.pc += ins.len();
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
    (0x28, AddressingMode::Implied),
];

#[cfg(test)]
mod test {
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::Flags;

    #[test]
    fn test() {
        let mut runner = TestRunner::new();
        runner.load_program(&[0x28]);
        runner.push(0b0101_1100);
        runner.test()
            .verify(Flags{}, 0b0110_1100);
    }
}