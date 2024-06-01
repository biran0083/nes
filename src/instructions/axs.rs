use crate::cpu::addressing_mode::{load_operand, AddressingMode};
use super::InstFun;

pub const RUN : InstFun = |ins, cpu| {
    cpu.x = (cpu.x & cpu.a).wrapping_sub(load_operand(ins.mode, cpu, ins.param.unwrap()));
    cpu.update_z(cpu.x);
    cpu.update_n(cpu.x);
    // TODO: how to update C?
    cpu.pc += ins.len();
};

pub const OPCODE_MAP: &[(u8, AddressingMode)] =  &[(0xCB, AddressingMode::Immediate)];

#[cfg(test)]
mod test {
    use crate::cpu::test_util::TestRunner;
    use crate::cpu::Register8::*;
    use crate::cpu::Flag::*;

    #[test]
    fn test_inx() {
        let mut runner = TestRunner::new();
        runner.set_mem(0x2312, 0x22);
        runner.set(X, 0x56);
        runner.set(A, 0x78);
        runner.load_and_test(&[0xCB, 0x12])
            .verify(X, 0x3E)
            .verify(Z, false)
            .verify(N, false);
    }
}