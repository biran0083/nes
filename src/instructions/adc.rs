use crate::cpu::addressing_mode::{load_operand, read_param, AddressingMode};
use super::Inst;

pub fn make(mode: AddressingMode, bytes: &[u8]) -> Inst {
    Inst {
        name: "ADC",
        param: read_param(mode, bytes),
        mode,
        f: move |ins, cpu| {
            let operand = load_operand(ins.mode, cpu, ins.param.unwrap());
            let result16 = cpu.A as u16 + operand as u16 + cpu.flags.C as u16;
            let result = result16 as u8;
            cpu.flags.C = ((result16 >> 8) & 1) as u8;
            cpu.flags.V = if (cpu.A ^ result) & (operand ^ result) & 0x80 != 0 {
                1
            } else {
                0
            };
            cpu.A = result;
            cpu.update_z();
            cpu.update_n();
            cpu.PC += ins.len();
        },
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::test_util::TestRunner;


    #[test]
    fn test_adc_implied() {
        let mut runner = TestRunner::new()
            .verify_registers(&["A"])
            .verify_flags(&["C", "Z", "V", "N"]);
        runner.set_register("A", 0x01);
        runner.test(&[0x69, 0x01], &[0x02], &[0, 0, 0, 0]);
        runner.set_register("A", 0x7f);
        runner.test(&[0x69, 0x01], &[0x80], &[0, 0, 1, 1]);
        runner.set_register("A", 0x80);
        runner.test(&[0x69, 0x01], &[0x81], &[0, 0, 0, 1]);
        runner.set_register("A", 0xff);
        runner.test(&[0x69, 0x01], &[0x00], &[1, 1, 0, 0]);
        runner.set_register("A", 0xff);
        runner.set_flag("C", 1);
        runner.test(&[0x69, 0xff], &[0xff], &[1, 0, 0, 1]);
    }

    #[test]
    fn test_adc_zero_page() {
        let mut runner = TestRunner::new()
            .verify_registers(&["A"])
            .verify_flags(&["C", "Z", "V", "N"]);
        runner.set_mem(0x01, 0x01);
        runner.set_register("A", 0x01);
        runner.test(&[0x65, 0x01], &[0x02], &[0, 0, 0, 0]);
        runner.set_mem(0x01, 0x7f);
        runner.set_register("A", 0x7f);
        runner.test(&[0x65, 0x01], &[0xfe], &[0, 0, 1, 1]);
        runner.set_mem(0x01, 0x80);
        runner.set_register("A", 0x80);
        runner.test(&[0x65, 0x01], &[0], &[1, 1, 1, 0]);
        runner.set_flag("C", 1);
        runner.set_mem(0x01, 0xff);
        runner.set_register("A", 0xff);
        runner.test(&[0x65, 0x01], &[0xff], &[1, 0, 0, 1]);
        runner.set_mem(0x01, 0xff);
        runner.set_register("A", 0xff);
        runner.set_flag("C", 1);
        runner.test(&[0x65, 0x01], &[0xff], &[1, 0, 0, 1]);
    }
}