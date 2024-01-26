use std::collections::HashMap;

use crate::instructions::{make_inst_factories_by_op_code, Inst, InstFactory};

#[derive(Default)]
pub struct Flags {
    pub C: u8,
    pub Z: u8,
    pub I: u8,
    pub D: u8,
    pub B: u8,
    pub V: u8,
    pub N: u8,
}

pub struct CPU {
    // registers
    pub X: u8,
    pub Y: u8,
    pub A: u8,
    pub SP: u8,
    pub PC: u16,
    pub flags: Flags,
    pub mem: Vec<u8>,
    inst_factories: HashMap<u8, InstFactory>,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            X: 0,
            Y: 0,
            A: 0,
            SP: 0,
            PC: 0,
            flags: Flags::default(),
            mem: vec![0; 0x10000],
            inst_factories: make_inst_factories_by_op_code(),
        }
    }

    pub fn update_z(&mut self) {
        self.flags.Z = if self.A == 0 { 1 } else { 0 };
    }

    pub fn update_n(&mut self) {
        self.flags.N = if self.A & 0x80 != 0 { 1 } else { 0 };
    }

    pub fn load_program(&mut self, bytes: &[u8]) {
        assert!(bytes.len() < 0x8000);
        let start: usize = 0x8000;
        self.PC = start as u16;
        self.mem[start..(start + bytes.len())].copy_from_slice(bytes);
    }

    fn decode(&mut self) -> Inst {
        let op = self.mem[self.PC as usize];
        self.inst_factories
            .get(&op)
            .unwrap()
            .make(&self.mem[((self.PC + 1) as usize)..])
    }

    pub fn run_once(&mut self) {
        let ins = self.decode();
        ins.run(self);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::iter::zip;

    struct TestRunner {
        registers: Vec<String>,
        flags: Vec<String>,
        cpu: CPU,
    }

    impl TestRunner {
        fn new() -> Self {
            TestRunner {
                registers: vec![],
                flags: vec![],
                cpu: CPU::new(),
            }
        }

        fn verify_registers(mut self, rs: &[&str]) -> Self {
            for r in rs {
                self.registers.push(r.to_string());
            }
            self
        }

        fn verify_flags(mut self, fs: &[&str]) -> Self {
            for f in fs {
                self.flags.push(f.to_string())
            }
            self
        }

        fn set_memory(&mut self, addr: usize, value: u8) {
            self.cpu.mem[addr] = value;
        }

        fn set_register(&mut self, name: &str, value: u16) {
            match name {
                "X" => self.cpu.X = value as u8,
                "Y" => self.cpu.Y = value as u8,
                "A" => self.cpu.A = value as u8,
                "SP" => self.cpu.SP = value as u8,
                "PC" => self.cpu.PC = value,
                _ => panic!("unknown register {name}"),
            }
        }

        fn test(&mut self, bytes: &[u8], rs: &[u16], fs: &[u8]) {
            assert_eq!(rs.len(), self.registers.len());
            assert_eq!(fs.len(), self.flags.len());
            self.cpu.load_program(bytes);
            self.cpu.run_once();
            for (name, r) in zip(self.registers.iter(), rs.iter()) {
                match name.as_ref() {
                    "X" => assert_eq!(self.cpu.X, *r as u8),
                    "Y" => assert_eq!(self.cpu.Y, *r as u8),
                    "A" => assert_eq!(self.cpu.A, *r as u8),
                    "SP" => assert_eq!(self.cpu.SP, *r as u8),
                    "PC" => assert_eq!(self.cpu.PC, *r),
                    _ => panic!("unknown register {name}"),
                }
            }
            for (name, f) in zip(self.flags.iter(), fs.iter()) {
                match name.as_ref() {
                    "C" => assert_eq!(self.cpu.flags.C, *f),
                    "Z" => assert_eq!(self.cpu.flags.Z, *f),
                    "I" => assert_eq!(self.cpu.flags.I, *f),
                    "D" => assert_eq!(self.cpu.flags.D, *f),
                    "B" => assert_eq!(self.cpu.flags.B, *f),
                    "V" => assert_eq!(self.cpu.flags.V, *f),
                    "N" => assert_eq!(self.cpu.flags.N, *f),
                    _ => panic!("unknown flag {name}"),
                }
            }
        }
    }

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
        runner.set_memory(0x01, 10);
        runner.test(&[0xA5, 0x01], &[10], &[0, 0]);
        runner.set_memory(0x01, 0xff);
        runner.test(&[0xA5, 0x01], &[0xff], &[0, 1]);
        // Zero Page X
        runner.set_memory(0x01, 0x00);
        runner.test(&[0xB5, 0x01], &[0], &[1, 0]);
        runner.set_register("X", 2);
        runner.set_memory(0x03, 10);
        runner.test(&[0xB5, 0x01], &[10], &[0, 0]);
        runner.set_register("X", 0x80);
        runner.set_memory(0x7f, 0xff);
        runner.test(&[0xB5, 0xff], &[0xff], &[0, 1]);
        // Absolute
        runner.set_memory(0x1234, 0x11);
        runner.test(&[0xAD, 0x34, 0x12], &[0x11], &[0, 0]);
        // Absolute X
        runner.set_memory(0x1235, 0xf0);
        runner.set_register("X", 1);
        runner.test(&[0xBD, 0x34, 0x12], &[0xf0], &[0, 1]);
        // Absolute Y
        runner.set_memory(0x1236, 0x13);
        runner.set_register("Y", 2);
        runner.test(&[0xB9, 0x34, 0x12], &[0x13], &[0, 0]);
        // (Indirect,X)
        runner.set_register("X", 0x11);
        runner.set_memory(0x21, 0x12);
        runner.set_memory(0x22, 0x34);
        runner.set_memory(0x3412, 0x56);
        runner.test(&[0xA1, 0x10], &[0x56], &[0, 0]);
        // (Indirect,Y)
        runner.set_register("Y", 0x0f);
        runner.set_memory(0x10, 0x45);
        runner.set_memory(0x11, 0x23);
        runner.set_memory(0x2345, 0xff);
        runner.test(&[0xB1, 0x10], &[0x0e], &[0, 0]);
    }

    #[test]
    fn test_tax() {
        let mut runner = TestRunner::new()
            .verify_registers(&["X"])
            .verify_flags(&["Z", "N"]);
        runner.set_register("A", 0x21);
        runner.test(&[0xaa], &[0x21], &[0, 0]);
        runner.set_register("A", 0);
        runner.test(&[0xaa], &[0], &[1, 0]);
        runner.set_register("A", 0xf0);
        runner.test(&[0xaa], &[0xf0], &[0, 1]);
    }
}
