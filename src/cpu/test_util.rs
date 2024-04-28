
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
            "X" => self.cpu.x = value as u8,
            "Y" => self.cpu.y = value as u8,
            "A" => self.cpu.a = value as u8,
            "SP" => self.cpu.sp = value as u16,
            "PC" => self.cpu.pc = value,
            _ => panic!("unknown register {name}"),
        }
    }

    pub fn set_flag(&mut self, name: &str, v: u8) {
        let v = if v == 0 { false} else {true};
        match name {
            "C" => self.cpu.flags.set_c(v),
            "Z" => self.cpu.flags.set_z(v),
            "I" => self.cpu.flags.set_i(v),
            "D" => self.cpu.flags.set_d(v),
            "B" => self.cpu.flags.set_b(v),
            "V" => self.cpu.flags.set_v(v),
            "N" => self.cpu.flags.set_n(v),
            _ => panic!("unknown flag {name}"),
        }
    }

    pub fn test(&mut self, inst: &[u8], registers: &[u16], flags: &[u8]) {
        assert_eq!(registers.len(), self.registers.len());
        assert_eq!(flags.len(), self.flags.len());
        let flags : Vec<bool> = flags.iter().map(|v| *v != 0).collect();
        self.cpu.load_program(inst);
        self.cpu.pc = self.cpu.get_mem16(0xFFFC);
        self.cpu.run_once();
        for (name, r) in zip(self.registers.iter(), registers.iter()) {
            match name.as_ref() {
                "X" => assert_eq!(self.cpu.x, *r as u8),
                "Y" => assert_eq!(self.cpu.y, *r as u8),
                "A" => assert_eq!(self.cpu.a, *r as u8),
                "SP" => assert_eq!(self.cpu.sp, *r as u16),
                "PC" => assert_eq!(self.cpu.pc, *r),
                _ => panic!("unknown register {name}"),
            }
        }
        for (name, f) in zip(self.flags.iter(), flags.iter()) {
            match name.as_ref() {
                "C" => assert_eq!(self.cpu.flags.c(), *f),
                "Z" => assert_eq!(self.cpu.flags.z(), *f),
                "I" => assert_eq!(self.cpu.flags.i(), *f),
                "D" => assert_eq!(self.cpu.flags.d(), *f),
                "B" => assert_eq!(self.cpu.flags.b(), *f),
                "V" => assert_eq!(self.cpu.flags.v(), *f),
                "N" => assert_eq!(self.cpu.flags.n(), *f),
                _ => panic!("unknown flag {name}"),
            }
        }
    }
}
