
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

pub struct Flags {}

impl Retriever<u8> for Flags {
    fn get(&self, cpu: &CPU) -> u8 {
        cpu.flags.get()
    }
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
pub trait Setter<T> {
    fn set(&self, cpu: &mut CPU, value: T);
}

impl Setter<u8> for Register8 {
    fn set(&self, cpu: &mut CPU, value: u8) {
        match self {
            Register8::X => cpu.x = value,
            Register8::Y => cpu.y = value,
            Register8::A => cpu.a = value,
        }
    }
}

impl Setter<u16> for Register16 {
    fn set(&self, cpu: &mut CPU, value: u16) {
        match self {
            Register16::SP => cpu.sp = value,
            Register16::PC => cpu.pc = value,
        }
    }
}

impl Setter<bool> for Flag {
    fn set(&self, cpu: &mut CPU, value: bool) {
        match self {
            Flag::C => cpu.flags.set_c(value),
            Flag::Z => cpu.flags.set_z(value),
            Flag::I => cpu.flags.set_i(value),
            Flag::D => cpu.flags.set_d(value),
            Flag::B => cpu.flags.set_b(value),
            Flag::V => cpu.flags.set_v(value),
            Flag::N => cpu.flags.set_n(value),
        }
    }
}

pub trait Retriever<T> {
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

    pub fn verify_stack(&self, offset: i16, value: u8) -> &Self {
        assert_eq!(self.cpu.get_mem(self.cpu.sp as usize + offset as usize), value);
        self
    }

    pub fn verify_stack16(&self, offset: i16, value: u16) -> &Self {
        assert_eq!(self.cpu.get_mem16(self.cpu.sp as usize + offset as usize), value);
        self
    }
}

impl TestRunner {
    pub fn new() -> Self {
        TestRunner {
            cpu: CPU::new(),
        }
    }

    pub fn get<T>(&self, retriever: impl Retriever<T>) -> T {
        retriever.get(&self.cpu)
    }

    pub fn set_mem(&mut self, addr: usize, value: u8) -> &mut Self {
        self.cpu.mem[addr] = value;
        self
    }

    pub fn set_mem16(&mut self, addr: usize, value: u16) -> &mut Self {
        self.cpu.set_mem16(addr, value);
        self
    }

    pub fn set<T>(&mut self, setter: impl Setter<T>, value: T) -> &mut Self {
        setter.set(&mut self.cpu, value);
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
