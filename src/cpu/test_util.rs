
use std::iter::zip;
use crate::cpu::CPU;

pub struct TestRunner {
    registers: Vec<String>,
    flags: Vec<String>,
    cpu: CPU,
}

impl TestRunner {
    pub fn new() -> Self {
        TestRunner {
            registers: vec![],
            flags: vec![],
            cpu: CPU::new(),
        }
    }

    pub fn verify_registers(mut self, rs: &[&str]) -> Self {
        for r in rs {
            self.registers.push(r.to_string());
        }
        self
    }

    pub fn verify_flags(mut self, fs: &[&str]) -> Self {
        for f in fs {
            self.flags.push(f.to_string())
        }
        self
    }

    pub fn set_mem(&mut self, addr: usize, value: u8) {
        self.cpu.mem[addr] = value;
    }

    pub fn set_register(&mut self, name: &str, value: u16) {
        match name {
            "X" => self.cpu.X = value as u8,
            "Y" => self.cpu.Y = value as u8,
            "A" => self.cpu.A = value as u8,
            "SP" => self.cpu.SP = value as u8,
            "PC" => self.cpu.PC = value,
            _ => panic!("unknown register {name}"),
        }
    }

    pub fn set_flag(&mut self, name: &str, v: u8) {
        assert!(v == 0 || v == 1);
        match name {
            "C" => self.cpu.flags.C = v,
            "Z" => self.cpu.flags.Z = v,
            "I" => self.cpu.flags.I = v,
            "D" => self.cpu.flags.D = v,
            "B" => self.cpu.flags.B = v,
            "V" => self.cpu.flags.V = v,
            "N" => self.cpu.flags.N = v,
            _ => panic!("unknown flag {name}"),
        }
    }

    pub fn test(&mut self, inst: &[u8], registers: &[u16], flags: &[u8]) {
        assert_eq!(registers.len(), self.registers.len());
        assert_eq!(flags.len(), self.flags.len());
        self.cpu.load_program(inst);
        self.cpu.PC = self.cpu.get_mem16(0xFFFC);
        self.cpu.run_once();
        for (name, r) in zip(self.registers.iter(), registers.iter()) {
            match name.as_ref() {
                "X" => assert_eq!(self.cpu.X, *r as u8),
                "Y" => assert_eq!(self.cpu.Y, *r as u8),
                "A" => assert_eq!(self.cpu.A, *r as u8),
                "SP" => assert_eq!(self.cpu.SP, *r as u8),
                "PC" => assert_eq!(self.cpu.PC, *r),
                _ => panic!("unknown register {name}"),
            }
        }
        for (name, f) in zip(self.flags.iter(), flags.iter()) {
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
