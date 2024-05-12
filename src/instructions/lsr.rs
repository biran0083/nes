use crate::cpu::addressing_mode::{load_operand_addr, AddressingMode};
use crate::cpu::CPU;
use super::InstFun;


pub const RUN : InstFun = |ins, cpu: &mut CPU| {
    if ins.mode == AddressingMode::Accumulator {
        let value = cpu.a;
        cpu.a =  value >> 1;
        cpu.update_z(cpu.a);
        cpu.flags.set_c(value & 1 == 1);
    } else {
        let addr = load_operand_addr(ins.mode, cpu, ins.param.unwrap());
        let value = cpu.get_mem(addr);
        let res = value >> 1;
        cpu.set_mem(addr, res);
        cpu.update_z(res);
        cpu.flags.set_c(value & 1 == 1);
    }
    cpu.flags.set_n(false);
    cpu.pc += ins.len();
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
        (0x4A, AddressingMode::Accumulator),
        (0x46, AddressingMode::ZeroPage),
        (0x56, AddressingMode::ZeroPageX),
        (0x4E, AddressingMode::Absolute),
        (0x5E, AddressingMode::AbsoluteX),
    ];


#[cfg(test)]
mod test {
    use crate::cpu::test_util::Mem;
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::test_util::Register8::*;
    use crate::cpu::test_util::Flag::*;

    #[test]
    fn test_accumulator() {
        let mut runner = TestRunner::new();
        runner.set(A, 0x01);
        runner.test(&[0x4A])
            .verify(A, 0)
            .verify(C, true)
            .verify(Z, true)
            .verify(N, false);
        runner.set(A, 0x80);
        runner.test(&[0x4A])
            .verify(A, 0x40)
            .verify(C, false)
            .verify(Z, false)
            .verify(N, false);
    }

    #[test]
    fn test_zero_page() {
        let mut runner = TestRunner::new();
        runner.set_mem(0x10, 0x01);
        runner.test(&[0x46, 0x10])
            .verify(Mem::new(0x10), 0)
            .verify(C, true)
            .verify(Z, true)
            .verify(N, false);
        runner.set_mem(0x11, 0x80);
        runner.test(&[0x46, 0x11])
            .verify(Mem::new(0x11), 0x40)
            .verify(C, false)
            .verify(Z, false)
            .verify(N, false);
    }
}