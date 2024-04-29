use crate::cpu::addressing_mode::{load_operand, load_operand_opt, read_param, AddressingMode};
use crate::cpu::CPU;
use super::Inst;

pub fn make(mode: AddressingMode, bytes: &[u8]) -> Inst {
    Inst {
        name: "BCC",
        param: read_param(mode, bytes),
        mode,
        f: move |ins, cpu: &mut CPU| {
            let operand : i8 = load_operand(ins.mode, cpu, ins.param.unwrap()) as i8;
            if !cpu.flags.c() {
                cpu.pc = cpu.pc.wrapping_add(operand as u16);
            }
            cpu.pc = cpu.pc.wrapping_add(ins.len());
        },
    }
}

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
        (0x90, AddressingMode::Relative),
    ];


#[cfg(test)]
mod test {
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::test_util::Register16::*;
    use crate::cpu::test_util::Flag::*;

    #[test]
    fn test_relative() {
        let mut runner = TestRunner::new();
        runner.set(PC, 0x8000);
        runner.test(&[0x90, 0x01])
            .verify(PC, 0x8003);
        runner.set(PC, 0x8000);
        runner.test(&[0x90, 0x80])
            .verify(PC, 0x7f82);
        runner.set(PC, 0x8000);
        runner.test(&[0x90, 0xff])
            .verify(PC, 0x8001);
        runner.set(C, true);
        runner.test(&[0x90, 0xff])
            .verify(PC, 0x8002);
    }
}