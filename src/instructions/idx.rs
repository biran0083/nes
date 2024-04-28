use crate::cpu::addressing_mode::{read_param, AddressingMode};
use super::Inst;

pub fn make(mode: AddressingMode, bytes: &[u8]) -> Inst {
    assert!(matches!(mode, AddressingMode::Implied));
    Inst {
        name: "INX",
        param: read_param(mode, bytes),
        mode,
        f: move |ins, cpu| {
            cpu.X = cpu.X.wrapping_add(1);
            cpu.update_z(cpu.X);
            cpu.update_n(cpu.X);
            cpu.PC += ins.len();
        },
    }
}

pub const OPCODE_MAP: &[(u8, AddressingMode)] =  &[(0xe8, AddressingMode::Implied)];

#[cfg(test)]
mod test {
    use crate::cpu::test_util::TestRunner;

    #[test]
    fn test_inx() {
        let mut runner = TestRunner::new()
            .verify_registers(&["X"])
            .verify_flags(&["Z", "N"]);
        runner.test(&[0xe8], &[1], &[0, 0]);
        runner.set_register("X", 0xff);
        runner.test(&[0xe8], &[0], &[1, 0]);
        runner.set_register("X", 0x7f);
        runner.test(&[0xe8], &[0x80], &[0, 1]);
    }
}