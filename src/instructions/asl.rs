use crate::cpu::addressing_mode::{load_operand_addr, AddressingMode};
use crate::cpu::CPU;
use super::InstFun;


pub const RUN : InstFun = |ins, cpu: &mut CPU| {
    let result: u16 = if ins.mode == AddressingMode::Accumulator {
        let res: u16 = (cpu.a as u16) << 1;
        cpu.a = res as u8;
        res
    } else {
        let addr = load_operand_addr(ins.mode, cpu, ins.param.unwrap());
        let m = cpu.get_mem(addr);
        let res: u16 = (m as u16) << 1;
        cpu.set_mem(addr, res as u8);
        res
    };
    cpu.update_z(result as u8);
    cpu.update_n(result as u8);
    cpu.flags.set_c(result & 0x100 != 0);
    cpu.pc += ins.len();
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
        (0x0A, AddressingMode::Accumulator),
        (0x06, AddressingMode::ZeroPage),
        (0x16, AddressingMode::ZeroPageX),
        (0x0E, AddressingMode::Absolute),
        (0x1E, AddressingMode::AbsoluteX),
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
        runner.load_and_test(&[0x0A])
            .verify(A, 2)
            .verify(C, false)
            .verify(Z, false)
            .verify(N, false);
        runner.set(A, 0x80);
        runner.load_and_test(&[0x0A])
            .verify(A, 0)
            .verify(C, true)
            .verify(Z, true)
            .verify(N, false);
        runner.set(A, 0x40);
        runner.load_and_test(&[0x0A])
            .verify(A, 0x80)
            .verify(C, false)
            .verify(Z, false)
            .verify(N, true);
    }

    #[test]
    fn test_zero_page() {
        let mut runner = TestRunner::new();
        runner.set_mem(0x10, 0x01);
        runner.load_and_test(&[0x06, 0x10])
            .verify(Mem::new(0x10), 2)
            .verify(C, false)
            .verify(Z, false)
            .verify(N, false);
        runner.set_mem(0x11, 0x80);
        runner.load_and_test(&[0x06, 0x11])
            .verify(Mem::new(0x11), 0)
            .verify(C, true)
            .verify(Z, true)
            .verify(N, false);
        runner.set_mem(0x12, 0x40);
        runner.load_and_test(&[0x06, 0x12])
            .verify(Mem::new(0x12), 0x80)
            .verify(C, false)
            .verify(Z, false)
            .verify(N, true);
    }
}