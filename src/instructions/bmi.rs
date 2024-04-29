use crate::cpu::addressing_mode::{read_param, load_operand, AddressingMode};
use super::Inst;

pub fn make(mode: AddressingMode, bytes: &[u8]) -> Inst {
    assert!(matches!(mode, AddressingMode::Relative));
    Inst {
        name: "BMI",
        param: read_param(mode, bytes),
        mode,
        f: move |ins, cpu| {
            let operand : i8 = load_operand(ins.mode, cpu, ins.param.unwrap()) as i8;
            if cpu.flags.n() {
                cpu.pc = cpu.pc.wrapping_add(operand as u16);
            }
            cpu.pc = cpu.pc.wrapping_add(ins.len());

        },
    }
}

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[(0x30, AddressingMode::Relative)];

#[cfg(test)]
mod tests {
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::test_util::Register16::*;
    use crate::cpu::test_util::Flag::*;

    #[test]
    fn test_relative() {
        let mut runner = TestRunner::new();
        runner.set(N, true);
        runner.set(PC, 0x8000);
        runner.test(&[0x30, 0x01])
            .verify(PC, 0x8003);
        runner.set(PC, 0x8000);
        runner.test(&[0x30, 0x80])
            .verify(PC, 0x7f82);
        runner.set(PC, 0x8000);
        runner.test(&[0x30, 0xff])
            .verify(PC, 0x8001);
        runner.set(N, false);
        runner.test(&[0x30, 0xff])
            .verify(PC, 0x8002);
    }
}