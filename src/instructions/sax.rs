use crate::cpu::addressing_mode::{load_operand_addr, AddressingMode};
use super::InstFun;

pub const RUN : InstFun = |ins, cpu| {
    let addr = load_operand_addr(ins.mode, cpu, ins.param.unwrap());
    let value = cpu.a & cpu.x;
    cpu.set_mem(addr, value);
    cpu.pc += ins.len();
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
        (0x87, AddressingMode::ZeroPage),
        (0x97, AddressingMode::ZeroPageY),
        (0x83, AddressingMode::IndexedIndirect),
        (0x8F, AddressingMode::Absolute),
    ];

#[cfg(test)]
mod test {
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::Mem;
    use crate::cpu::Register8::*;

    #[test]
    fn test_zero_page() {
        let mut runner = TestRunner::new();
        runner.set(A, 0x52);
        runner.set(X, 0x61);
        runner.load_and_test(&[0x87, 0x10])
            .verify(Mem::new(0x10), 0x40);
    }
}