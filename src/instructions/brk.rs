
use crate::cpu::addressing_mode::{read_param, AddressingMode};
use super::Inst;

pub fn make(mode: AddressingMode, bytes: &[u8]) -> Inst {
    assert!(matches!(mode, AddressingMode::Implied));
    Inst {
        name: "BRK",
        param: read_param(mode, bytes),
        mode,
        f: move |_, cpu| {
            cpu.flags.set_b(true);
            cpu.push16(cpu.pc);
            cpu.push8(cpu.flags.get());
            cpu.pc = cpu.get_mem16(0xFFFE);

        },
    }
}

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[(0x00, AddressingMode::Implied)];

#[cfg(test)]
mod tests {
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::test_util::Register8::*;
    use crate::cpu::test_util::Flag::*;

    #[test]
    fn test_brk() {
        let mut runner = TestRunner::new();
        runner.test(&[0xe8])
            .verify(X, 1)
            .verify(Z, false)
            .verify(N, false);
        runner.set(X, 0xff);
        runner.test(&[0xe8])
            .verify(X, 0)
            .verify(Z, true)
            .verify(N, false);
        runner.set(X, 0x7f);
        runner.test(&[0xe8])
            .verify(X, 0x80)
            .verify(Z, false)
            .verify(N, true);
    }
}