use crate::instructions::{Inst, INST_FACTORIES};

#[derive(Default)]
pub struct Flags {
    pub value: u8,
}

impl Flags {
    fn get_bit(&self, n: u8) -> bool {
        self.value & (1 << n) != 0
    }

    fn set_bit(&mut self, n: u8, v: bool) {
        if v {
            self.value |= 1 << n;
        } else {
            self.value &= !(1 << n);
        }
    }

    pub fn get(&self) -> u8 {
        self.value
    }

    pub fn set(&mut self, v: u8) {
        self.value = v
    }

    pub fn c(&self) -> bool {
        self.get_bit(0)
    }

    pub fn set_c(&mut self, c: bool) {
        self.set_bit(0, c)
    }

    pub fn z(&self) -> bool {
        self.get_bit(1)
    }

    pub fn set_z(&mut self, c: bool) {
        self.set_bit(1, c)
    }

    pub fn i(&self) -> bool {
        self.get_bit(2)
    }

    pub fn set_i(&mut self, i: bool) {
        self.set_bit(2, i)
    }

    pub fn d(&self) -> bool {
        self.get_bit(3)
    }

    pub fn set_d(&mut self, d: bool) {
        self.set_bit(3, d)
    }

    pub fn b(&self) -> bool {
        self.get_bit(4)
    }

    pub fn set_b(&mut self, b: bool) {
        self.set_bit(4, b)
    }

    pub fn v(&self) -> bool {
        self.get_bit(5)
    }

    pub fn set_v(&mut self, v: bool) {
        self.set_bit(5, v)
    }

    pub fn n(&self) -> bool {
        self.get_bit(6)
    }

    pub fn set_n(&mut self, n: bool) {
        self.set_bit(6, n)
    }
}

pub struct CPU {
    // registers
    pub x: u8,
    pub y: u8,
    pub a: u8,
    pub sp: u16,
    pub pc: u16,
    pub flags: Flags,
    pub mem: Vec<u8>,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            a: 0,
            sp: 0,
            pc: 0,
            flags: Flags::default(),
            mem: vec![0; 0x10000],
        }
    }

    pub fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
        self.a = 0;
        self.sp = 0x01ff;
        self.flags = Flags::default();
        self.pc = self.get_mem16(0xFFFC);
    }

    pub fn set_mem(&mut self, addr: usize, value: u8) {
        self.mem[addr] = value;
    }

    pub fn set_mem16(&mut self, addr: usize, value: u16) {
        let lsb = (value & 0xff) as u8;
        let msb = (value >> 8) as u8;
        self.set_mem(addr, lsb);
        self.set_mem(addr + 1, msb);
    }

    pub fn get_mem(&self, addr: usize) -> u8 {
        self.mem[addr]
    }

    pub fn get_mem16(&self, addr: usize) -> u16 {
        let lsb = self.get_mem(addr) as u16;
        let msb = self.get_mem(addr + 1) as u16;
        (msb << 8) + lsb
    }

    pub fn update_z(&mut self, value: u8) {
        self.flags.set_z(value == 0);
    }

    pub fn update_n(&mut self, value: u8) {
        self.flags.set_n(value & 0x80 != 0);
    }

    pub fn load_program(&mut self, bytes: &[u8]) {
        assert!(bytes.len() < 0x8000);
        let start: usize = 0x8000;
        self.set_mem16(0xFFFC, start as u16);
        self.mem[start..(start + bytes.len())].copy_from_slice(bytes);
        self.pc = start as u16;
        self.sp = 0x01ff;
        //self.reset()
    }

    fn decode(&mut self) -> Inst {
        let op = self.mem[self.pc as usize];
        INST_FACTORIES
            .get(&op)
            .unwrap()
            .make(&self.mem[((self.pc + 1) as usize)..])
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

    pub fn push8(&mut self, value: u8) {
        self.mem[self.sp as usize] = value;
        self.sp -= 1;
    }

    pub fn push16(&mut self, value: u16) {
        let lsb = (value & 0xff) as u8;
        let msb = (value >> 8) as u8;
        self.push8(msb);
        self.push8(lsb);
    }

    pub fn pop8(&mut self) -> u8 {
        self.sp += 1;
        self.mem[self.sp as usize]
    }

    pub fn pop16(&mut self) -> u16 {
        let lsb = self.pop8() as u16;
        let msb = self.pop8() as u16;
        (msb << 8) + lsb
    }
}
