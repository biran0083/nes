use std::collections::HashMap;
use lazy_static::lazy_static;

use crate::{
    cpu::addressing_mode::AddressingMode,
    cpu::CPU,
};

type InstFun = fn(&Inst, &mut CPU);
type InstFactoryFun = fn(AddressingMode, &[u8]) -> Inst;
pub struct Inst {
    pub name: &'static str,
    pub param: Option<u16>,
    pub mode: AddressingMode,
    pub f: InstFun,
}

impl Inst {

    pub fn run(&self, cpu: &mut CPU) {
        (self.f)(self, cpu)
    }

    pub fn len(&self) -> u16 {
        match self.mode {
            AddressingMode::Implied => 1,
            AddressingMode::Immediate
            | AddressingMode::ZeroPage
            | AddressingMode::ZeroPageX
            | AddressingMode::IndexedIndirect
            | AddressingMode::IndirectIndexed => 2,
            AddressingMode::Absolute | AddressingMode::AbsoluteX | AddressingMode::AbsoluteY => 3,
        }
    }
}

impl std::fmt::Debug for Inst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.mode {
            AddressingMode::Implied => write!(f, "{}", self.name),
            AddressingMode::Immediate => write!(f, "{} #${:02x}", self.name, self.param.unwrap()),
            AddressingMode::ZeroPage => write!(f, "{} ${:02x}, X", self.name, self.param.unwrap()),
            AddressingMode::ZeroPageX => {
                write!(f, "{} ${:#02x}, Y", self.name, self.param.unwrap())
            }
            AddressingMode::Absolute => write!(f, "{} ${:04x}", self.name, self.param.unwrap()),
            AddressingMode::AbsoluteX => {
                write!(f, "{} ${:#04x}, X", self.name, self.param.unwrap())
            }
            AddressingMode::AbsoluteY => {
                write!(f, "{} ${:#04x}, Y", self.name, self.param.unwrap())
            }
            AddressingMode::IndexedIndirect => {
                write!(f, "{} (${:#02x}, X)", self.name, self.param.unwrap())
            }
            AddressingMode::IndirectIndexed => {
                write!(f, "{} (${:#02x}), Y", self.name, self.param.unwrap())
            }
        }
    }
}

pub struct InstFactory {
    pub mode: AddressingMode,
    pub f: InstFactoryFun,
}

impl InstFactory {
    pub fn make(&self, iter: &[u8]) -> Inst {
        (self.f)(self.mode, iter)
    }
}

struct InstructionInfo {
    f: InstFactoryFun,
    opcode_to_addressing_mode: Vec<(u8, AddressingMode)>,
}

impl InstructionInfo {
    pub fn new(f: InstFactoryFun, opcode_to_addressing_mode: Vec<(u8, AddressingMode)>) -> Self {
        InstructionInfo {
            f,
            opcode_to_addressing_mode,
        }
    }
}

lazy_static! {
static ref INSTRUCTINOSS: Vec<InstructionInfo> = vec![
    InstructionInfo::new(
        crate::instructions::lda::make,
        vec![
            (0xA9, AddressingMode::Immediate),
            (0xA5, AddressingMode::ZeroPage),
            (0xB5, AddressingMode::ZeroPageX),
            (0xAD, AddressingMode::Absolute),
            (0xBD, AddressingMode::AbsoluteX),
            (0xB9, AddressingMode::AbsoluteY),
            (0xA1, AddressingMode::IndexedIndirect),
            (0xB1, AddressingMode::IndirectIndexed),
        ]),
    InstructionInfo::new(crate::instructions::tax::make, vec![(0xaa, AddressingMode::Implied)]),
    InstructionInfo::new(crate::instructions::idx::make, vec![(0xe8, AddressingMode::Implied)]),
    InstructionInfo::new(crate::instructions::brk::make, vec![(0x00, AddressingMode::Implied)]),
    InstructionInfo::new(crate::instructions::adc::make, vec![
        (0x69, AddressingMode::Immediate),
        (0x65, AddressingMode::ZeroPage),
        (0x75, AddressingMode::ZeroPageX),
        (0x6D, AddressingMode::Absolute),
        (0x7D, AddressingMode::AbsoluteX),
        (0x79, AddressingMode::AbsoluteY),
        (0x61, AddressingMode::IndexedIndirect),
        (0x71, AddressingMode::IndirectIndexed),
    ]),
];
}

pub fn make_inst_factories_by_op_code() -> HashMap<u8, InstFactory> {
    let mut inst_factory_by_op_code: HashMap<u8, InstFactory> = HashMap::new();
    for info in INSTRUCTINOSS.iter() {
        for (op, mode) in &info.opcode_to_addressing_mode {
            inst_factory_by_op_code.insert(
                *op,
                InstFactory {
                    mode: *mode,
                    f: info.f,
                },
            );
        }
    }
    inst_factory_by_op_code
}

pub fn disassemble(bytes: &[u8]) -> Vec<Inst> {
    let inst_factory_by_op_code = make_inst_factories_by_op_code();
    let mut res = vec![];
    let mut idx = 0;
    while idx < bytes.len() {
        let op = bytes[idx];
        if let Some(factory) = inst_factory_by_op_code.get(&op) {
            let inst = factory.make(&bytes[(idx + 1)..]);
            idx += inst.len() as usize;
            res.push(inst);
        } else {
            panic!("unknown op code: {:#x}", op);
        }
    }
    res
}
