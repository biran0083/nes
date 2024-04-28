use crate::cpu::addressing_mode::{read_param, AddressingMode};
use super::Inst;

pub fn make(mode: AddressingMode, bytes: &[u8]) -> Inst {
    assert!(matches!(mode, AddressingMode::Implied));
    Inst {
        name: "INX",
        param: read_param(mode, bytes),
        mode,
        f: move |ins, cpu| {
            cpu.x = cpu.x.wrapping_add(1);
            cpu.update_z(cpu.x);
            cpu.update_n(cpu.x);
            cpu.pc += ins.len();
        },
    }
}

pub const OPCODE_MAP: &[(u8, AddressingMode)] =  &[(0xe8, AddressingMode::Implied)];

#[cfg(test)]
mod test {
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::test_util::Register8::*;
    use crate::cpu::test_util::Flag::*;

    #[test]
    fn test_inx() {
        let mut runner = TestRunner::new();
        runner.test(&[0xe8])
            .verify(X, 1)
            .verify(Z, false)
            .verify(N, false);

        runner.set_register("X", 0xff);
        runner.test(&[0xe8])
            .verify(X, 0)
            .verify(Z, true)
            .verify(N, false);
        runner.set_register("X", 0x7f);
        runner.test(&[0xe8])
            .verify(X, 0x80)
            .verify(Z, false)
            .verify(N, true);
    }
}