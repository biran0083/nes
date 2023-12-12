use std::collections::HashMap;

#[derive(Debug, Default)]
struct CPU {
    // registers
    X: u8,
    Y: u8,
    A: u8,
    SP: u8,
    PC: u16,
    status: u8,
    mem: Vec<u8>,
}

impl CPU {
    fn new() -> Self {
        Self {
            mem: vec![0; 0x10000],
            ..Default::default()
        }
    }
}
type InstFun = fn(&Inst, &mut CPU);
type InstFactoryFun = fn(AddressingMode, &mut std::slice::Iter<u8>) -> Inst;
struct Inst {
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
            AddressingMode::IndirectX => {
                write!(f, "{} (${:#02x}, X)", self.name, self.param.unwrap())
            }
            AddressingMode::IndirectY => {
                write!(f, "{} (${:#02x}), Y", self.name, self.param.unwrap())
            }
        }
    }
}

#[derive(Clone, Copy)]
enum AddressingMode {
    Implied,
    Immediate,
    ZeroPage,
    ZeroPageX,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
}

fn read_param(mode: AddressingMode, iter: &mut std::slice::Iter<u8>) -> Option<u16> {
    match mode {
        AddressingMode::Implied => None,
        AddressingMode::Immediate
        | AddressingMode::ZeroPage
        | AddressingMode::ZeroPageX
        | AddressingMode::IndirectX
        | AddressingMode::IndirectY => Some(*iter.next().unwrap() as u16),
        AddressingMode::Absolute | AddressingMode::AbsoluteX | AddressingMode::AbsoluteY => {
            let lsb: u16 = *iter.next().unwrap() as u16;
            let msb: u16 = *iter.next().unwrap() as u16;
            Some((msb << 8) + lsb)
        }
    }
}

fn load_operand(mode: AddressingMode, cpu: &CPU, param: u16) -> u8 {
    match mode {
        AddressingMode::Implied => {
            panic!("load_operand should not be called for Implied instruction")
        }
        AddressingMode::Immediate => {
            assert!(param <= 0xff);
            param as u8
        }
        AddressingMode::ZeroPage => {
            assert!(param <= 0xff);
            cpu.mem[param as usize]
        }
        AddressingMode::ZeroPageX => {
            assert!(param <= 0xff);
            cpu.mem[(cpu.X + (param as u8)) as usize]
        }
        AddressingMode::Absolute => cpu.mem[param as usize],
        AddressingMode::AbsoluteX => cpu.mem[(param + cpu.X as u16) as usize],
        AddressingMode::AbsoluteY => cpu.mem[(param + cpu.Y as u16) as usize],
        AddressingMode::IndirectX => {
            assert!(param <= 0xff);
            let addr = cpu.mem[(cpu.X as u16 + param) as usize] as usize;
            let lsb = cpu.mem[addr] as u16;
            let msb = cpu.mem[addr + 1] as u16;
            cpu.mem[((msb << 8) + lsb) as usize]
        }
        AddressingMode::IndirectY => {
            assert!(param <= 0xff);
            let addr = cpu.mem[(cpu.Y as u16 + param) as usize] as usize;
            let lsb = cpu.mem[addr] as u16;
            let msb = cpu.mem[addr + 1] as u16;
            cpu.mem[((msb << 8) + lsb) as usize]
        }
    }
}

fn make_lda(mode: AddressingMode, iter: &mut std::slice::Iter<u8>) -> Inst {
    Inst {
        name: "LDA",
        param: read_param(mode, iter),
        mode,
        f: move |ins, cpu| {
            cpu.A = load_operand(ins.mode, cpu, ins.param.unwrap());
        },
    }
}

fn make_tax(mode: AddressingMode, iter: &mut std::slice::Iter<u8>) -> Inst {
    assert!(matches!(mode, AddressingMode::Implied));
    Inst {
        name: "TAX",
        param: read_param(mode, iter),
        mode,
        f: move |_ins, cpu| cpu.X = cpu.A,
    }
}

fn make_inx(mode: AddressingMode, iter: &mut std::slice::Iter<u8>) -> Inst {
    assert!(matches!(mode, AddressingMode::Implied));
    Inst {
        name: "INX",
        param: read_param(mode, iter),
        mode,
        f: move |_ins, cpu| cpu.X = cpu.A,
    }
}

fn make_brk(mode: AddressingMode, iter: &mut std::slice::Iter<u8>) -> Inst {
    assert!(matches!(mode, AddressingMode::Implied));
    Inst {
        name: "BRK",
        param: read_param(mode, iter),
        mode,
        f: move |_ins, cpu| cpu.X = cpu.A,
    }
}

struct InstFunction {
    mode: AddressingMode,
    f: InstFactoryFun,
}

impl InstFunction {
    fn make(&self, iter: &mut std::slice::Iter<u8>) -> Inst {
        (self.f)(self.mode, iter)
    }
}

fn main() {
    let bytes: Vec<u8> = vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00];
    let mut iter = bytes.iter();
    let mut inst_factory_by_op_code: HashMap<u8, InstFunction> = HashMap::new();
    for (opcode, mode) in [
        (0xa9, AddressingMode::Immediate),
        (0x05, AddressingMode::ZeroPage),
        (0x15, AddressingMode::ZeroPageX),
        (0x0D, AddressingMode::Absolute),
        (0x1D, AddressingMode::AbsoluteX),
        (0x19, AddressingMode::AbsoluteY),
        (0x01, AddressingMode::IndirectX),
        (0x11, AddressingMode::IndirectY),
    ] {
        inst_factory_by_op_code.insert(opcode, InstFunction { mode, f: make_lda });
    }
    inst_factory_by_op_code.insert(
        0xaa,
        InstFunction {
            mode: AddressingMode::Implied,
            f: make_tax,
        },
    );
    inst_factory_by_op_code.insert(
        0xe8,
        InstFunction {
            mode: AddressingMode::Implied,
            f: make_inx,
        },
    );
    inst_factory_by_op_code.insert(
        0x00,
        InstFunction {
            mode: AddressingMode::Implied,
            f: make_brk,
        },
    );
    while let Some(b) = iter.next() {
        if let Some(factory) = inst_factory_by_op_code.get(b) {
            let inst = factory.make(&mut iter);
            println!("{inst:?}");
        } else {
            panic!("unknown op code: {:#x}", b);
        }
    }
}
