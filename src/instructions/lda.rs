use crate::cpu::addressing_mode::{load_operand, read_param, AddressingMode};
use crate::cpu::CPU;
use super::Inst;

pub fn make(mode: AddressingMode, bytes: &[u8]) -> Inst {
    Inst {
        name: "LDA",
        param: read_param(mode, bytes),
        mode,
        f: move |ins, cpu: &mut CPU| {
            cpu.A = load_operand(ins.mode, cpu, ins.param.unwrap());
            cpu.update_z(cpu.A);
            cpu.update_n(cpu.A);
            cpu.PC += ins.len();
        },
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::test_util::TestRunner;

    #[test]
    fn test_lda() {
        // Immediate
        let mut runner = TestRunner::new()
            .verify_registers(&["A"])
            .verify_flags(&["Z", "N"]);
        runner.test(&[0xA9, 0x00], &[0], &[1, 0]);
        runner.test(&[0xA9, 0x01], &[0x01], &[0, 0]);
        runner.test(&[0xA9, 0x91], &[0x91], &[0, 1]);
        // Zero Page
        runner.test(&[0xA5, 0x01], &[0], &[1, 0]);
        runner.set_mem(0x01, 10);
        runner.test(&[0xA5, 0x01], &[10], &[0, 0]);
        runner.set_mem(0x01, 0xff);
        runner.test(&[0xA5, 0x01], &[0xff], &[0, 1]);
        // Zero Page X
        runner.set_mem(0x01, 0x00);
        runner.test(&[0xB5, 0x01], &[0], &[1, 0]);
        runner.set_register("X", 2);
        runner.set_mem(0x03, 10);
        runner.test(&[0xB5, 0x01], &[10], &[0, 0]);
        runner.set_register("X", 0x80);
        runner.set_mem(0x7f, 0xff);
        runner.test(&[0xB5, 0xff], &[0xff], &[0, 1]);
        // Absolute
        runner.set_mem(0x1234, 0x11);
        runner.test(&[0xAD, 0x34, 0x12], &[0x11], &[0, 0]);
        // Absolute X
        runner.set_mem(0x1235, 0xf0);
        runner.set_register("X", 1);
        runner.test(&[0xBD, 0x34, 0x12], &[0xf0], &[0, 1]);
        // Absolute Y
        runner.set_mem(0x1236, 0x13);
        runner.set_register("Y", 2);
        runner.test(&[0xB9, 0x34, 0x12], &[0x13], &[0, 0]);
        // (Indirect,X)
        runner.set_register("X", 0x11);
        runner.set_mem(0x21, 0x12);
        runner.set_mem(0x22, 0x34);
        runner.set_mem(0x3412, 0x56);
        runner.test(&[0xA1, 0x10], &[0x56], &[0, 0]);
        // (Indirect,Y)
        runner.set_register("Y", 0x0f);
        runner.set_mem(0x10, 0x45);
        runner.set_mem(0x11, 0x23);
        runner.set_mem(0x2345, 0xff);
        runner.test(&[0xB1, 0x10], &[0x0e], &[0, 0]);
    }

}