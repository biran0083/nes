use crate::cpu::addressing_mode::AddressingMode;
use super::InstFun;

pub const RUN : InstFun = |ins, cpu| {
    cpu.a = cpu.pop8();
    cpu.update_n(cpu.a);
    cpu.update_z(cpu.a);
    cpu.pc += ins.len();
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
    (0x68, AddressingMode::Implied),
];

#[cfg(test)]
mod test {
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::Register8::*;
    use crate::cpu::Flag::*;

    #[test]
    fn test() {
        let mut runner = TestRunner::new();
        runner.load_program(&[0x68]);
        runner.push(0x12);
        runner.test()
            .verify(A, 0x12)
            .verify(SP, 0xff)
            .verify(N, false)
            .verify(Z,false);

        runner.load_program(&[0x68]);
        runner.push(0);
        runner.test()
            .verify(A, 0)
            .verify(SP, 0xff)
            .verify(N, false)
            .verify(Z,true);

        runner.load_program(&[0x68]);
        runner.push(0xf0);
        runner.test()
            .verify(A, 0xf0)
            .verify(SP, 0xff)
            .verify(N, true)
            .verify(Z, false);
    }
}