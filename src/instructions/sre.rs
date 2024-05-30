use crate::cpu::addressing_mode::{load_operand_addr, AddressingMode};
use crate::cpu::CPU;
use super::InstFun;


pub const RUN : InstFun = |ins, cpu: &mut CPU| {
    let addr = load_operand_addr(ins.mode, cpu, ins.param.unwrap());
    let value = cpu.get_mem(addr);
    let res = value >> 1;
    cpu.set_mem(addr, res);
    cpu.flags.set_c(value & 1 == 1);
    cpu.a = cpu.a ^ res;
    cpu.update_z(cpu.a);
    cpu.update_n(cpu.a);
    cpu.pc += ins.len();
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
        (0x47, AddressingMode::ZeroPage),
        (0x57, AddressingMode::ZeroPageX),
        (0x4F, AddressingMode::Absolute),
        (0x5F, AddressingMode::AbsoluteX),
        (0x5B, AddressingMode::AbsoluteY),
        (0x43, AddressingMode::IndexedIndirect),
        (0x53, AddressingMode::IndirectIndexed),
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
        runner.set_mem(0x10, 0xff);
        runner.set(A, 0x32);
        runner.load_and_test(&[0x47, 0x10])
            .verify(Mem::new(0x10), 0x7f)
            .verify(A, 0x4d)
            .verify(C, true)
            .verify(Z, false)
            .verify(N, false);

    }
}