use std::collections::HashMap;

use crate::{
    addressing_mode::{load_operand, read_param, AddressingMode},
    cpu::CPU,
};

type InstFun = fn(&Inst, &mut CPU);
type InstFactoryFun = fn(AddressingMode, &[u8]) -> Inst;
pub struct Inst {
    name: &'static str,
    param: Option<u16>,
    mode: AddressingMode,
    f: InstFun,
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

pub fn make_lda(mode: AddressingMode, bytes: &[u8]) -> Inst {
    Inst {
        name: "LDA",
        param: read_param(mode, bytes),
        mode,
        f: move |ins, cpu: &mut CPU| {
            cpu.A = load_operand(ins.mode, cpu, ins.param.unwrap());
            cpu.update_z();
            cpu.update_n();
            cpu.PC += ins.len();
        },
    }
}

pub fn make_tax(mode: AddressingMode, bytes: &[u8]) -> Inst {
    assert!(matches!(mode, AddressingMode::Implied));
    Inst {
        name: "TAX",
        param: read_param(mode, bytes),
        mode,
        f: move |ins, cpu| {
            cpu.X = cpu.A;
            cpu.update_z();
            cpu.update_n();
            cpu.PC += ins.len();
        },
    }
}

pub fn make_inx(mode: AddressingMode, bytes: &[u8]) -> Inst {
    assert!(matches!(mode, AddressingMode::Implied));
    Inst {
        name: "INX",
        param: read_param(mode, bytes),
        mode,
        f: move |ins, cpu| {
            cpu.X = cpu.A;
            cpu.PC += ins.len();
        },
    }
}

pub fn make_brk(mode: AddressingMode, bytes: &[u8]) -> Inst {
    assert!(matches!(mode, AddressingMode::Implied));
    Inst {
        name: "BRK",
        param: read_param(mode, bytes),
        mode,
        f: move |ins, cpu| {
            cpu.X = cpu.A;
            cpu.PC += ins.len();
        },
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

pub fn make_inst_factories_by_op_code() -> HashMap<u8, InstFactory> {
    let mut inst_factory_by_op_code: HashMap<u8, InstFactory> = HashMap::new();
    for (opcode, mode) in [
        (0xA9, AddressingMode::Immediate),
        (0xA5, AddressingMode::ZeroPage),
        (0xB5, AddressingMode::ZeroPageX),
        (0xAD, AddressingMode::Absolute),
        (0xBD, AddressingMode::AbsoluteX),
        (0xB9, AddressingMode::AbsoluteY),
        (0xA1, AddressingMode::IndexedIndirect),
        (0xB1, AddressingMode::IndirectIndexed),
    ] {
        inst_factory_by_op_code.insert(opcode, InstFactory { mode, f: make_lda });
    }
    inst_factory_by_op_code.insert(
        0xaa,
        InstFactory {
            mode: AddressingMode::Implied,
            f: make_tax,
        },
    );
    inst_factory_by_op_code.insert(
        0xe8,
        InstFactory {
            mode: AddressingMode::Implied,
            f: make_inx,
        },
    );
    inst_factory_by_op_code.insert(
        0x00,
        InstFactory {
            mode: AddressingMode::Implied,
            f: make_brk,
        },
    );
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
