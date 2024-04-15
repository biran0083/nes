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

    pub fn reset(&mut self) {
        self.X = 0;
        self.Y = 0;
        self.Y = 0;
        self.SP = 0;
        self.flags = Flags::default();
        self.PC = self.get_mem16(0xFFFC);
    }

    pub fn set_mem16(&mut self, addr: usize, value: u16) {
        let lsb = (value & 0xff) as u8;
        let msb = (value >> 8) as u8;
        self.mem[addr] = lsb;
        self.mem[addr + 1] = msb;
    }

    pub fn get_mem16(&self, addr: usize) -> u16 {
        let lsb = self.mem[addr] as u16;
        let msb = self.mem[addr + 1] as u16;
        (msb << 8) + lsb
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
        self.set_mem16(0xFFFC, start as u16);
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

    pub fn load_and_run(&mut self, bytes: &[u8]) {
        self.load_program(bytes);
        self.reset();
        loop {
            self.run_once();
        }
    }
}
