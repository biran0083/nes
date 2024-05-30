use crate::cpu::addressing_mode::{load_operand_addr, AddressingMode};
use crate::cpu::CPU;
use super::InstFun;


pub const RUN : InstFun = |ins, cpu: &mut CPU| {
    let addr = load_operand_addr(ins.mode, cpu, ins.param.unwrap());
    let m = cpu.get_mem(addr);
    let value: u16 = (m as u16) << 1;
    cpu.set_mem(addr, value as u8);
    cpu.flags.set_c(value & 0x100 != 0);
    cpu.a = value as u8 | cpu.a;
    cpu.update_z(cpu.a as u8);
    cpu.update_n(cpu.a as u8);
    cpu.pc += ins.len();
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
        (0x07, AddressingMode::ZeroPage),
        (0x17, AddressingMode::ZeroPageX),
        (0x0F, AddressingMode::Absolute),
        (0x1F, AddressingMode::AbsoluteX),
        (0x1B, AddressingMode::AbsoluteY),
        (0x03, AddressingMode::IndexedIndirect),
        (0x13, AddressingMode::IndirectIndexed),
    ];


#[cfg(test)]
mod test {
    use crate::cpu::Mem;
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::Register8::*;
    use crate::cpu::Flag::*;

    #[test]
    fn test_zero_page() {
        let mut runner = TestRunner::new();
        runner.set_mem(0x10, 0x01);
        runner.set(A, 0x10);
        runner.load_and_test(&[0x07, 0x10])
            .verify(Mem::new(0x10), 0x02)
            .verify(A, 0x12)
            .verify(C, false)
            .verify(Z, false)
            .verify(N, false);

        runner.set_mem(0x10, 0xf0);
        runner.set(A, 0x00);
        runner.load_and_test(&[0x07, 0x10])
            .verify(Mem::new(0x10), 0xe0)
            .verify(A, 0xe0)
            .verify(C, true)
            .verify(Z, false)
            .verify(N, true);

        runner.set_mem(0x10, 0x80);
        runner.set(A, 0x00);
        runner.load_and_test(&[0x07, 0x10])
            .verify(Mem::new(0x10), 0)
            .verify(A, 0)
            .verify(C, true)
            .verify(Z, true)
            .verify(N, false);
    }
}