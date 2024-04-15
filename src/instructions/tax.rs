use crate::cpu::addressing_mode::{read_param, AddressingMode};
use super::Inst;

pub fn make(mode: AddressingMode, bytes: &[u8]) -> Inst {
    assert!(matches!(mode, AddressingMode::Implied));
    Inst {
        name: "TAX",
        param: read_param(mode, bytes),
        mode,
        f: move |ins, cpu| {
            cpu.X = cpu.A;
            cpu.update_z(cpu.X);
            cpu.update_n(cpu.X);
            cpu.PC += ins.len();
        },
    }
}

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[(0xaa, AddressingMode::Implied)];

#[cfg(test)]
mod test {
    use crate::cpu::test_util::TestRunner;

    #[test]
    fn test_tax() {
        let mut runner = TestRunner::new()
            .verify_registers(&["X"])
            .verify_flags(&["Z", "N"]);
        runner.set_register("A", 0x21);
        runner.test(&[0xaa], &[0x21], &[0, 0]);
        runner.set_register("A", 0);
        runner.test(&[0xaa], &[0], &[1, 0]);
        runner.set_register("A", 0xf0);
        runner.test(&[0xaa], &[0xf0], &[0, 1]);
    }
}