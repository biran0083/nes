
use std::iter::zip;
use crate::cpu::CPU;

pub struct TestRunner {
    cpu: CPU,
}

pub enum Register8 {
    X,
    Y,
    A,
}
pub enum Register16 {
    SP, PC
}


pub enum Flag {
    C,
    Z,
    I,
    D,
    B,
    V,
    N,
}

impl Retriever<bool> for Flag {
    fn get(&self, cpu: &CPU) -> bool {
        match self {
            Flag::C => cpu.flags.c(),
            Flag::Z => cpu.flags.z(),
            Flag::I => cpu.flags.i(),
            Flag::D => cpu.flags.d(),
            Flag::B => cpu.flags.b(),
            Flag::V => cpu.flags.v(),
            Flag::N => cpu.flags.n(),
        }
    }
}

trait Retriever<T> {
    fn get(&self, cpu: &CPU) -> T;
}

impl Retriever<u8> for Register8 {
    fn get(&self, cpu: &CPU) -> u8 {
        match self {
            Register8::X => cpu.x,
            Register8::Y => cpu.y,
            Register8::A => cpu.a,
        }
    }
}

impl Retriever<u16> for Register16 {
    fn get(&self, cpu: &CPU) -> u16 {
        match self {
            Register16::SP => cpu.sp,
            Register16::PC => cpu.pc,
        }
    }
}

pub struct TestResult<'a> {
    cpu: &'a CPU
}

impl<'a> TestResult<'a> {
    pub fn verify<T: PartialEq + std::fmt::Debug>(&self, retriever: impl Retriever<T>, value: T) -> &Self {
        assert_eq!(retriever.get(self.cpu), value);
        self
    }
}

impl TestRunner {
    pub fn new() -> Self {
        TestRunner {
            cpu: CPU::new(),
        }
    }

    pub fn set_mem(&mut self, addr: usize, value: u8) -> &mut Self {
        self.cpu.mem[addr] = value;
        self
    }

    pub fn set_register(&mut self, name: &str, value: u16) -> &mut Self {
        match name {
            "X" => self.cpu.x = value as u8,
            "Y" => self.cpu.y = value as u8,
            "A" => self.cpu.a = value as u8,
            "SP" => self.cpu.sp = value as u16,
            "PC" => self.cpu.pc = value,
            _ => panic!("unknown register {name}"),
        }
        self
    }

    pub fn set_flag(&mut self, name: &str, v: u8) -> &mut Self {
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
        self
    }

    pub fn test(&mut self, inst: &[u8]) -> TestResult {
        self.cpu.load_program(inst);
        self.cpu.pc = self.cpu.get_mem16(0xFFFC);
        self.cpu.run_once();
        TestResult {
            cpu: &self.cpu
        }
    }
}
