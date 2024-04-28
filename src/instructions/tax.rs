use crate::cpu::addressing_mode::{read_param, AddressingMode};
use super::Inst;

pub fn make(mode: AddressingMode, bytes: &[u8]) -> Inst {
    assert!(matches!(mode, AddressingMode::Implied));
    Inst {
        name: "TAX",
        param: read_param(mode, bytes),
        mode,
        f: move |ins, cpu| {
            cpu.x = cpu.a;
            cpu.update_z(cpu.x);
            cpu.update_n(cpu.x);
            cpu.pc += ins.len();
        },
    }
}

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[(0xaa, AddressingMode::Implied)];

#[cfg(test)]
mod test {
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::test_util::Flag::*;
    use crate::cpu::test_util::Register8::*;

    #[test]
    fn test_tax() {
        let mut runner = TestRunner::new();
        runner.set_register("A", 0x21);
        runner.test(&[0xaa])
            .verify(X, 0x21)
            .verify(Z, false)
            .verify(N, false);
        runner.set_register("A", 0);
        runner.test(&[0xaa])
            .verify(X, 0)
            .verify(Z, true)
            .verify(N, false);
        runner.set_register("A", 0xf0);
        runner.test(&[0xaa])
            .verify(X, 0xf0)
            .verify(Z, false)
            .verify(N, true);
    }
}