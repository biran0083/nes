use crate::{
    addressing_mode::{load_operand, read_param, AddressingMode},
    cpu::CPU,
};

type InstFun = fn(&Inst, &mut CPU);
type InstFactoryFun = fn(AddressingMode, &mut std::slice::Iter<u8>) -> Inst;
pub struct Inst {
    name: &'static str,
    param: Option<u16>,
    mode: AddressingMode,
    f: InstFun,
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

pub fn make_lda(mode: AddressingMode, iter: &mut std::slice::Iter<u8>) -> Inst {
    Inst {
        name: "LDA",
        param: read_param(mode, iter),
        mode,
        f: move |ins, cpu| {
            cpu.A = load_operand(ins.mode, cpu, ins.param.unwrap());
        },
    }
}

pub fn make_tax(mode: AddressingMode, iter: &mut std::slice::Iter<u8>) -> Inst {
    assert!(matches!(mode, AddressingMode::Implied));
    Inst {
        name: "TAX",
        param: read_param(mode, iter),
        mode,
        f: move |_ins, cpu| cpu.X = cpu.A,
    }
}

pub fn make_inx(mode: AddressingMode, iter: &mut std::slice::Iter<u8>) -> Inst {
    assert!(matches!(mode, AddressingMode::Implied));
    Inst {
        name: "INX",
        param: read_param(mode, iter),
        mode,
        f: move |_ins, cpu| cpu.X = cpu.A,
    }
}

pub fn make_brk(mode: AddressingMode, iter: &mut std::slice::Iter<u8>) -> Inst {
    assert!(matches!(mode, AddressingMode::Implied));
    Inst {
        name: "BRK",
        param: read_param(mode, iter),
        mode,
        f: move |_ins, cpu| cpu.X = cpu.A,
    }
}

pub struct InstFactory {
    pub mode: AddressingMode,
    pub f: InstFactoryFun,
}

impl InstFactory {
    pub fn make(&self, iter: &mut std::slice::Iter<u8>) -> Inst {
        (self.f)(self.mode, iter)
    }
}
