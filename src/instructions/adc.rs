use crate::cpu::addressing_mode::{load_operand, read_param, AddressingMode};
use super::Inst;

pub fn make(mode: AddressingMode, bytes: &[u8]) -> Inst {
    Inst {
        name: "ADC",
        param: read_param(mode, bytes),
        mode,
        f: move |ins, cpu| {
            let operand = load_operand(ins.mode, cpu, ins.param.unwrap());
            let result16 = cpu.a as u16 + operand as u16 + cpu.flags.c() as u16;
            let result = result16 as u8;
            cpu.flags.set_c((result16 >> 8) & 1 != 0);
            cpu.flags.set_v((cpu.a ^ result) & (operand ^ result) & 0x80 != 0);
            cpu.a = result;
            cpu.update_z(cpu.a);
            cpu.update_n(cpu.a);
            cpu.pc += ins.len();
        },
    }
}

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
        (0x69, AddressingMode::Immediate),
        (0x65, AddressingMode::ZeroPage),
        (0x75, AddressingMode::ZeroPageX),
        (0x6D, AddressingMode::Absolute),
        (0x7D, AddressingMode::AbsoluteX),
        (0x79, AddressingMode::AbsoluteY),
        (0x61, AddressingMode::IndexedIndirect),
        (0x71, AddressingMode::IndirectIndexed),
    ];

#[cfg(test)]
mod test {
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::test_util::Register8::*;
    use crate::cpu::test_util::Flag::*;


    #[test]
    fn test_adc_immediate() {
        let mut runner = TestRunner::new();
        runner.set(A, 0x01);
        runner.test(&[0x69, 0x01])
            .verify(A, 0x02)
            .verify(C, false)
            .verify(Z, false)
            .verify(V, false)
            .verify(N, false);
        runner.set(A, 0x7f);
        runner.test(&[0x69, 0x01])
            .verify(A, 0x80)
            .verify(C, false)
            .verify(Z, false)
            .verify(V, true)
            .verify(N, true);
        runner.set(A, 0x80);
        runner.test(&[0x69, 0x01])
            .verify(A, 0x81)
            .verify(C, false)
            .verify(Z, false)
            .verify(V, false)
            .verify(N, true);
        runner.set(A, 0xff);
        runner.test(&[0x69, 0x01])
            .verify(A, 0x00)
            .verify(C, true)
            .verify(Z, true)
            .verify(V, false)
            .verify(N, false);
        runner.set(A, 0xff);
        runner.set(C, true);
        runner.test(&[0x69, 0xff])
            .verify(A, 0xff)
            .verify(C, true)
            .verify(Z, false)
            .verify(V, false)
            .verify(N, true);
    }

    #[test]
    fn test_adc_zero_page() {
        let mut runner = TestRunner::new();
        runner.set_mem(0x01, 0x01);
        runner.set(A, 0x01);
        runner.test(&[0x65, 0x01])
            .verify(A, 0x02)
            .verify(C, false)
            .verify(Z, false)
            .verify(V, false)
            .verify(N, false);
        runner.set_mem(0x01, 0x7f);
        runner.set(A, 0x7f);
        runner.test(&[0x65, 0x01])
            .verify(A, 0xfe)
            .verify(C, false)
            .verify(Z, false)
            .verify(V, true)
            .verify(N, true);
        runner.set_mem(0x01, 0x80);
        runner.set(A, 0x80);
        runner.test(&[0x65, 0x01])
            .verify(A, 0)
            .verify(C, true)
            .verify(Z, true)
            .verify(V, true)
            .verify(N, false);
        runner.set(C, true);
        runner.set_mem(0x01, 0xff);
        runner.set(A, 0xff);
        runner.test(&[0x65, 0x01])
            .verify(A, 0xff)
            .verify(C, true)
            .verify(Z, false)
            .verify(V, false)
            .verify(N, true);
        runner.set_mem(0x01, 0xff);
        runner.set(A, 0xff);
        runner.set(C, true);
        runner.test(&[0x65, 0x01])
            .verify(A, 0xff)
            .verify(C, true)
            .verify(Z, false)
            .verify(V, false)
            .verify(N, true);
    }
}