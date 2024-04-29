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
            AddressingMode::Implied |
            AddressingMode::Accumulator => 1,
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
            AddressingMode::Accumulator => write!(f, "{} A", self.name),
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
    opcode_to_addressing_mode: &'static[(u8, AddressingMode)],
}

impl InstructionInfo {
    pub fn new(f: InstFactoryFun, opcode_to_addressing_mode: &'static[(u8, AddressingMode)]) -> Self {
        InstructionInfo {
            f,
            opcode_to_addressing_mode,
        }
    }
}

macro_rules! instruction_info {
    ($module:ident) => {
        InstructionInfo::new(
            crate::instructions::$module::make,
            crate::instructions::$module::OPCODE_MAP
        )
    };
}

lazy_static! {
pub static ref INST_FACTORIES: HashMap<u8, InstFactory> = {
    let instructions = &[
        instruction_info!(lda),
        instruction_info!(tax),
        instruction_info!(inx),
        instruction_info!(brk),
        instruction_info!(adc),
        instruction_info!(and),
        instruction_info!(asl),
    ];
    let mut inst_factory_by_op_code: HashMap<u8, InstFactory> = HashMap::new();
    for info in instructions.iter() {
        for (op, mode) in info.opcode_to_addressing_mode {
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
};
}

pub fn disassemble(bytes: &[u8]) -> Vec<Inst> {
    let mut res = vec![];
    let mut idx = 0;
    while idx < bytes.len() {
        let op = bytes[idx];
        if let Some(factory) = INST_FACTORIES.get(&op) {
            let inst = factory.make(&bytes[(idx + 1)..]);
            idx += inst.len() as usize;
            res.push(inst);
        } else {
            panic!("unknown op code: {:#x}", op);
        }
    }
    res
}
