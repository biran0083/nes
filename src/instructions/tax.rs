use crate::cpu::addressing_mode::AddressingMode;
use super::InstFun;


pub const RUN : InstFun = |ins, cpu| {
    cpu.x = cpu.a;
    cpu.update_z(cpu.x);
    cpu.update_n(cpu.x);
    cpu.pc += ins.len();
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[(0xaa, AddressingMode::Implied)];

#[cfg(test)]
mod test {
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::test_util::Flag::*;
    use crate::cpu::test_util::Register8::*;

    #[test]
    fn test_tax() {
        let mut runner = TestRunner::new();
        runner.set(A, 0x21);
        runner.load_and_test(&[0xaa])
            .verify(X, 0x21)
            .verify(Z, false)
            .verify(N, false);
        runner.set(A, 0);
        runner.load_and_test(&[0xaa])
            .verify(X, 0)
            .verify(Z, true)
            .verify(N, false);
        runner.set(A, 0xf0);
        runner.load_and_test(&[0xaa])
            .verify(X, 0xf0)
            .verify(Z, false)
            .verify(N, true);
    }
}