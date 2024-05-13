
use crate::cpu::CPU;

use super::common::{Retriever, Setter};

pub struct TestRunner {
    cpu: CPU,
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

    // push a byte to stack
    pub fn push(&mut self, value: u8) -> &mut Self {
        self.cpu.push8(value);
        self
    }

    pub fn load_and_test(&mut self, inst: &[u8]) -> TestResult {
        self.load_program(inst);
        self.test()
    }

    pub fn load_program(&mut self, bytes: &[u8]) -> &mut Self {
        self.cpu.load_program(bytes);
        self.cpu.pc = self.cpu.get_mem16(0xFFFC);
        self
    }

    pub fn test(&mut self) -> TestResult {
        self.cpu.run_once();
        TestResult {
            cpu: &self.cpu
        }
    }
}
